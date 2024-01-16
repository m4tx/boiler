use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

pub struct RustfmtTomlAction;

const RUSTFMT_TOML_FILENAME: &str = "rustfmt.toml";

impl Action for RustfmtTomlAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(RUSTFMT_TOML_FILENAME, data)?;
        Ok(())
    }
}
