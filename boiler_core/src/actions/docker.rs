use boiler_macros::FunctionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::detectors_utils::ActionDataExt;
use crate::template_renderer;

/// Generates a Docker CI configuration file for GitHub Actions.
#[derive(Debug, FunctionMeta)]
pub struct DockerCiAction;

const DOCKER_CI_FILENAME: &str = ".github/workflows/docker-publish.yml";

impl Action for DockerCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.has_lang("docker") {
            template_renderer::render_template(DOCKER_CI_FILENAME, data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::docker::{DockerCiAction, DOCKER_CI_FILENAME};
    use crate::actions::{Action, ActionData};
    use crate::context_keys;
    use crate::data::Value;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_empty_context() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(repo.repo(), Value::new_object([]));

        DockerCiAction.run(&action_data).unwrap();

        assert!(repo.is_empty());
    }

    #[test]
    fn test_generate() {
        let repo = TempRepo::new();
        let action_data = ActionData::new(
            repo.repo(),
            Value::new_object([
                (
                    context_keys::REPO_OWNER.to_owned(),
                    Value::new_array(vec![Value::new_string("m4tx")]),
                ),
                (
                    context_keys::REPO_NAME.to_owned(),
                    Value::new_array(vec![Value::new_string("boiler")]),
                ),
                (
                    context_keys::REPO_DEFAULT_BRANCH.to_owned(),
                    Value::new_array(vec![Value::new_string("master")]),
                ),
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array(vec![Value::new_string("docker")]),
                ),
                (
                    context_keys::DOCKERFILES.to_owned(),
                    Value::new_array(vec![Value::new_string("Dockerfile")]),
                ),
            ]),
        );

        DockerCiAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(DOCKER_CI_FILENAME));
    }
}
