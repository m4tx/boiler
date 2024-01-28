use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::Context;
use boiler_core::actions::ActionData;
use boiler_core::context::ContextOverrides;
use boiler_core::data::{Repo, Value};
use clap::{Parser, Subcommand};
use clap_verbosity_flag::InfoLevel;
use color_print::cprintln;
use log::info;

fn run_in_repo(repo: Repo) -> anyhow::Result<()> {
    let repo_path = repo.path().to_owned();

    let mut data = boiler_core::detectors::detect_all(&repo)
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
    boiler_core::actions::run_all_actions(&action_data)
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
    /// List all detectors with a short description
    ListDetectors,
    /// List all actions with a short description
    ListActions,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_module("boiler", cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Commands::ListDetectors => {
            list_detectors();
        }
        Commands::ListActions => {
            list_actions();
        }
        Commands::Update { repo } => {
            run_in_repo(Repo::new(repo.clone()))?;
        }
    }

    Ok(())
}

fn list_detectors() {
    let detectors = &boiler_core::detectors::DETECTORS;
    let detectors_meta: Vec<_> = detectors
        .iter()
        .map(|detector| {
            (
                detector.name(),
                detector.description(),
                detector.default_enabled(),
            )
        })
        .collect();

    print_functions("Detectors", &detectors_meta);
}

fn list_actions() {
    let actions = &boiler_core::actions::ACTIONS;
    let actions_meta: Vec<_> = actions
        .iter()
        .map(|action| {
            (
                action.name(),
                action.description(),
                action.default_enabled(),
            )
        })
        .collect();

    print_functions("Actions", &actions_meta);
}

fn print_functions(section_name: &str, functions: &[(&str, &str, bool)]) {
    let max_len = functions
        .iter()
        .map(|function| function.0.len())
        .max()
        .unwrap_or(0);

    cprintln!(
        "<strong><underline>{}:</underline></strong>\n",
        section_name
    );

    for function in functions {
        let enabled_str = if function.2 { "enabled" } else { "disabled" };
        cprintln!(
            "  <strong>{:<width$}</strong>  {} [default: {}]",
            function.0,
            function.1,
            enabled_str,
            width = max_len
        );
    }
}
