use sqlite::State;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::{DirEntry, WalkDir};

fn should_traverse(entry: &DirEntry) -> bool {
    // List of directory names to ignore
    let ignore_list = [".local", ".cache"];
    // Check if the entry's file name is in the ignore list.
    // We use `to_string_lossy()` because file names may not be valid UTF-8.
    !ignore_list.contains(&entry.file_name().to_string_lossy().as_ref())
}

fn find_git_repos(dir: &Path) -> Vec<PathBuf> {
    let mut repos = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| should_traverse(e))
        .filter_map(Result::ok)
    {
        if entry.file_type().is_dir() {
            let path = entry.path();
            if path.join(".git").is_dir() {
                repos.push(path.to_path_buf());
            }
        }
    }
    repos
}

fn create_sqlite(git_repos: Vec<PathBuf>) {
    let connection = sqlite::open("mydatabase.db").unwrap();

    // Use "CREATE TABLE IF NOT EXISTS" to avoid errors if the table already exists.
    let query = "
        CREATE TABLE IF NOT EXISTS git_repos (
            name TEXT UNIQUE,
            managed INTEGER
        );
    ";
    connection.execute(query).unwrap();
    for repo in git_repos {
        let query = "INSERT OR IGNORE INTO git_repos (name, managed) VALUES (?, 0)";
        let mut statement = connection.prepare(query).unwrap();
        statement
            .bind((1, repo.display().to_string().as_str()))
            .unwrap();
        statement.next().unwrap();
    }

    let query = "SELECT * FROM git_repos";
    let mut statement = connection.prepare(query).unwrap();
    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("enabled = {}", statement.read::<i64, _>("managed").unwrap());
    }
}

fn main() {
    Command::new("ls")
        .arg("-a")
        .spawn()
        .expect("failed to execute command")
        .wait();

    // Replace this with the user directory you want to scan,
    // for example, using the HOME environment variable:
    let home_dir = std::env::var("HOME").expect("Could not get HOME directory");
    let home_path = Path::new(&home_dir);

    let git_repos = find_git_repos(home_path);
    // Output the number of repositories found and list their paths
    println!("Found {} Git repositories:", git_repos.len());
    create_sqlite(git_repos);
}
