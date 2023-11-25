use std::str;
use std::{io::Write, process::Command};

use anyhow::{anyhow, Result};

use serde::Deserialize;

pub fn create_volume(name: &str) -> Result<()> {
    let output = Command::new("docker")
        .arg("volume")
        .arg("create")
        .arg(name)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to create the volume {:?}, stderr: {}",
            name,
            str::from_utf8(&output.stderr)?
        ));
    }

    Ok(())
}

pub fn remove_volume(name: &str) -> Result<()> {
    Command::new("docker")
        .arg("volume")
        .arg("rm")
        .arg(name)
        .output()?;
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct ImageState {
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    pub dead: bool,
    pub pid: u32,
    pub exit_code: u8,
    pub error: String,
    pub started_at: String,
    pub finished_at: String,
}

impl ImageState {
    pub fn load_from(container_id: &str) -> Result<ImageState> {
        let inspect = Command::new("docker")
            .arg("inspect")
            .arg("-f")
            .arg(r#"{{json .State}}"#)
            .arg(container_id)
            .output()?;

        let stderr = str::from_utf8(&inspect.stderr)?.to_string();

        if !inspect.status.success() {
            return Err(anyhow!(
                "Failed to inspect the container {:?}, stderr: {}",
                container_id,
                stderr.trim()
            ));
        }

        let inspect = str::from_utf8(&inspect.stdout)?.to_string();

        let state: ImageState = serde_json::from_str(&inspect)?;
        Ok(state)
    }
}

#[allow(dead_code)]
pub fn docker_ps() -> Result<String> {
    let ps = Command::new("docker").arg("ps").arg("-a").output()?;

    if !ps.status.success() {
        return Err(anyhow!("Failed to get the list of containers"));
    }

    let ps = str::from_utf8(&ps.stdout)?.to_string();

    Ok(ps)
}

pub struct Output {
    pub stdout: String,
    pub stderr: String,
}

pub fn docker_logs(container_id: &str) -> Result<Output> {
    let logs = Command::new("docker")
        .arg("logs")
        .arg(container_id)
        .output()?;

    if !logs.status.success() {
        return Err(anyhow!(
            "Failed to get logs from the container {:?}",
            container_id
        ));
    }

    let stdout = str::from_utf8(&logs.stdout)?.to_string();

    let stderr = str::from_utf8(&logs.stderr)?.to_string();

    Ok(Output { stdout, stderr })
}

pub fn docker_kill(container_id: &str) -> Result<()> {
    let kill = Command::new("docker")
        .arg("kill")
        .arg(container_id)
        .output()?;

    if !kill.status.success() {
        return Err(anyhow!("Failed to kill the container {:?}", container_id));
    }

    Ok(())
}

pub fn docker_rm(container_id: &str) -> Result<()> {
    let rm = Command::new("docker")
        .arg("rm")
        .arg("--force")
        .arg("--volumes")
        .arg(container_id)
        .output()?;

    if !rm.status.success() {
        return Err(anyhow!("Failed to remove the container {:?}", container_id));
    }

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Dockerfile {
    File(String),
    Stdin(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DockerBuildSpec {
    pub dockerfile: Dockerfile,
    pub context: String,
    pub tag: String,
}

pub fn docker_build(spec: &DockerBuildSpec) -> Result<()> {
    let mut command = Command::new("docker");
    command
        .arg("build")
        .arg(".")
        .arg("-t")
        .arg(spec.tag.clone())
        .arg("-f");

    let mut process = match &spec.dockerfile {
        Dockerfile::File(ref path) => command.arg(path).spawn()?,
        Dockerfile::Stdin(dockerfile_str) => {
            let process = command
                .arg("-")
                .stdin(std::process::Stdio::piped())
                .spawn()?;
            process
                .stdin
                .as_ref()
                .unwrap()
                .write_all(dockerfile_str.as_bytes())?;
            process
        }
    };

    if !process.wait()?.success() {
        return Err(anyhow!("Failed to build the container {:?}", spec));
    }

    Ok(())
}

#[derive(Debug)]
pub struct DockerRunSpec {
    pub image: String,
    pub volumes: Vec<String>,
    pub env: Vec<String>,
    pub ports: Vec<String>,
    pub memory: String,
    pub cpus: f64,
    pub pids_limit: usize,
    pub command: Vec<String>,
    pub network: String,
}

#[derive(Debug)]
pub struct DockerRunRes {
    pub container_id: String,
    pub stderr: String,
}

pub fn docker_run(spec: &DockerRunSpec) -> Result<DockerRunRes> {
    let mut command = Command::new("docker");
    command
        .arg("run")
        .arg("--memory")
        .arg(spec.memory.clone())
        .arg("--cpus")
        .arg(format!("{}", spec.cpus))
        .arg("--pids-limit")
        .arg(format!("{}", spec.pids_limit))
        .arg("--network")
        .arg(spec.network.clone())
        .arg("--detach");

    for volume in &spec.volumes {
        command.arg("-v").arg(volume.clone());
    }

    for env in &spec.env {
        command.arg("-e").arg(env.clone());
    }

    for port in &spec.ports {
        command.arg("-p").arg(port.clone());
    }

    command.arg(spec.image.clone());

    for cm in &spec.command {
        command.arg(cm);
    }

    let run = command.output()?;

    let container_id = str::from_utf8(&run.stdout)?.to_string().trim().to_string();

    let stderr = str::from_utf8(&run.stderr)?.to_string();

    if !run.status.success() {
        return Err(anyhow!(
            "Failed to run the container {:?}, stderr: {}",
            spec,
            stderr
        ));
    }

    Ok(DockerRunRes {
        container_id,
        stderr,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_docker_status() {
        let status = r#"{"Status":"exited","Running":false,"Paused":false,"Restarting":false,"OOMKilled":false,"Dead":false,"Pid":0,"ExitCode":0,"Error":"","StartedAt":"2023-11-18T08:48:21.142447846Z","FinishedAt":"2023-11-18T08:48:51.150078186Z"}"#;
        let state: ImageState = serde_json::from_str(status).unwrap();
        assert_eq!(state.status, "exited");
        assert_eq!(state.running, false);
        assert_eq!(state.paused, false);
        assert_eq!(state.restarting, false);
        assert_eq!(state.oom_killed, false);
        assert_eq!(state.dead, false);
        assert_eq!(state.pid, 0);
        assert_eq!(state.exit_code, 0);
        assert_eq!(state.error, "");
        assert_eq!(state.started_at, "2023-11-18T08:48:21.142447846Z");
        assert_eq!(state.finished_at, "2023-11-18T08:48:51.150078186Z");
    }
}
