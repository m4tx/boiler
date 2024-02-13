use anyhow::Context;
use log::info;

use crate::actions::ActionData;
use crate::context::{RepoConfig, ReposConfig};
use crate::data::{Repo, Value};
use crate::detectors::{create_detectors_enabled, detect_with_defaults};

pub mod actions;
mod actions_utils;
pub mod context;
mod context_keys;
pub mod data;
pub mod detectors;
mod detectors_utils;
pub mod function_meta;
mod template_renderer;
#[cfg(test)]
mod test_utils;
mod time;

pub fn run_in_repo(repo: Repo) -> anyhow::Result<()> {
    let repo_path = repo.path().to_owned();

    let mut data =
        Ok::<Value, anyhow::Error>(detect_with_defaults(&repo, &create_detectors_enabled())?)
            .with_context(|| format!("Could not build context for {}", repo_path.display()))?;
    let repo_string = data["repo_owner"].as_string().unwrap().to_owned()
        + "/"
        + data["repo_name"].as_string().unwrap();
    info!("Detected context:\n{}", data.as_yaml());

    let mut repo_config = RepoConfig::default();

    let repos_config = ReposConfig::from_yaml_string(include_str!("overrides.yml"));
    if let Some(repo_override) = repos_config.get(&repo_string) {
        repo_config = repo_config.override_with(repo_override);
    }

    info!(
        "Overriding context for {} with:\n{}",
        repo_string,
        repo_config.context().as_yaml()
    );
    data.override_with(repo_config.context());

    info!("New context:\n{}", data.as_yaml());

    let actions_enabled = repo_config.create_actions_enabled(&actions::create_actions_enabled())?;
    let action_data = ActionData {
        repo,
        context: data,
    };
    actions::run_actions(&action_data, &actions_enabled)
        .with_context(|| format!("Could not run actions for {}", repo_path.display()))?;

    Ok(())
}
