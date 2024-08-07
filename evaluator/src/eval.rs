use std::{collections::HashMap, fs::File, io::Write};

use anyhow::{anyhow, bail, Context, Result};
use axum::{extract, response};
use log::debug;
use metrics::{counter, Label};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::AppError;
use crate::{nsjail, spec::RunSpec};

use lazy_static::lazy_static;

lazy_static! {
    static ref WORKDIR: String =
        std::env::var("EVALUATOR_WORKDIR").unwrap_or_else(|_| "/opt/evaluator/sources".to_string());
    static ref CONFIG: HashMap<String, RunSpec> =
        initialize_configs().expect("Failed to initialize configs");
    static ref CONFIG_PATH: String = std::env::var("EVALUATOR_CONFIG_PATH")
        .unwrap_or_else(|_| "/opt/evaluator/config".to_string());
    static ref COMPILERS_PATH: String = std::env::var("EVALUATOR_COMPILERS_PATH")
        .unwrap_or_else(|_| "/opt/evaluator/compilers".to_string());
}

const EXECUTOR_REPLACE: &str = "${EXECUTOR}";
const EXECUTOR_ARGS_REPLACE: &str = "${EXECUTOR_ARGS}";
const SOURCE_FILE_REPLACE: &str = "${SOURCE_FILE}";
const SOURCE_ARGS_REPLACE: &str = "${SOURCE_ARGS}";
const COMPILER_REPLACE: &str = "${COMPILER}";
const COMPILER_ARGS_REPLACE: &str = "${COMPILER_ARGS}";
const SOURCE_FILE_NAME: &str = "source";
const ID_LENGTH: usize = 32;
const MAX_OUTPUT_LENGTH: usize = 1024 * 1024; // 1MB

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPayload {
    language: String,

    compiler: Option<String>,
    #[serde(default)]
    compiler_args: Vec<String>,

    executor: Option<String>,
    #[serde(default)]
    executor_args: Vec<String>,

    #[serde(default)]
    program_args: Vec<String>,

    stdin: Option<String>,

    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePayload {
    stdout: String,
    stderr: String,
}

#[allow(dead_code)]
struct EvalResult {
    exit_code: i32,
    stdout: Option<String>,
    stderr: Option<String>,
}

struct ExecuteResult {
    #[allow(dead_code)]
    exit_code: i32,
    stdout: String,
    stderr: String,
}

fn initialize_configs() -> Result<HashMap<String, RunSpec>> {
    let config = format!("{}/config.yaml", *CONFIG_PATH);
    let config = fs::read_to_string(config)?;
    let config: Vec<RunSpec> = serde_yaml::from_str(&config)?;
    Ok(config.into_iter().fold(HashMap::new(), |mut acc, spec| {
        acc.insert(spec.name.clone(), spec);
        acc
    }))
}

/// Generates random id
fn random_id(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Escapes args into 'arg1', also removes args
/// which are whitespace only.
fn escape_args(args: &[String]) -> Vec<String> {
    args.iter()
        .filter(|arg| !arg.trim().is_empty())
        .map(|arg| format!("'{}'", arg))
        .collect::<Vec<String>>()
}

fn replace_variable(command: &str, variable: &str, value: &str) -> Result<String> {
    if str::find(command, variable).is_none() {
        bail!("Variable {} is not present in the command", variable);
    }

    Ok(str::replace(command, variable, value))
}

fn truncate_output(output: &str) -> String {
    if output.len() > MAX_OUTPUT_LENGTH {
        format!(
            "{} ... (truncated {} bytes)",
            &output[..MAX_OUTPUT_LENGTH],
            output.len() - MAX_OUTPUT_LENGTH
        )
    } else {
        output.to_string()
    }
}

async fn execute(spec: &RunSpec, payload: &RequestPayload, eval_id: &str) -> Result<ExecuteResult> {
    // TODO: memory, cpu etc. limits are set by the config,
    // but allow the user to override them.

    let compiler_args = escape_args(&payload.compiler_args);
    let executor_args = escape_args(&payload.executor_args);
    let program_args = escape_args(&payload.program_args);

    let source_folder = format!("{}/{}/{}", *WORKDIR, spec.name, eval_id);
    let source_file = format!("{}/{}", source_folder, SOURCE_FILE_NAME);

    let command = spec.commands.join(" && ");

    if str::find(&command, EXECUTOR_REPLACE).is_some() && payload.executor.is_none() {
        bail!("Executor not provided in payload.");
    }

    if str::find(&command, COMPILER_REPLACE).is_some() && payload.compiler.is_none() {
        bail!("Compiler not provided in payload.");
    }

    // Substitute variables in the command

    let mut command = command
        .replace(EXECUTOR_ARGS_REPLACE, &executor_args.join(" "))
        .replace(SOURCE_FILE_REPLACE, source_file.as_str())
        .replace(SOURCE_ARGS_REPLACE, &program_args.join(" "))
        .replace(COMPILER_ARGS_REPLACE, &compiler_args.join(" "));

    if payload.executor.is_some() {
        let executor = payload.executor.as_ref().unwrap();
        let executor_spec = &spec
            .executors
            .iter()
            .find(|e| e.name == *executor)
            .with_context(|| format!("Executor '{}' not found", executor))?;
        let executor_path = match &executor_spec.path {
            Some(path) => path.to_owned(),
            None => format!("{}/{}/{}", *COMPILERS_PATH, spec.name, executor_spec.name),
        };
        command = replace_variable(&command, EXECUTOR_REPLACE, &executor_path)?;
    }

    if payload.compiler.is_some() {
        let compiler = payload.compiler.as_ref().unwrap();
        let compiler_spec = &spec
            .compilers
            .iter()
            .find(|c| c.name == *compiler)
            .with_context(|| format!("Compiler '{}' not found", compiler))?;
        let compiler_path = match &compiler_spec.path {
            Some(path) => path.to_owned(),
            None => format!("{}/{}/{}", *COMPILERS_PATH, spec.name, compiler_spec.name),
        };
        debug!("Using compiler path {}", compiler_path);
        command = replace_variable(&command, COMPILER_REPLACE, &compiler_path)?;
    }

    // Compose the command together and run it

    let bash_wrapper = vec!["/bin/bash".to_string(), "-c".to_string(), command];
    debug!("Running command {:?} in jail", bash_wrapper);

    let nsjail = nsjail::NsJailConfig::new()
        .config(format!("{}/userspace.cfg", *CONFIG_PATH).as_str())
        .readonly_bind(source_folder.as_str(), source_folder.as_str());
    let output = tokio::task::spawn_blocking(move || {
        let mut cmd = nsjail.run(bash_wrapper);
        cmd.output()
    })
    .await??;

    let stdout = truncate_output(&String::from_utf8(output.stdout)?);
    let mut stderr = truncate_output(&String::from_utf8(output.stderr)?);
    let exit_code = output.status.code().unwrap();

    let timeout_message = if exit_code == 137 { " (timed out)" } else { "" };

    if exit_code != 0 {
        stderr = format!(
            "{} {}{}\n{}",
            "error: program exited with non-zero exit code", exit_code, timeout_message, stderr
        );
    }

    debug!(
        "Command exited with code {}, stdout: '{}', stderr: '{}'",
        exit_code, stdout, stderr
    );

    Ok(ExecuteResult {
        exit_code,
        stdout,
        stderr,
    })
}

async fn run_spec(
    spec: &RunSpec,
    payload: &RequestPayload,
    eval_id: &str,
) -> Result<ResponsePayload> {
    let execute_result = execute(spec, payload, eval_id).await?;

    Ok(ResponsePayload {
        stdout: execute_result.stdout,
        stderr: execute_result.stderr,
    })
}

pub async fn eval_handler(
    extract::Json(payload): extract::Json<RequestPayload>,
) -> Result<response::Json<ResponsePayload>, AppError> {
    let runspec = CONFIG
        .get(&payload.language)
        .ok_or_else(|| anyhow!("No such language {}.", payload.language))?;

    let execution_id = random_id(ID_LENGTH);
    let path_str = format!("{}/{}/{}/source", *WORKDIR, runspec.name, execution_id,);
    let path = std::path::Path::new(&path_str);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = File::create(path).context("Couldn't create file")?;
    file.write_all(payload.code.as_bytes())
        .context("Couldn't write bytes to file")?;

    counter!(
        "evaluator_requests_by_language",
        1,
        vec![Label::new("language", payload.language.clone())]
    );
    let result = run_spec(runspec, &payload, &execution_id).await;
    if result.is_err() {
        counter!(
            "evaluator_errors_by_language",
            1,
            vec![Label::new("language", payload.language.clone())]
        );
    }
    let result = result?;

    fs::remove_dir_all(prefix).context("Could not remove source folder")?;

    Ok(axum::Json(result))
}

pub async fn initialize_evaluator() -> Result<()> {
    fs::create_dir_all(&*WORKDIR).expect("Failed to create workdir");
    Ok(())
}
