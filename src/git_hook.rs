use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandConfig {
    pub cmd: String,
    pub args: Option<String>,
    pub directory: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHook {
    pub name: String,
    command: CommandConfig,
    glob_pattern: Vec<String>,
    description: Option<String>,
}

impl std::fmt::Display for GitHook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " - {}: \n  {{", self.name)?;
        write!(f, "\n    path: {:?}", self.command)?;
        write!(f, "\n    glob_pattern: {:?}", self.glob_pattern)?;
        match &self.description {
            Some(text) => write!(f, "\n    description: {}", text),
            None => Ok(()),
        }?;
        write!(f, "\n  }}")
    }
}

impl GitHook {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running {}", self.command.cmd);
        let mut cmd = Command::new(&self.command.cmd);
        if let Some(v) = &self.command.args {
            cmd.arg(v);
        };
        if let Some(v) = &self.command.directory {
            cmd.current_dir(v);
        };
        cmd.spawn()
            .unwrap_or_else(|_| panic!("Failed to execute {:?}", self.command.cmd))
            .wait()?;
        Ok(())
    }
}
