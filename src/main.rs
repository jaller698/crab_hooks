use clap::{Parser, Subcommand};
use rusty_hooker::{git_hook::GitHook, yml_parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "githook-manager")]
#[command(about = "Manage and reuse Git hooks across repositories", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///// Scan for Git repositories
    // Scan {
    //     #[arg(default_value = "~")]
    //     dir: String,
    // },
    /// List all managed repositories
    ListRepos,
    /// List hooks in a given repository
    ListHooks,
    /// Apply a hook to one or more repositories
    ApplyHook {
        hook_name: String,
        #[arg(required = true)]
        repos: Vec<PathBuf>,
    },
    /// Test if the config is valid
    Test,

    /// Run a hook in the current repo
    Run { hook_name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // Commands::Scan { dir } => println!("Scan"),
        Commands::ListRepos => println!("List repos"),
        Commands::ListHooks => {
            yml_parser::display_hooks();
            ()
        }
        Commands::ApplyHook { hook_name, repos } => {
            println!("Apply hook {} in {:?}", hook_name, repos)
        }
        Commands::Test => println!("Test"),
        Commands::Run { hook_name } => {
            let hooks = match yml_parser::read_file() {
                Ok(hooks) => hooks,
                Err(_) => Vec::<GitHook>::new(),
            };
            for hook in hooks {
                if hook.name == *hook_name {
                    hook.run().expect("Failed to run git hook");
                };
            }
        }
    }
}
