pub mod http;

use std::collections::HashMap;
use std::{env, fs};
use std::io::{Result, Error, ErrorKind, prelude::*};
use std::fs::File;
use std::process::Command;
use std::str;

use rand::{distributions::Alphanumeric, Rng};

use serde::{Deserialize, Serialize};
use serde_json;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

const TIMEOUT: &str = "5";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Language {
    Python,
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
    exit_code: i32,
}

fn create_error(err: &str) -> Error {
    Error::new(ErrorKind::Other, err)
}

fn random_filename() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

fn run_python(filename: &str) -> Result<ResponsePayload> {
    println!("Running python3 command on file {}", filename);
    let output = Command::new("timeout")
        .arg(TIMEOUT)
        .arg("python3")
        .arg(filename)
        .output()?;

    let stderr = if output.status.code().is_none() {
        "The program timeouted".to_string()
    } else {
        str::from_utf8(&output.stderr).unwrap().to_string()
    };

    Ok(ResponsePayload {
        stdout: str::from_utf8(&output.stdout).unwrap().to_string(),
        stderr,
        exit_code: output.status.code().unwrap_or(124),
    })
}

fn get_extension(lang: &Language) -> &str {
    match lang {
        Language::Python => ".py",
    }
}

async fn handle_connection(sock: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(sock);
    let request = http::parse_http_request(&mut reader).await?;

    let content_type = request.content_type().ok_or(create_error("Content-Type must be present"))?;
    if content_type != "application/json" {
        return Err(create_error("Content type must be application/json"));
    }

    let json = request.content.unwrap();
    let instructions: RequestPayload = serde_json::from_str(&json)?;

    let filename = random_filename() + get_extension(&instructions.language);
    let mut file = File::create(&filename)?;
    file.write_all(&instructions.code.as_bytes())?;

    let result = match instructions.language {
        Language::Python => run_python(&filename),
    }?;

    fs::remove_file(filename)?;

    let response_payload = serde_json::to_string(&result)?;
    
    let response = http::HttpResponse {
        code: "200".to_string(),
        reason_phrase: "OK".to_string(),
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Content-Length".to_string(), format!("{}", response_payload.len())),
        ],
        body: Some(response_payload),
    };

    let response_str = http::create_http_response(&response);
    reader.write_all(&response_str.as_bytes()).await
}

async fn run_and_log(sock: TcpStream) {
    println!("Handling connection");
    match handle_connection(sock).await {
        Ok(_) => (),
        Err(e) => {
            println!("Error while handling connection: {:?}", e);
        },
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running runtime server...");
    let addr  = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:7800".to_string());

    let listener = TcpListener::bind(&addr).await?;
    
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(run_and_log(socket));
    }
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
        let res = parse_json(r#"{ "language": "Python", "code": "print()"}"#);
        assert!(res.is_ok());
        let req = res.unwrap();
        assert_eq!(req.language, Language::Python);
        assert_eq!(req.code, "print()");
    }
}
