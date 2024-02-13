use anyhow::Context;
use boiler_macros::FunctionMeta;
use log::warn;
use regex::Regex;

use crate::actions::{Action, ActionData, ActionResult};
use crate::actions_utils::write_file;
use crate::template_renderer::{build_template_renderer_context, TERA};

/// Updated the README.md file header with badges.
#[derive(Debug, FunctionMeta)]
pub struct ReadmeAction;

const README_FILENAME: &str = "README.md";
const README_HEADER_TEMPLATE: &str = "README.header.md.j2";

impl Action for ReadmeAction {
    fn run(&self, data: &ActionData) -> ActionResult {
        let readme_path = data.repo.path().join(README_FILENAME);
        let mut readme = if readme_path.exists() {
            std::fs::read_to_string(&readme_path)
                .with_context(|| format!("could not read {}", README_FILENAME))?
        } else {
            warn!("{} does not exist; generating a new one", README_FILENAME);
            String::new()
        };

        // Match the top-level header, empty lines, and all the badges at the top of the
        // README
        let header_regex = Regex::new(r"(?m)(?:^.+\n=+\n|^# .+\n|^\s*\n|^\[!.+\)\n)*").unwrap();
        if let Some(captures) = header_regex.captures(&readme) {
            let header_end = captures.get(0).unwrap().end();
            readme = readme[header_end..].to_string();
        }

        let output = TERA
            .render(
                README_HEADER_TEMPLATE,
                &build_template_renderer_context(data),
            )
            .with_context(|| format!("could not render {}", README_HEADER_TEMPLATE))?;
        readme = format!("{}\n\n{}", output, readme);
        let readme = readme.trim().to_owned() + "\n";

        write_file(&data.repo, README_FILENAME, &readme)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::readme::{ReadmeAction, README_FILENAME};
    use crate::actions::{Action, ActionData};
    use crate::context_keys;
    use crate::data::Value;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_generate() {
        let repo = TempRepo::new();
        let action_data = get_test_action_data(&repo);

        ReadmeAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(README_FILENAME));
    }

    #[test]
    fn test_overwrite() {
        let repo = TempRepo::new();
        repo.write_str(README_FILENAME, r#"# Example Project
[![Build Status](https://github.com/riichi/trello-to-discord-webhook-service/workflows/some-url)](https://github.com/riichi/trello-to-discord-webhook-service/actions)

This is a very useful tool!"#);

        let action_data = get_test_action_data(&repo);

        ReadmeAction.run(&action_data).unwrap();

        assert!(repo.file_not_empty(README_FILENAME));
        assert_eq!(
            repo.read_str(README_FILENAME),
            r#"Example Project
===============

[![Rust Build Status](https://github.com/m4tx/boiler/workflows/Rust%20CI/badge.svg)](https://github.com/m4tx/boiler/actions/workflows/rust.yml)
[![MIT licensed](https://img.shields.io/github/license/m4tx/boiler)](https://github.com/m4tx/boiler/blob/master/LICENSE)

This is a very useful tool!
"#
        );
    }

    fn get_test_action_data(repo: &TempRepo) -> ActionData {
        ActionData::new(
            repo.repo(),
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array([Value::new_string("rust")]),
                ),
                (
                    context_keys::NAME.to_owned(),
                    Value::new_string("Example Project"),
                ),
                (context_keys::LICENSE.to_owned(), Value::new_string("MIT")),
                (
                    context_keys::REPO_OWNER.to_owned(),
                    Value::new_string("m4tx"),
                ),
                (
                    context_keys::REPO_NAME.to_owned(),
                    Value::new_string("boiler"),
                ),
            ]),
        )
    }
}
