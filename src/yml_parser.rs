use crate::git_hook::GitHook;

pub fn read_file() -> Result<Vec<GitHook>, Box<dyn std::error::Error>> {
    const CONFIG_FILE: &str = "./config.yml";
    let f = std::fs::File::open(CONFIG_FILE)?;
    let hooks: Vec<GitHook> = serde_yaml::from_reader(f)?;

    Ok(hooks)
}

pub fn display_hooks() {
    let hooks_result = read_file();
    match hooks_result {
        Ok(hooks) => {
            for hook in hooks.iter() {
                println!("{}", hook)
            }
        }
        Err(e) => println!("Some error occured in finding the config file: [{}]", e),
    }
}
