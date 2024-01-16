use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

pub struct PythonCiAction;

const PYTHON_CI_FILENAME: &str = ".github/workflows/python.yml";

impl Action for PythonCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(PYTHON_CI_FILENAME, data)?;
        Ok(())
    }
}
