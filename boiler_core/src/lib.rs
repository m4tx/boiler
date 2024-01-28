use anyhow::Context;
use log::info;

use crate::actions::ActionData;
use crate::context::ContextOverrides;
use crate::data::Repo;

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

    let mut data = detectors::detect_all(&repo)
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

    let action_data = ActionData {
        repo,
        context: data,
    };
    actions::run_all_actions(&action_data)
        .with_context(|| format!("Could not run actions for {}", repo_path.display()))?;

    Ok(())
}
