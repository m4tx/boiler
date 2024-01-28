use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

/// Generates a Dependabot configuration file.
#[derive(Debug, FunctionMeta)]
pub struct DependabotConfigAction;

const DEPENDABOT_CONFIG_FILENAME: &str = ".github/dependabot.yml";

impl Action for DependabotConfigAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(DEPENDABOT_CONFIG_FILENAME, data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::dependabot_config::{DependabotConfigAction, DEPENDABOT_CONFIG_FILENAME};
    use crate::actions::{Action, ActionData};
    use crate::context_keys;
    use crate::data::Value;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_generate() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(
            repo.repo(),
            Value::new_object([(context_keys::LANGS.to_owned(), Value::new_array([]))]),
        );

        DependabotConfigAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(DEPENDABOT_CONFIG_FILENAME));
    }
}
