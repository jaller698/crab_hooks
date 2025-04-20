use serde::{Deserialize, Serialize};
use std::{
    fs::{self, set_permissions},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
};

use crate::hook_types::HookTypes;

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
        let status = cmd
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to execute {:?}", self.command.cmd))
            .wait()?;
        if status.success() {
            // exit code was zero
            Ok(())
        } else {
            // non‐zero or signal‐terminated
            match status.code() {
                // exited with some code != 0
                Some(code) => Err(format!("Command failed with status {}", code).into()),
                // e.g. killed by signal on Unix
                None => Err("Cmd terminated by signal".into()),
            }
        }
    }

    pub fn apply_hook(&self, hook_type: &HookTypes) -> Result<(), Box<dyn std::error::Error>> {
        println!("Apply hook {} as {}", self.name, hook_type);

        // First check if the current directory is a git repo
        fs::read_dir("./.git/")?;
        // Then check the current hook types is not already made (for now just both un-managed and
        // managed)
        // TODO: This file path should instead use the git config core.hooksPath
        let file_path = format!("./.git/hooks/{}", hook_type);
        if fs::read(&file_path).is_ok() {
            return Err("Failed to apply hook, the selected hook type already exists".into());
        }

        let mut hook_file = fs::File::create(&file_path)?;
        let exe_location = std::env::current_exe()?;
        let file_content = format!("{} run {}", exe_location.to_str().expect(""), self.name);
        writeln!(hook_file, "#!/usr/bin/env sh")?;
        writeln!(hook_file, "{}", file_content)?;
        drop(hook_file);

        let mut permissions = fs::metadata(&file_path)?.permissions();

        permissions.set_mode(0o755);

        set_permissions(file_path, permissions)?;

        Ok(())
    }
}
