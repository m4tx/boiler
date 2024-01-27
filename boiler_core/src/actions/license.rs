use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::{context_keys, template_renderer};

#[derive(Debug, FunctionMeta)]
pub struct LicenseAction;

const LICENSE_FILENAME: &str = "LICENSE";

impl Action for LicenseAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.context["boiler"][context_keys::LICENSE]
            .as_string()
            .expect("license is of invalid type")
            != "LicenseRef-proprietary"
        {
            template_renderer::render_template(LICENSE_FILENAME, data)?;
        }
        Ok(())
    }
}
