use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

pub struct PreCommitConfigAction;

const PRE_COMMIT_CONFIG_FILENAME: &str = ".pre-commit-config.yaml";

impl Action for PreCommitConfigAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(PRE_COMMIT_CONFIG_FILENAME, data)?;
        Ok(())
    }
}
