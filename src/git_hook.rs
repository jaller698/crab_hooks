use git2::{DiffOptions, Repository, StatusOptions};
use globset::GlobBuilder;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, set_permissions, OpenOptions},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
};

use crate::{hook_types::HookTypes, sqllite::SqlLiteConfig};

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
    fn find_changed_or_to_be_pushed_files(
        &self,
    ) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let repo = Repository::discover(".")?;
        let workdir = repo
            .workdir()
            .ok_or_else(|| git2::Error::from_str("not a workdir"))?;

        // 1) Gather unstaged + staged changes
        let mut opts = StatusOptions::new();
        opts.include_untracked(true)
            .recurse_untracked_dirs(true)
            .include_ignored(false)
            .renames_head_to_index(true);

        let statuses = repo.statuses(Some(&mut opts))?;
        let mut paths: Vec<PathBuf> = statuses
            .iter()
            .filter_map(|e| {
                let s = e.status();
                let changed = s.is_index_new()
                    || s.is_index_modified()
                    || s.is_index_deleted()
                    || s.is_wt_new()
                    || s.is_wt_modified()
                    || s.is_wt_deleted();
                if changed {
                    e.path().map(|p| workdir.join(p))
                } else {
                    None
                }
            })
            .collect();

        // 2) Now diff upstream → HEAD to pick up committed‑but‑not‑pushed files
        if let Ok(upstream_obj) = repo.revparse_single("@{u}") {
            // peel to commits
            let upstream_commit = upstream_obj.peel_to_commit()?;
            let head_commit = repo.head()?.peel_to_commit()?;

            let upstream_tree = upstream_commit.tree()?;
            let head_tree = head_commit.tree()?;

            let mut diff_opts = DiffOptions::new();
            let diff = repo.diff_tree_to_tree(
                Some(&upstream_tree),
                Some(&head_tree),
                Some(&mut diff_opts),
            )?;

            for delta in diff.deltas() {
                // new_file() covers added/modified/deleted
                if let Some(p) = delta.new_file().path() {
                    paths.push(workdir.join(p));
                }
            }
        }
        // else: no upstream configured → skip this part

        // 3) Dedupe & return
        paths.sort();
        paths.dedup();
        Ok(paths)
    }

    fn check_files_match_glob(&self) -> bool {
        let file_result = self.find_changed_or_to_be_pushed_files();
        if let Ok(files) = file_result {
            for pattern in &self.glob_pattern {
                if let Ok(glob) = GlobBuilder::new(pattern).literal_separator(true).build() {
                    let glob_matcher = glob.compile_matcher();
                    for path in &files {
                        let relative_path =
                            path.strip_prefix(std::env::current_dir().unwrap()).unwrap();
                        if glob_matcher.is_match(relative_path) {
                            println!("pattern {} matched {:?}", pattern, relative_path);
                            return true;
                        }
                    }
                };
            }
        };
        false
    }

    pub fn run(&self, sql_config: &SqlLiteConfig) -> Result<(), Box<dyn std::error::Error>> {
        if !self.check_files_match_glob() {
            println!("Pattern does not match the glob provided, skipping this!");
            return Ok(());
        }

        println!("Running {}", self.command.cmd);
        let mut cmd = Command::new(&self.command.cmd);
        if let Some(v) = &self.command.args {
            let args = v.split(" ");
            cmd.args(args);
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
            sql_config.add_successful_run(&self.name)?;
            Ok(())
        } else {
            // non‐zero or signal‐terminated
            sql_config.add_failed_run(&self.name)?;
            match status.code() {
                // exited with some code != 0
                Some(code) => Err(format!("Command failed with status {}", code).into()),
                // e.g. killed by signal on Unix
                None => Err("Cmd terminated by signal".into()),
            }
        }
    }

    pub fn apply_hook(
        &self,
        hook_type: &HookTypes,
        sql_config: &SqlLiteConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("Apply hook {} as {}", self.name, hook_type);
        let cd = std::env::current_dir()?
            .to_str()
            .expect("Failed to get current dir")
            .to_string();

        // First check if the current directory is a git repo
        fs::read_dir("./.git/")?;

        // Check if there is already a git hook
        let mut already_managed = false;

        let file_path = format!("./.git/hooks/{}", hook_type);
        match sql_config.check_if_new_hook_is_known(&cd, hook_type) {
            Ok(true) => match sql_config.check_if_new_hook_is_same(&cd, hook_type, &self.name) {
                Ok(false) => {
                    println!("There is already a existing managed git hook, will try to truncate exisiting config");
                    already_managed = true;
                }
                Ok(true) => {
                    return Err(
                        "Git hooks is already setup for this repo with this type, aborting".into(),
                    );
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to check for pre exisiting similar hooks, error: {}",
                        e
                    )
                    .into());
                }
            },
            Ok(false) => {
                if fs::read(&file_path).is_ok() {
                    return Err(
                        "Failed to apply hook, the selected hook type already exists, and may not be managed".into(),
                    );
                }
            }
            Err(e) => {
                return Err(
                    format!("Failed to check for pre exisiting hooks, error: {}", e).into(),
                );
            }
        }

        let exe_location = std::env::current_exe()?;
        let file_content = format!("{} run {}", exe_location.to_str().expect(""), self.name);
        if !already_managed {
            let mut hook_file = fs::File::create(&file_path)?;
            writeln!(hook_file, "#!/usr/bin/env sh")?;
            writeln!(hook_file, "set -e")?;
            writeln!(hook_file, "{}", file_content)?;
            drop(hook_file);

            let mut permissions = fs::metadata(&file_path)?.permissions();
            permissions.set_mode(0o755);
            set_permissions(file_path, permissions)?;
        } else {
            let mut hook_file = OpenOptions::new().append(true).open(file_path)?;
            writeln!(hook_file, "{}", file_content)?;
            drop(hook_file);
        }

        sql_config.add_hook(&self.name)?;
        sql_config.add_hook_to_repo(&self.name, &cd, hook_type)?;

        Ok(())
    }
}
