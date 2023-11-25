use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RunOptions {
    pub memory_limit: String,
    pub cpus_limit: f64,
    pub pids_limit: usize,
    pub storage_limit: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExecutionStep {
    pub name: String,
    pub command: Vec<String>,
    pub timeout: u32,
    pub image: Option<String>,
    pub packages: Option<Vec<String>>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub run_options: Option<RunOptions>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RunSpec {
    pub name: String,
    pub steps: Vec<ExecutionStep>,
    pub packages: Option<Vec<String>>,
    pub run_options: Option<RunOptions>,
}
