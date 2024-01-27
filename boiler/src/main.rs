use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::Context;
use boiler::actions::ActionData;
use boiler::context::ContextOverrides;
use boiler::data::{Repo, Value};
use clap::{Parser, Subcommand};
use clap_verbosity_flag::InfoLevel;
use log::info;

fn run_in_repo(repo: Repo) -> anyhow::Result<()> {
    let repo_path = repo.path().to_owned();

    let mut data = boiler::detectors::detect_all(&repo)
        .with_context(|| format!("Could not build context for {}", repo_path.display()))?;
    let repo_string = data["repo_owner"].as_string().unwrap().to_owned()
        + "/"
        + data["repo_name"].as_string().unwrap();
    info!("Detected context:\n{}", data.as_yaml());

    let context_overrides = ContextOverrides::from_yaml_string(include_str!("overrides.yml"));
    if let Some(repo_override) = context_overrides.get(&repo_string) {
        info!(
            "Overriding context for {} with:\n{}",
            repo_string,
            repo_override.as_yaml()
        );
        data.override_with(repo_override);

        info!("New context:\n{}", data.as_yaml());
    }

    let context = Value::new_object(BTreeMap::from([("boiler".to_string(), data)]));

    let action_data = ActionData { repo, context };
    boiler::actions::run_all_actions(&action_data)
        .with_context(|| format!("Could not run actions for {}", repo_path.display()))?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Update {
        #[clap(long, short, default_value = ".")]
        repo: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_module("boiler", cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Commands::Update { repo } => {
            run_in_repo(Repo::new(repo.clone()))?;
        }
    }

    Ok(())
}
