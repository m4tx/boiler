use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::data::Value;
use crate::{context_keys, template_renderer};

#[derive(Debug, FunctionMeta)]
pub struct RustCiAction;

const RUST_CI_FILENAME: &str = ".github/workflows/rust.yml";

impl Action for RustCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.context["boiler"][context_keys::LANGS]
            .as_array()
            .expect("no langs detected")
            .contains(&Value::new_string("rust"))
        {
            template_renderer::render_template(RUST_CI_FILENAME, data)?;
        }
        Ok(())
    }
}
