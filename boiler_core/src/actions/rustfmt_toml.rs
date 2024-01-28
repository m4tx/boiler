use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::detectors_utils::ActionDataExt;
use crate::template_renderer;

/// Generates a rustfmt configuration file.
#[derive(Debug, FunctionMeta)]
pub struct RustfmtTomlAction;

const RUSTFMT_TOML_FILENAME: &str = "rustfmt.toml";

impl Action for RustfmtTomlAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.has_lang("rust") {
            template_renderer::render_template(RUSTFMT_TOML_FILENAME, data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::rustfmt_toml::{RustfmtTomlAction, RUSTFMT_TOML_FILENAME};
    use crate::actions::{Action, ActionData};
    use crate::context_keys;
    use crate::data::Value;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_empty_context() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(repo.repo(), Value::new_object([]));

        RustfmtTomlAction.run(&action_data).unwrap();

        assert!(repo.is_empty());
    }

    #[test]
    fn test_generate() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(
            repo.repo(),
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array(vec![Value::new_string("rust")]),
            )]),
        );

        RustfmtTomlAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(RUSTFMT_TOML_FILENAME));
    }
}
