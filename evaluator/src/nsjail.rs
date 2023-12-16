use std::process::Command;

pub struct NsJailConfig {
    options: Vec<String>,
}

#[allow(dead_code, unused_variables, unused_mut)]
impl NsJailConfig {
    pub fn new() -> Self {
        NsJailConfig {
            options: Vec::new(),
        }
    }

    pub fn memory_limit(mut self, limit_mb: u64) -> Self {
        todo!()
    }

    pub fn time_limit(mut self, limit_s: u64) -> Self {
        todo!()
    }

    pub fn cpu_limit(mut self, limit_s: u64) -> Self {
        todo!()
    }

    pub fn output_limit(mut self, limit_mb: u64) -> Self {
        todo!()
    }

    pub fn fork_limit(mut self, limit: u64) -> Self {
        todo!()
    }

    pub fn readonly_bind(mut self, source: &str, dest: &str) -> Self {
        self.options.push("-R".to_string());
        self.options.push(format!("{}:{}", source, dest));
        self
    }

    pub fn config(mut self, config_path: &str) -> Self {
        self.options.push("--config".to_string());
        self.options.push(config_path.to_string());
        self
    }

    pub fn run(self, executable: Vec<String>) -> Command {
        let mut output = Command::new("nsjail");
        for option in self.options {
            output.arg(option);
        }
        output.arg("--");
        for arg in executable {
            output.arg(arg);
        }
        output
    }
}
