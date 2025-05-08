use std::path::PathBuf;

use clap::{Parser, Subcommand};
use crab_hooks::{
    git_hook::GitHook,
    hook_types::HookTypes,
    sqllite,
    yml_parser::{self, test_config},
};

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
    DeleteHook {
        #[arg(required = true)]
        hook_name: String,
    },
    /// Test if the config is valid
    Test,

    /// Run a hook in the current repo
    Run { hook_name: String },
}

fn find_hook(config_file: PathBuf, name: &String) -> Result<GitHook, Box<dyn std::error::Error>> {
    let hooks = yml_parser::read_file(config_file).unwrap_or_default();
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
    let sql_config = if let Some(sql_db_path) = config_file.parent() {
        let sql_db_path = sql_db_path.join("hooks.db");
        sqllite::SqlLiteConfig::new(sql_db_path.to_str().unwrap())?
    } else {
        sqllite::SqlLiteConfig::new("mydb.db")?
    };

    match &cli.command {
        // Commands::Scan { dir } => println!("Scan"),
        Commands::ListRepos => println!("List repos"),
        Commands::ListHooks => {
            yml_parser::display_hooks(config_file);
        }
        Commands::ApplyHook {
            hook_name,
            hook_type,
        } => {
            return find_hook(config_file, hook_name)
                .expect("Failed to find the hook")
                .apply_hook(hook_type, &sql_config);
        }
        Commands::RemoveHook {
            hook_name,
            hook_type,
        } => {
            return find_hook(config_file, hook_name)
                .expect("Failed to find the hook")
                .remove_hook(hook_type, &sql_config);
        }
        Commands::DeleteHook { hook_name } => {
            return find_hook(config_file.clone(), hook_name)
                .expect("Failed to find the git hook to be deleted")
                .delete_hook(&sql_config, config_file);
        }
        Commands::Test => match test_config(config_file) {
            Ok(_) => {
                println!("Config is good to go!");
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        },
        Commands::Run { hook_name } => {
            let hook = find_hook(config_file, hook_name).expect("Failed to find hook");
            return hook.run(&sql_config);
        }
    }
    Ok(())
}
