use std::path::PathBuf;

use clap::{Parser, Subcommand};
use crab_hooks::{git_hook::GitHook, hook_types::HookTypes, sqllite, yml_parser};

#[derive(Parser)]
#[command(name = "githook-manager")]
#[command(about = "Manage and reuse Git hooks across repositories", long_about = None)]
/// Manage and reuse Git hooks across repositories
struct Cli {
    #[arg(long, global = true)]
    config_file: Option<PathBuf>,

    #[arg(long, global = true)]
    force: bool,

    #[arg(long = "no-test", global = true)]
    no_test: bool,

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
        hook_type: HookTypes,
    },
    RemoveHook {
        hook_name: String,
        #[arg(required = true)]
        hook_type: HookTypes,
    },
    /// Test if the config is valid
    Test,

    /// Run a hook in the current repo
    Run { hook_name: String },
}

fn find_hook(name: &String) -> Result<GitHook, Box<dyn std::error::Error>> {
    let hooks = yml_parser::read_file().unwrap_or_default();
    for hook in hooks {
        if hook.name == *name {
            return Ok(hook);
        };
    }
    Err("No such hook found, please add it to the config".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config_file = cli.config_file.unwrap_or_else(|| {
        let mut p = home::home_dir().expect("Could not find HOME directory");
        p.push(".config/crabs_hooks/config.yml");
        p
    });

    let sql_config = sqllite::SqlLiteConfig::new("mydb.db")?;

    match &cli.command {
        // Commands::Scan { dir } => println!("Scan"),
        Commands::ListRepos => println!("List repos"),
        Commands::ListHooks => {
            yml_parser::display_hooks();
        }
        Commands::ApplyHook {
            hook_name,
            hook_type,
        } => {
            return find_hook(hook_name)
                .expect("Failed to find the hook")
                .apply_hook(hook_type, &sql_config);
        }
        Commands::RemoveHook {
            hook_name,
            hook_type,
        } => {
            return find_hook(hook_name)
                .expect("Failed to find the hook")
                .apply_hook(hook_type, &sql_config);
        }
        Commands::Test => println!("Test"),
        Commands::Run { hook_name } => {
            let hook = find_hook(hook_name).expect("Failed to find hook");
            return hook.run(&sql_config);
        }
    }
    Ok(())
}
