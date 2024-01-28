use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

/// Generates a pre-commit configuration file.
#[derive(Debug, FunctionMeta)]
pub struct PreCommitConfigAction;

const PRE_COMMIT_CONFIG_FILENAME: &str = ".pre-commit-config.yaml";

impl Action for PreCommitConfigAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(PRE_COMMIT_CONFIG_FILENAME, data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::pre_commit_config::{PreCommitConfigAction, PRE_COMMIT_CONFIG_FILENAME};
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

        PreCommitConfigAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(PRE_COMMIT_CONFIG_FILENAME));
    }
}
