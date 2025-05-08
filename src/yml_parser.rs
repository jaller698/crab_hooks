use glob::Pattern;
use std::{
    fmt,
    fs::File,
    path::{Path, PathBuf},
};
use which::which;

#[derive(Debug)]
pub struct ValidationError {
    pub hook_name: String,
    pub field: String,
    pub problem: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hook `{}`: field `{}` — {}",
            self.hook_name, self.field, self.problem
        )
    }
}

use crate::git_hook::GitHook;

pub fn read_file(config_file: PathBuf) -> Result<Vec<GitHook>, Box<dyn std::error::Error>> {
    let f: File;
    if Path::new(&config_file).exists() {
        f = std::fs::File::open(config_file)?;
    } else if Path::new("./config.yml").exists() {
        f = std::fs::File::open("./config.yml")?;
    } else {
        return Err(format!(
            "Cannot locate a config.yml file, please make one here: {:?}",
            config_file
        )
        .into());
    }

    let hooks: Vec<GitHook> = serde_yaml::from_reader(f)?;
    Ok(hooks)
}

pub fn test_config(config_file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let hooks = read_file(config_file)?;

    let mut errors = Vec::new();

    for hook in hooks {
        // --- name
        if hook.name.trim().is_empty() {
            errors.push(ValidationError {
                hook_name: hook.name.clone(),
                field: "name".into(),
                problem: "must not be empty".into(),
            });
        }

        // --- command.cmd must be non‐empty
        let cmd = hook.command.cmd.trim();
        if cmd.is_empty() {
            errors.push(ValidationError {
                hook_name: hook.name.clone(),
                field: "command.cmd".into(),
                problem: "must not be empty".into(),
            });
        } else {
            // --- check existence without executing
            if let Some(dir) = &hook.command.directory {
                // Build the path to <directory>/<cmd>
                let candidate = dir.join(cmd);

                // Try finding it globally on PATH as well
                let found_globally = which(cmd).is_ok();

                if !candidate.exists() && !found_globally {
                    errors.push(ValidationError {
                        hook_name: hook.name.clone(),
                        field: "command.cmd".into(),
                        problem: format!(
                            "could not find executable `{}` in directory {:?} or on PATH",
                            cmd, dir
                        ),
                    });
                }
            } else {
                // No directory specified → must find it globally
                if which(cmd).is_err() {
                    errors.push(ValidationError {
                        hook_name: hook.name.clone(),
                        field: "command.cmd".into(),
                        problem: format!("could not locate `{}` on PATH", cmd),
                    });
                }
            }
        }

        // --- directory
        if let Some(dir) = &hook.command.directory {
            if dir.as_os_str().is_empty() {
                errors.push(ValidationError {
                    hook_name: hook.name.clone(),
                    field: "command.directory".into(),
                    problem: "path is empty".into(),
                });
            }
        }

        // --- glob patterns
        if hook.glob_pattern.is_empty() {
            errors.push(ValidationError {
                hook_name: hook.name.clone(),
                field: "glob_pattern".into(),
                problem: "must contain at least one pattern".into(),
            });
        } else {
            for pat in &hook.glob_pattern {
                if let Err(e) = Pattern::new(pat) {
                    errors.push(ValidationError {
                        hook_name: hook.name.clone(),
                        field: "glob_pattern".into(),
                        problem: format!("invalid glob `{}`: {}", pat, e),
                    });
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        for e in errors {
            eprintln!("Config error: {}", e);
        }
        Err("Config error".into())
    }
}

pub fn display_hooks(config_file: PathBuf) {
    let hooks_result = read_file(config_file);
    match hooks_result {
        Ok(hooks) => {
            for hook in hooks.iter() {
                println!("{}", hook)
            }
        }
        Err(e) => println!("Some error occured in finding the config file: [{}]", e),
    }
}
