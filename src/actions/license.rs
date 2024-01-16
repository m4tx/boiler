use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

pub struct LicenseAction;

const LICENSE_FILENAME: &str = "LICENSE";

impl Action for LicenseAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(LICENSE_FILENAME, data)?;
        Ok(())
    }
}
