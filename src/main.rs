use std::collections::HashMap;
use std::path::PathBuf;

use actions::ActionData;
use clap::{Parser, Subcommand};
use data::{Repo, Value};

mod actions;
mod data;
mod detectors;

fn run_in_repo(repo: &Repo) {
    let data = detectors::detect_all(repo);
    println!("Context: {:?}", data);
    let context = Value::new_object(HashMap::from([("boiler".to_string(), data)]));
    let action_data = ActionData {
        repo: repo.clone(),
        context,
    };
    actions::run_all_actions(&action_data);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Update {
        #[clap(long, short, default_value = ".")]
        repo: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update { repo } => {
            run_in_repo(&Repo::new(repo.clone()));
        }
    }
}
