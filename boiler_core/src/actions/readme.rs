use anyhow::Context;
use boiler_macros::FunctionMeta;
use regex::Regex;

use crate::actions::{Action, ActionData, ActionResult};
use crate::actions_utils::write_file;
use crate::template_renderer::TERA;

#[derive(Debug, FunctionMeta)]
pub struct ReadmeAction;

const README_FILENAME: &str = "README.md";
const README_HEADER_TEMPLATE: &str = "README.header.md.j2";

impl Action for ReadmeAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        let readme_path = data.repo.path().join(README_FILENAME);
        let mut readme = std::fs::read_to_string(readme_path).expect("could not read README.md");

        let header_regex = Regex::new(r"(?m)(?:^.+\n=+$\n|^# .+$|^\s*$\n|^\[!.+\)$\n)*").unwrap();
        if let Some(captures) = header_regex.captures(&readme) {
            let header_end = captures.get(0).unwrap().end();
            readme = readme[header_end..].to_string();
        }

        let output = TERA
            .render(README_HEADER_TEMPLATE, &data.context.clone().into())
            .with_context(|| format!("could not render {}", README_HEADER_TEMPLATE))?;
        readme = format!("{}\n{}", output, readme);

        write_file(&data.repo, README_FILENAME, &readme)?;

        Ok(())
    }
}
