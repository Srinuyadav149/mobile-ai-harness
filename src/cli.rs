use clap;
use std::path::PathBuf;

/// AI Harness for Mobile
#[derive(clap::Parser, Debug)]
#[command(name = "mobile-ai-harness", version = "1.0")]
pub struct MobileAiHarnessCli {
    /// Workspace root directory
    #[arg(short = 'w', long = "workspace-root")]
    pub workspace_root: PathBuf,
    /// Task for the ai harness
    #[arg(short = 't', long = "task")]
    pub task: String,
    #[arg(short = 'n', long = "no-of-retries", default_value_t = 3)]
    pub no_of_retries: u8,
    #[arg(short = 'c', long = "connection-timeout-secs", default_value_t = 2)]
    pub connection_timeout: u8,
    #[arg(short = 'r', long = "response-timeout-secs", default_value_t = 90)]
    pub response_timeout: u8,
}

impl MobileAiHarnessCli {
    pub fn validate(self) -> Result<(), String> {
        if !self.workspace_root.exists() {
            return Err(format!(
                "ERROR: Workspace does not exists: {:?}",
                self.workspace_root
            ));
        }
        if !self.workspace_root.is_dir() {
            return Err(format!(
                "ERROR: Workspace is not a directory: {:?}",
                self.workspace_root
            ));
        }
        if self.task.trim().is_empty() {
            return Err(format!("ERROR: Task is empty: {:?}", self.task));
        }
        if self.no_of_retries == 0 {
            return Err(format!(
                "ERROR: No of retries cannot be zero: {:?}",
                self.no_of_retries
            ));
        }
        if self.connection_timeout == 0 {
            return Err(format!(
                "ERROR: Connection timeout cannot be zero: {:?}",
                self.connection_timeout
            ));
        }
        if self.response_timeout == 0 {
            return Err(format!(
                "ERROR: Respone timeout cannot be zero: {:?}",
                self.response_timeout
            ));
        }
        Ok(())
    }
}
