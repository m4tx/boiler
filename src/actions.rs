use dependabot_config::DependabotConfigAction;
use pre_commit_config::PreCommitConfigAction;
use readme::ReadmeAction;
use rust_ci::RustCiAction;
use rustfmt_toml::RustfmtTomlAction;

use crate::actions::docker::DockerCiAction;
use crate::actions::python::PythonCiAction;
use crate::data::{Repo, Value};

mod dependabot_config;
mod docker;
mod license;
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

pub trait Action {
    fn run(&self, data: &ActionData) -> ActionResult;
}

pub fn run_all_actions(action_data: &ActionData) -> ActionResult {
    PreCommitConfigAction.run(action_data)?;
    // LicenseAction.run(action_data)?;
    RustfmtTomlAction.run(action_data)?;
    DependabotConfigAction.run(action_data)?;
    RustCiAction.run(action_data)?;
    PythonCiAction.run(action_data)?;
    DockerCiAction.run(action_data)?;
    ReadmeAction.run(action_data)?;

    Ok(())
}
