use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

#[derive(Debug, FunctionMeta)]
pub struct DependabotConfigAction;

const DEPENDABOT_CONFIG_FILENAME: &str = ".github/dependabot.yml";

impl Action for DependabotConfigAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(DEPENDABOT_CONFIG_FILENAME, data)?;
        Ok(())
    }
}
