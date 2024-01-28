use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::data::Value;
use crate::{context_keys, template_renderer};

/// Generates a rustfmt configuration file.
#[derive(Debug, FunctionMeta)]
pub struct RustfmtTomlAction;

const RUSTFMT_TOML_FILENAME: &str = "rustfmt.toml";

impl Action for RustfmtTomlAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.context["boiler"][context_keys::LANGS]
            .as_array()
            .expect("no langs detected")
            .contains(&Value::new_string("rust"))
        {
            template_renderer::render_template(RUSTFMT_TOML_FILENAME, data)?;
        }
        Ok(())
    }
}
