use crate::actions::{Action, ActionData, ActionResult};
use crate::data::Value;
use crate::{context_keys, template_renderer};

pub struct PythonCiAction;

const PYTHON_CI_FILENAME: &str = ".github/workflows/python.yml";

impl Action for PythonCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.context["boiler"][context_keys::LANGS]
            .as_array()
            .expect("no langs detected")
            .contains(&Value::new_string("python"))
        {
            template_renderer::render_template(PYTHON_CI_FILENAME, data)?;
        }
        Ok(())
    }
}
