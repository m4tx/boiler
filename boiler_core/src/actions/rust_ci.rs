use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::detectors_utils::ActionDataExt;
use crate::template_renderer;

/// Generates a Rust CI configuration file for GitHub Actions.
#[derive(Debug, FunctionMeta)]
pub struct RustCiAction;

const RUST_CI_FILENAME: &str = ".github/workflows/rust.yml";

impl Action for RustCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.has_lang("rust") {
            template_renderer::render_template(RUST_CI_FILENAME, data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::rust_ci::{RustCiAction, RUST_CI_FILENAME};
    use crate::actions::{Action, ActionData};
    use crate::context_keys;
    use crate::data::Value;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_empty_context() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(repo.repo(), Value::new_object([]));

        RustCiAction.run(&action_data).unwrap();

        assert!(repo.is_empty());
    }

    #[test]
    fn test_generate() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(
            repo.repo(),
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array(vec![Value::new_string("rust")]),
                ),
                (
                    context_keys::GH_ACTIONS_RUST_VERSIONS.to_owned(),
                    Value::new_array(vec![Value::new_string("stable")]),
                ),
                (
                    context_keys::GH_ACTIONS_RUST_FEATURES.to_owned(),
                    Value::new_array(vec![Value::new_string("default")]),
                ),
                (
                    context_keys::GH_ACTIONS_RUST_OS.to_owned(),
                    Value::new_array(vec![Value::new_string("ubuntu-latest")]),
                ),
            ]),
        );

        RustCiAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(RUST_CI_FILENAME));
    }
}
