use once_cell::sync::Lazy;
use regex::Regex;
use tera::Tera;

use crate::data::{Repo, Value};

#[derive(Debug)]
pub struct ActionData {
    pub repo: Repo,
    pub context: Value,
}

pub trait Action {
    fn run(&self, data: &ActionData);
}

macro_rules! template {
    ($template_name:literal) => {
        (
            $template_name,
            include_str!(concat!("templates/", $template_name)),
        )
    };
}

static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        template!(".pre-commit-config.yaml.j2"),
        template!("LICENSE.j2"),
        template!("rustfmt.toml.j2"),
        template!(".github/dependabot.yml.j2"),
        template!(".github/workflows/rust.yml.j2"),
        template!("README.header.md.j2"),
    ])
    .expect("could not add raw templates");
    tera.autoescape_on(vec![]);
    tera
});

pub struct PreCommitConfigAction;

const PRE_COMMIT_CONFIG_FILENAME: &str = ".pre-commit-config.yaml";

impl Action for PreCommitConfigAction {
    fn run(&self, data: &ActionData) {
        render_template(PRE_COMMIT_CONFIG_FILENAME, data);
    }
}

pub struct LicenseAction;

const LICENSE_FILENAME: &str = "LICENSE";

impl Action for LicenseAction {
    fn run(&self, data: &ActionData) {
        render_template(LICENSE_FILENAME, data);
    }
}

pub struct RustfmtTomlAction;

const RUSTFMT_TOML_FILENAME: &str = "rustfmt.toml";

impl Action for RustfmtTomlAction {
    fn run(&self, data: &ActionData) {
        render_template(RUSTFMT_TOML_FILENAME, data);
    }
}

pub struct DependabotConfigAction;

const DEPENDABOT_CONFIG_FILENAME: &str = ".github/dependabot.yml";

impl Action for DependabotConfigAction {
    fn run(&self, data: &ActionData) {
        render_template(DEPENDABOT_CONFIG_FILENAME, data);
    }
}

pub struct RustCiAction;

const RUST_CI_FILENAME: &str = ".github/workflows/rust.yml";

impl Action for RustCiAction {
    fn run(&self, data: &ActionData) {
        render_template(RUST_CI_FILENAME, data);
    }
}

pub struct ReadmeAction;

const README_FILENAME: &str = "README.md";
const README_HEADER_TEMPLATE: &str = "README.header.md.j2";

impl Action for ReadmeAction {
    fn run(&self, data: &ActionData) {
        let readme_path = data.repo.path().join(README_FILENAME);
        let mut readme = std::fs::read_to_string(readme_path).expect("could not read README.md");

        let header_regex = Regex::new(r"(?m)(?:^.+\n=+$\n|^# .+$|^\s*$\n|^\[!.+\)$\n)*").unwrap();
        if let Some(captures) = header_regex.captures(&readme) {
            let header_end = captures.get(0).unwrap().end();
            readme = readme[header_end..].to_string();
        }

        let output = TERA
            .render(README_HEADER_TEMPLATE, &data.context.clone().into())
            .unwrap_or_else(|_| panic!("could not render {}", README_HEADER_TEMPLATE));
        readme = format!("{}\n{}", output, readme);

        std::fs::write(data.repo.path().join(README_FILENAME), readme)
            .unwrap_or_else(|_| panic!("could not write {}", README_FILENAME));
    }
}

fn render_template(file_name: &str, data: &ActionData) {
    let template_name = format!("{}.j2", file_name);
    let output = TERA
        .render(&template_name, &data.context.clone().into())
        .unwrap_or_else(|_| panic!("could not render {}", template_name));

    std::fs::create_dir_all(data.repo.path().join(file_name).parent().unwrap())
        .expect("could not create directory");
    std::fs::write(data.repo.path().join(file_name), output)
        .unwrap_or_else(|_| panic!("could not write {}", file_name));
}

pub fn run_all_actions(action_data: &ActionData) {
    PreCommitConfigAction.run(action_data);
    LicenseAction.run(action_data);
    RustfmtTomlAction.run(action_data);
    DependabotConfigAction.run(action_data);
    RustCiAction.run(action_data);
    ReadmeAction.run(action_data);
}
