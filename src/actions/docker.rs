use crate::actions::{Action, ActionData, ActionResult};
use crate::template_renderer;

pub struct DockerCiAction;

const DOCKER_CI_FILENAME: &str = ".github/workflows/docker-publish.yml";

impl Action for DockerCiAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        template_renderer::render_template(DOCKER_CI_FILENAME, data)?;
        Ok(())
    }
}
