use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

pub struct RustCiAction;

const RUST_CI_FILENAME: &str = ".github/workflows/rust.yml";

impl Action for RustCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(RUST_CI_FILENAME, data)?;
        Ok(())
    }
}
