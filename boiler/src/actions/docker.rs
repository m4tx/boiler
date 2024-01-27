use boiler_macros::ActionMeta;

use crate::actions::{Action, ActionData, ActionResult};
use crate::data::Value;
use crate::{context_keys, template_renderer};

#[derive(Debug, ActionMeta)]
pub struct DockerCiAction;

const DOCKER_CI_FILENAME: &str = ".github/workflows/docker-publish.yml";

impl Action for DockerCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        if data.context["boiler"][context_keys::LANGS]
            .as_array()
            .expect("No langs detected")
            .contains(&Value::new_string("docker"))
        {
            template_renderer::render_template(DOCKER_CI_FILENAME, data)?;
        }
        Ok(())
    }
}
