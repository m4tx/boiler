use anyhow::Context;
use dependabot_config::DependabotConfigAction;
use once_cell::sync::Lazy;
use pre_commit_config::PreCommitConfigAction;
use readme::ReadmeAction;
use rust_ci::RustCiAction;
use rustfmt_toml::RustfmtTomlAction;

use crate::actions::docker::DockerCiAction;
use crate::actions::license::LicenseAction;
use crate::actions::pre_commit_ci::PreCommitCiAction;
use crate::actions::python::PythonCiAction;
use crate::data::{Repo, Value};

mod dependabot_config;
mod docker;
mod license;
mod pre_commit_ci;
mod pre_commit_config;
mod python;
mod readme;
mod rust_ci;
mod rustfmt_toml;

#[derive(Debug)]
pub struct ActionData {
    pub repo: Repo,
    pub context: Value,
}

type ActionResult = anyhow::Result<()>;

pub trait ActionMeta {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn default_enabled(&self) -> bool;
}

pub trait Action: ActionMeta + Send + Sync {
    fn run(&self, data: &ActionData) -> ActionResult;
}

pub static ACTIONS: Lazy<[&dyn Action; 9]> = Lazy::new(|| {
    [
        &PreCommitConfigAction,
        &LicenseAction,
        &RustfmtTomlAction,
        &DependabotConfigAction,
        &RustCiAction,
        &PythonCiAction,
        &DockerCiAction,
        &ReadmeAction,
        &PreCommitCiAction,
    ]
});

pub fn run_all_actions(action_data: &ActionData) -> ActionResult {
    for action in *ACTIONS {
        action
            .run(action_data)
            .with_context(|| format!("Failed to run action: {}", action.name()))?;
    }

    Ok(())
}
