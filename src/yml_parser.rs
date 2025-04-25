use std::{
    fs::File,
    path::{Path, PathBuf},
};

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
