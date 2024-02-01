use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::{context_keys, template_renderer};

/// Generates the LICENSE file, updating year or author if necessary.
#[derive(Debug, FunctionMeta)]
pub struct LicenseAction;

const LICENSE_FILENAME: &str = "LICENSE";

impl Action for LicenseAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.context[context_keys::LICENSE]
            .as_string()
            .expect("license is of invalid type")
            != "LicenseRef-proprietary"
        {
            template_renderer::render_template(LICENSE_FILENAME, data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::license::{LicenseAction, LICENSE_FILENAME};
    use crate::actions::{Action, ActionData};
    use crate::context_keys;
    use crate::data::Value;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_proprietary() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(
            repo.repo(),
            Value::new_object([(
                context_keys::LICENSE.to_owned(),
                Value::new_string("LicenseRef-proprietary"),
            )]),
        );
        LicenseAction.run(&action_data).unwrap();

        assert!(repo.is_empty());
    }

    #[test]
    fn test_generate_mit() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(
            repo.repo(),
            Value::new_object([
                (context_keys::LICENSE.to_owned(), Value::new_string("MIT")),
                (
                    context_keys::FULL_NAME.to_owned(),
                    Value::new_string("John Doe"),
                ),
                (
                    context_keys::FIRST_ACTIVITY_YEAR.to_owned(),
                    Value::new_number(2020),
                ),
                (
                    context_keys::LAST_ACTIVITY_YEAR.to_owned(),
                    Value::new_number(2024),
                ),
            ]),
        );

        LicenseAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(LICENSE_FILENAME));
    }

    #[test]
    fn test_generate_gpl() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(
            repo.repo(),
            Value::new_object([(
                context_keys::LICENSE.to_owned(),
                Value::new_string("GNU GPL v3"),
            )]),
        );

        LicenseAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(LICENSE_FILENAME));
        assert!(repo
            .read_str(LICENSE_FILENAME)
            .starts_with("                    GNU GENERAL PUBLIC LICENSE"));
    }
}
