use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

/// Generates a pre-commit configuration file.
#[derive(Debug, FunctionMeta)]
pub struct PreCommitConfigAction;

const PRE_COMMIT_CONFIG_FILENAME: &str = ".pre-commit-config.yaml";

impl Action for PreCommitConfigAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(PRE_COMMIT_CONFIG_FILENAME, data)?;
        Ok(())
    }
}
