use std::collections::BTreeMap;
use std::path::PathBuf;

use actions::ActionData;
use clap::{Parser, Subcommand};
use data::{Repo, Value};

use crate::context::ContextOverrides;

mod actions;
mod context;
mod data;
mod detectors;

fn run_in_repo(repo: &Repo) {
    let mut data = detectors::detect_all(repo);
    let repo_string = data["repo_owner"].as_string().unwrap().to_owned()
        + "/"
        + data["repo_name"].as_string().unwrap();
    println!("Context: {:?}", data);

    let context_overrides = ContextOverrides::from_yaml_string(include_str!("overrides.yml"));
    if let Some(repo_override) = context_overrides.get(&repo_string) {
        println!(
            "Overriding context for {} with {:?}",
            repo_string, repo_override
        );
        data.override_with(repo_override);
    }

    println!("Context: {:?}", data);
    let context = Value::new_object(BTreeMap::from([("boiler".to_string(), data)]));

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
