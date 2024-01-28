use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::detectors_utils::ActionDataExt;
use crate::template_renderer;

/// Generates a Python CI configuration file for GitHub Actions.
#[derive(Debug, FunctionMeta)]
pub struct PythonCiAction;

const PYTHON_CI_FILENAME: &str = ".github/workflows/python.yml";

impl Action for PythonCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.has_lang("python") {
            template_renderer::render_template(PYTHON_CI_FILENAME, data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::python::{PythonCiAction, PYTHON_CI_FILENAME};
    use crate::actions::{Action, ActionData};
    use crate::context_keys;
    use crate::data::Value;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_empty_context() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(repo.repo(), Value::new_object([]));

        PythonCiAction.run(&action_data).unwrap();

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
                    Value::new_array(vec![Value::new_string("python")]),
                ),
                (
                    context_keys::PYTHON_PACKAGE_MANAGERS.to_owned(),
                    Value::new_array(vec![Value::new_string("poetry")]),
                ),
                (
                    context_keys::FRAMEWORKS.to_owned(),
                    Value::new_array(vec![Value::new_string("django")]),
                ),
            ]),
        );

        PythonCiAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(PYTHON_CI_FILENAME));
    }
}
