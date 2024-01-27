use boiler_macros::ActionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

#[derive(Debug, ActionMeta)]
pub struct PreCommitCiAction;

const PRE_COMMIT_CI_FILENAME: &str = ".github/workflows/pre-commit.yml";

impl Action for PreCommitCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(PRE_COMMIT_CI_FILENAME, data)?;
        Ok(())
    }
}
