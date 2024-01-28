use std::path::PathBuf;

use boiler_core::data::Repo;
use boiler_core::run_in_repo;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::InfoLevel;
use color_print::cprintln;

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
    /// List all detectors with a short description
    ListDetectors,
    /// List all actions with a short description
    ListActions,
    /// Run all actions in a repository
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
