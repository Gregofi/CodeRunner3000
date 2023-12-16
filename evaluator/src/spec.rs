use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Executor {
    pub name: String,
    // If not specified, the executor is assumed to be located
    // in /opt/evaluator/compilers/<language>/<executor>
    // if specified, then /opt/evaluator/compilers/<path>
    pub path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Compiler {
    pub name: String,
    // If not specified, the compiler is assumed to be located
    // in /opt/evaluator/compilers/<language>/<compiler>
    // if specified, then /opt/evaluator/compilers/<path>
    pub path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RunSpec {
    pub name: String,
    // This should be a hashmap name -> Compiler,
    // but don't know how to do it in serde.
    #[serde(default)]
    pub compilers: Vec<Compiler>,
    #[serde(default)]
    pub executors: Vec<Executor>,
    pub commands: Vec<String>,
}
