mod docker;
mod spec;
use metrics::{counter, describe_counter, describe_gauge, Label};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use spec::{ExecutionStep, RunOptions, RunSpec};

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::net::SocketAddr;
use std::str;
use std::sync::atomic::AtomicUsize;

use tokio::time::sleep;

use rand::{distributions::Alphanumeric, Rng};

use anyhow::{anyhow, bail, Context, Result};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;

lazy_static! {
    static ref LUA_REQUEST_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref PROMETHEUS: PrometheusHandle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to create PrometheusBuilder");
    static ref CONFIG: HashMap<String, RunSpec> =
        initialize_configs().expect("Failed to initialize configs");
}

const EVAL_FOLDER: &str = "/evaluator/mounted";
const TIMEOUT_CODE: u8 = 224;

#[derive(Serialize, Deserialize, Debug)]
struct RequestPayload {
    language: String,
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponsePayload {
    stdout: String,
    stderr: String,
}

#[allow(dead_code)]
struct EvalResult {
    exit_code: u8,
    container_stdout: Option<String>,
    container_stderr: Option<String>,
}

fn random_filename() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}

fn get_run_options(runspec: &RunSpec, execution_step: &ExecutionStep) -> Result<RunOptions> {
    match &execution_step.run_options {
        Some(run_options) => Ok(run_options.clone()),
        None => runspec
            .run_options
            .clone()
            .ok_or(anyhow!("No run options provided!")),
    }
}

async fn do_one_step(
    step: &ExecutionStep,
    run_spec: &RunSpec,
    eval_id: &str,
) -> Result<EvalResult> {
    println!("Running '{}' step with eval_id ID {}", step.name, eval_id);
    // The path to the sources must be inside the DIND container,
    // not this one!
    println!("/www/app/sources/{}/{}", run_spec.name, eval_id);
    let image = format!("{}-{}", run_spec.name, "runtime");
    let run_options = get_run_options(run_spec, step)?;
    let start = std::time::Instant::now();
    let mut command = vec![
        "/evaluator/evaluate.sh".to_string(),
        step.stdout.clone().unwrap_or("/dev/null".to_string()),
        step.stderr.clone().unwrap_or("/dev/null".to_string()),
    ];
    command.append(&mut step.command.clone());

    let container_id = docker::docker_run(&docker::DockerRunSpec {
        image,
        volumes: vec![
            format!(
                "/www/app/sources/{}/{}:{}",
                run_spec.name, eval_id, EVAL_FOLDER
            ),
            format!("{}:{}", eval_id, "/home/evaluator_nobody"),
        ],
        env: vec![],
        ports: vec![],
        memory: run_options.memory_limit.clone(),
        cpus: run_options.cpus_limit,
        pids_limit: run_options.pids_limit,
        command,
        network: "none".to_string(),
    })?
    .container_id;

    println!("Running container with ID '{}'", container_id);

    // An asynchronous sleep, wait until the task finishes and hand out execution
    // to other tasks.
    let mut timeout = false;
    let mut state = docker::ImageState::load_from(container_id.as_str())?;
    while state.running {
        let delta = start.elapsed();
        if delta.as_secs() >= step.timeout as u64 {
            println!("Timeout on step {}", step.name);
            timeout = true;
            break;
        }

        sleep(tokio::time::Duration::from_millis(50)).await;

        state = docker::ImageState::load_from(container_id.as_str())
            .context("Could not get docker image state")?;
    }

    if timeout {
        docker::docker_kill(container_id.as_str()).context("Could not kill docker container")?;
    }

    let container_log =
        docker::docker_logs(container_id.as_str()).context("Could not fetch docker logs")?;
    docker::docker_rm(container_id.as_str()).context("Unable to remove container")?;

    println!("Image state: {:?}", state);

    if timeout {
        Ok(EvalResult {
            exit_code: TIMEOUT_CODE,
            container_stdout: None,
            container_stderr: None,
        })
    } else {
        Ok(EvalResult {
            exit_code: state.exit_code,
            container_stdout: Some(container_log.stdout),
            container_stderr: Some(container_log.stderr),
        })
    }
}

async fn run_spec(spec: &RunSpec, eval_id: &str) -> Result<ResponsePayload> {
    counter!(
        "evaluator_requests",
        1,
        vec![Label::new("language", spec.name.clone())]
    );
    docker::create_volume(eval_id).context("Unable to create volume")?;

    for step in &spec.steps {
        let res = do_one_step(step, spec, eval_id).await?;
        println!(
            "Step '{}' finished with exit code {}",
            step.name, res.exit_code
        );
        if res.exit_code != 0 {
            counter!(
                "evaluator_errors",
                1,
                vec![Label::new("language", spec.name.clone())]
            );
            println!(
                "Step '{}' failed with exit code {}, stderr: {}",
                step.name,
                res.exit_code,
                res.container_stderr.unwrap_or("No logs".to_string())
            );
            let stderr = if res.exit_code == TIMEOUT_CODE {
                "Timeout".to_string()
            } else {
                let stderr = fs::read_to_string(format!(
                    "/www/app/sources/{}/{}/{}",
                    spec.name, eval_id, "stderr.txt"
                ))
                .unwrap_or("".to_string());
                format!(
                    "Step '{}' failed with exit code {}\n{}",
                    step.name, res.exit_code, stderr
                )
            };
            return Ok(ResponsePayload {
                stdout: "".to_string(),
                stderr,
            });
        }
    }

    docker::remove_volume(eval_id).context("Removing volume failed")?;

    let stdout = fs::read_to_string(format!(
        "/www/app/sources/{}/{}/{}",
        spec.name, eval_id, "stdout.txt"
    ))
    .unwrap_or("".to_string());
    let stderr = fs::read_to_string(format!(
        "/www/app/sources/{}/{}/{}",
        spec.name, eval_id, "stderr.txt"
    ))
    .unwrap_or("".to_string());

    Ok(ResponsePayload { stdout, stderr })
}

async fn handle_eval_request(req: Request<Body>) -> anyhow::Result<Response<Body>> {
    let content_type = req
        .headers()
        .get("Content-Type")
        .ok_or(anyhow!("Content-Length is missing"))?
        .to_str()?
        .to_owned();
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let body = String::from_utf8(body_bytes.to_vec())?;

    let data: RequestPayload = match content_type.as_str() {
        "application/json" => serde_json::from_str(&body)?,
        _ => bail!("Unsupported content type"),
    };

    let runspec = CONFIG
        .get(&data.language)
        .ok_or_else(|| anyhow!("No such language {}.", data.language))?;

    let folder_name = random_filename();
    let path_str = format!("/www/app/sources/{}/{}/source", runspec.name, folder_name,);
    let path = std::path::Path::new(&path_str);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = File::create(path)?;
    file.write_all(data.code.as_bytes())?;

    let result = run_spec(runspec, &folder_name).await?;

    fs::remove_dir_all(prefix).context("Could not remove source folder")?;

    let response_payload = serde_json::to_string(&result)?;
    let response = Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::from(response_payload))?;

    Ok(response)
}

async fn handle_connection(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/liveness") => Ok(Response::new(Body::from("OK"))),
        (&Method::POST, "/api/v1/evaluate") => {
            let res = handle_eval_request(req).await;
            match res {
                Ok(r) => Ok(r),
                Err(e) => {
                    println!("Error: {:?}", e);
                    counter!("total_evaluator_errors", 1);
                    let mut response = Response::new(Body::from(e.to_string()));
                    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    Ok(response)
                }
            }
        }
        (&Method::GET, "/metrics") => {
            let mut response = Response::new(Body::from(PROMETHEUS.render()));
            *response.status_mut() = StatusCode::OK;
            Ok(response)
        }
        _ => {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}

fn initialize_configs() -> Result<HashMap<String, RunSpec>> {
    let config = "/www/app/images/config.yaml";
    let config = fs::read_to_string(config)?;
    let config: Vec<RunSpec> = serde_yaml::from_str(&config)?;
    Ok(config.into_iter().fold(HashMap::new(), |mut acc, spec| {
        acc.insert(spec.name.clone(), spec);
        acc
    }))
}

fn prepare_images() -> Result<()> {
    for (_, spec) in CONFIG.iter() {
        if spec.packages.is_some() {
            docker::docker_build(&docker::DockerBuildSpec {
                dockerfile: docker::Dockerfile::Stdin(format!(
                    r#"FROM base-runtime-alpine
                       RUN apk add --no-cache {}"#,
                    spec.packages.clone().unwrap().join(" ")
                )),
                context: ".".to_string(),
                tag: format!("{}-runtime", spec.name),
            })?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Preparing images...");
    prepare_images()?;
    println!("Images preparation done");

    describe_gauge!(
        "total_connections_active",
        "Total number of active connections"
    );
    describe_counter!(
        "total_evaluator_errors",
        "Total number of errors (panics) in the evaluator"
    );
    describe_counter!("evaluator_requests", "Number of requests to the evaluator");
    describe_counter!(
        "evaluator_errors",
        "Number of errors in the user submitted program"
    );

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(handle_connection)) });
    let addr = SocketAddr::from(([0, 0, 0, 0], 7800));
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}

#[cfg(test)]
mod test {}
