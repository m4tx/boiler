use std::collections::HashMap;

use once_cell::sync::Lazy;
use tera::Tera;

use crate::actions::ActionData;

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
    #[error("could not render template")]
    Rendering(#[from] tera::Error),

    #[error("could not write file")]
    Io(#[from] std::io::Error),
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
            let item_quoted = format!("\"{}\"", item_str.replace('\"', "\\\""));
            Ok::<String, tera::Error>(item_quoted)
        })
        .collect::<Result<Vec<String>, _>>()
        .expect("item is not a string")
        .join(", ");

    Ok(tera::Value::String("[ ".to_string() + &array_str + " ]"))
}

pub static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        template!(".pre-commit-config.yaml.j2"),
        template!("LICENSE.j2"),
        template!("rustfmt.toml.j2"),
        template!(".github/dependabot.yml.j2"),
        template!(".github/workflows/docker-publish.yml.j2"),
        template!(".github/workflows/python.yml.j2"),
        template!(".github/workflows/rust.yml.j2"),
        template!("README.header.md.j2"),
    ])
    .expect("could not add raw templates");
    tera.register_filter("to_yaml_array", to_yaml_array);
    tera
});

pub fn render_template(file_name: &str, data: &ActionData) -> Result<(), TemplateRendererError> {
    let template_name = format!("{}.j2", file_name);
    let output = TERA
        .render(&template_name, &data.context.clone().into())
        .map_err(TemplateRendererError::Rendering)?;

    std::fs::create_dir_all(data.repo.path().join(file_name).parent().unwrap())
        .map_err(TemplateRendererError::Io)?;
    std::fs::write(data.repo.path().join(file_name), output).map_err(TemplateRendererError::Io)?;

    Ok(())
}
