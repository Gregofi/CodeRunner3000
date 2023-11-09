use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::process::Command;
use std::str;
use std::sync::atomic::AtomicUsize;

use rand::{distributions::Alphanumeric, Rng};

use anyhow::{anyhow, bail, Result};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use serde::{Deserialize, Serialize};

use metrics::{counter, describe_counter, describe_histogram, histogram, Unit};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use lazy_static::lazy_static;

lazy_static! {
    static ref LUA_REQUEST_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ref PROMETHEUS: PrometheusHandle = PrometheusBuilder::new()
        .install_recorder()
        .expect("Failed to create PrometheusBuilder");
}

// const TIMEOUT: &str = "5";
const EVAL_FOLDER: &str = "eval_env";
const MEMORY_LIMIT_MB: usize = 128;
const CPUS_LIMIT: f64 = 0.25;
const PIDS_LIMIT: usize = 32;
const MAX_STRING_OUTPUT_LENGTH: usize = 10000;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Language {
    Lua,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Lua => f.write_str("lua"),
        }
    }
}

impl Language {
    fn extension(&self) -> &str {
        match self {
            Language::Lua => "lua",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestPayload {
    language: Language,
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponsePayload {
    stdout: String,
    stderr: String,
}

fn random_filename() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

fn run_lua(folder: &str) -> Result<ResponsePayload> {
    println!("Running lua5.1 command with folder ID {}", folder);
    // The path to the sources must be inside the DIND container,
    // not this one!
    println!("/www/app/sources/lua/{}", folder);
    let start = std::time::Instant::now();
    let exec = Command::new("docker")
        .arg("run")
        .arg("--network")
        .arg("none")
        .arg("-v")
        .arg(&format!("/www/app/sources/lua/{}:/{}", folder, EVAL_FOLDER))
        .arg(&format!("--memory={}m", MEMORY_LIMIT_MB))
        .arg(&format!("--cpus={}", CPUS_LIMIT))
        .arg(&format!("--pids-limit={}", PIDS_LIMIT))
        .arg("lua-runtime")
        .arg("/usr/bin/evaluate.sh")
        .output()?;
    let delta = start.elapsed();
    counter!("lua_requests", 1);
    histogram!("lua_request_duration_seconds", delta.as_secs_f64());

    let stdout = str::from_utf8(&exec.stdout)
        .expect("Couldn't decode stdout")
        .to_string();
    let stderr = str::from_utf8(&exec.stderr)
        .expect("Couldn't decode stderr")
        .to_string();

    println!(
        "Lua finished, container stdout: '{}', stderr: '{}'",
        stdout, stderr
    );

    let program_stdout = fs::read_to_string(format!("/www/app/sources/lua/{}/stdout.txt", folder))
        .expect("Unable to read stdout");
    let program_stderr = fs::read_to_string(format!("/www/app/sources/lua/{}/stderr.txt", folder))
        .expect("Unable to read stdout");

    Ok(ResponsePayload {
        stdout: program_stdout[..std::cmp::min(MAX_STRING_OUTPUT_LENGTH, program_stdout.len())]
            .to_string(),
        stderr: program_stderr[..std::cmp::min(MAX_STRING_OUTPUT_LENGTH, program_stderr.len())]
            .to_string(),
    })
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

    let folder_name = random_filename();
    let path_str = format!(
        "/www/app/sources/{}/{}/source.{}",
        data.language,
        folder_name,
        data.language.extension()
    );
    let path = std::path::Path::new(&path_str);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = File::create(path)?;
    file.write_all(data.code.as_bytes())?;

    let result = match data.language {
        Language::Lua => run_lua(&folder_name),
    }?;

    // Keep the folders for now so that we can properly inspect errors
    // fs::remove_dir_all(folder_name)?;

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

#[tokio::main]
async fn main() -> Result<()> {
    describe_counter!("lua_requests", "Number of requests to the Lua evaluator");
    describe_histogram!(
        "lua_request_duration_seconds",
        Unit::Seconds,
        "Duration of Lua requests"
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
mod test {
    use super::*;
    fn parse_json(s: &str) -> Result<RequestPayload> {
        let res: RequestPayload = serde_json::from_str(s)?;
        Ok(res)
    }

    #[test]
    fn test_serialization() {
        assert!(parse_json("").is_err());
        assert!(parse_json("{}").is_err());
        let res = parse_json(r#"{ "language": "Lua", "code": "print()"}"#);
        assert!(res.is_ok());
        let req = res.unwrap();
        assert_eq!(req.language, Language::Lua);
        assert_eq!(req.code, "print()");
    }
}
