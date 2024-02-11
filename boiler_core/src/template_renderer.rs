use std::collections::{BTreeMap, HashMap};

use once_cell::sync::Lazy;
use tera::Tera;

use crate::actions::ActionData;
use crate::actions_utils::{write_file, ActionIoError};
use crate::context_keys::CONTEXT_ROOT;
use crate::data::Value;

macro_rules! template {
    ($template_name:literal) => {
        (
            $template_name,
            include_str!(concat!("templates/", $template_name)),
        )
    };
}

#[derive(Debug, thiserror::Error)]
pub enum TemplateRendererError {
    #[error("Could not render template")]
    Rendering(#[from] tera::Error),

    #[error("Could not write rendered template")]
    Io(#[from] ActionIoError),
}

pub fn to_yaml_array(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let value = value
        .as_array()
        .ok_or_else(|| tera::Error::msg("value is not an array"))?;

    let array_str = value
        .iter()
        .map(|item| {
            let item_str = item
                .as_str()
                .ok_or_else(|| tera::Error::msg("item is not a string"))?;
            let item_yaml = if item_str.is_empty()
                || item_str.contains('.')
                || item_str.contains('"')
                || item_str.contains(',')
            {
                format!("\"{}\"", item_str.replace('\"', "\\\""))
            } else {
                item_str.to_string()
            };

            Ok::<String, tera::Error>(item_yaml)
        })
        .collect::<Result<Vec<String>, _>>()
        .expect("item is not a string")
        .join(", ");

    Ok(tera::Value::String("[".to_string() + &array_str + "]"))
}

pub fn path_parent(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
    let value = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("value is not a string"))?;

    let parent = std::path::Path::new(value)
        .parent()
        .ok_or_else(|| tera::Error::msg("could not get parent"))?
        .to_string_lossy()
        .to_string();

    Ok(tera::Value::String(parent))
}

pub static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        template!(".pre-commit-config.yaml.j2"),
        template!("LICENSE.j2"),
        template!("rustfmt.toml.j2"),
        template!(".github/dependabot.yml.j2"),
        template!(".github/workflows/docker-publish.yml.j2"),
        template!(".github/workflows/pre-commit.yml.j2"),
        template!(".github/workflows/python.yml.j2"),
        template!(".github/workflows/rust.yml.j2"),
        template!("README.header.md.j2"),
    ])
    .expect("could not add raw templates");
    tera.register_filter("to_yaml_array", to_yaml_array);
    tera.register_filter("path_parent", path_parent);
    tera
});

pub fn render_template(file_name: &str, data: &ActionData) -> Result<(), TemplateRendererError> {
    let template_name = format!("{}.j2", file_name);
    let output = TERA
        .render(&template_name, &build_template_renderer_context(data))
        .map_err(TemplateRendererError::Rendering)?;

    write_file(&data.repo, file_name, &output).map_err(TemplateRendererError::Io)?;

    Ok(())
}

pub fn build_template_renderer_context(data: &ActionData) -> tera::Context {
    Value::new_object(BTreeMap::from([(
        CONTEXT_ROOT.to_string(),
        data.context.clone(),
    )]))
    .into()
}
