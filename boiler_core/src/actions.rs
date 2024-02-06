use anyhow::Context;
use dependabot_config::DependabotConfigAction;
use log::debug;
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
use crate::function_meta::{FunctionEnabled, FunctionMeta};

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

impl ActionData {
    #[must_use]
    pub fn new(repo: Repo, context: Value) -> Self {
        Self { repo, context }
    }
}

type ActionResult = anyhow::Result<()>;

pub trait Action: FunctionMeta + Send + Sync {
    fn run(&self, data: &ActionData) -> ActionResult;
}

pub static ACTIONS: Lazy<[&dyn Action; 9]> = Lazy::new(|| {
    [
        &DependabotConfigAction,
        &DockerCiAction,
        &LicenseAction,
        &PreCommitCiAction,
        &PreCommitConfigAction,
        &PythonCiAction,
        &ReadmeAction,
        &RustCiAction,
        &RustfmtTomlAction,
    ]
});

pub fn run_actions(action_data: &ActionData, actions_enabled: &FunctionEnabled) -> ActionResult {
    for action in *ACTIONS {
        if actions_enabled.is_enabled(action.name()) {
            action
                .run(action_data)
                .with_context(|| format!("Failed to run action: {}", action.name()))?;
        }
    }

    Ok(())
}

pub fn run_all_actions(action_data: &ActionData) -> ActionResult {
    let actions_enabled = create_actions_enabled();

    run_actions(action_data, &actions_enabled)?;

    Ok(())
}

fn create_actions_enabled() -> FunctionEnabled {
    let mut actions_enabled = FunctionEnabled::new();

    for action in ACTIONS.iter() {
        debug!("Running action: {}", action.name());
        actions_enabled.add(action.name().to_owned(), action.default_enabled());
    }

    actions_enabled
}
