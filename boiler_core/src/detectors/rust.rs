use boiler_macros::FunctionMeta;
use ignore::Walk;
use path_slash::PathExt;
use serde::Deserialize;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Option<CargoPackage>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: Option<String>,
    authors: Option<Vec<String>>,
}

/// Detects if the project contains Rust files, and retrieves basic metadata
/// from Cargo.toml, such as authors or the crate name.
#[derive(Debug, FunctionMeta)]
pub struct RustDetector;

impl Detector for RustDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::empty_object();

        let cargo_toml = repo.path().join("Cargo.toml");
        if cargo_toml.exists() {
            data.insert(context_keys::LANGS, vec![Value::new_string("rust")]);

            let cargo_toml: CargoToml = toml::from_str(
                &std::fs::read_to_string(&cargo_toml).expect("could not read Cargo.toml"),
            )
            .expect("could not parse Cargo.toml");
            if let Some(package) = &cargo_toml.package {
                if let Some(name) = &package.name {
                    data.insert(context_keys::CRATE_NAME, name);
                }
                if let Some(authors) = &package.authors {
                    if let Some(author) = authors.first() {
                        let full_name = author
                            .find('<')
                            .map(|index| &author[..index])
                            .unwrap_or(author)
                            .trim();
                        data.insert(context_keys::FULL_NAME, full_name);
                    }
                }
            }
        }

        let trunk_data = self.detect_trunk(repo)?;
        data.union(&trunk_data)
            .expect("could not merge trunk context data");

        Ok(data)
    }
}

impl RustDetector {
    fn detect_trunk(&self, repo: &Repo) -> DetectorResult {
        let mut config_paths = vec![];

        for entry in Walk::new(repo.path()) {
            let path = entry?.path().to_owned();
            if path.is_file() && path.file_name().unwrap_or_default() == "Trunk.toml" {
                let path_on_repo = path
                    .strip_prefix(repo.path())
                    .expect("could not strip repo prefix")
                    .to_owned();
                config_paths.push(path_on_repo);
            }
        }

        if config_paths.is_empty() {
            Ok(Value::empty_object())
        } else {
            let mut data = Value::empty_object();
            data.insert(context_keys::FRAMEWORKS, [Value::new_string("trunk")]);
            let config_paths: Vec<_> = config_paths
                .into_iter()
                .map(|path| Value::new_string(path.to_slash_lossy()))
                .collect();
            data.insert(context_keys::TRUNK_CONFIGS, config_paths);
            Ok(data)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::rust::RustDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_rust() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str(
            "Cargo.toml",
            r"
            [package]
            name = 'my_crate'
            authors = ['John Doe <test@example.com>']",
        );

        let detector = RustDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array(vec![Value::new_string("rust")])
                ),
                (
                    context_keys::CRATE_NAME.to_owned(),
                    Value::new_string("my_crate")
                ),
                (
                    context_keys::FULL_NAME.to_owned(),
                    Value::new_string("John Doe")
                ),
            ])
        );
    }

    #[test]
    fn test_detect_trunk() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str(
            "Cargo.toml",
            r"
            [package]
            name = 'my_crate'
            authors = ['John Doe <test@example.com>']",
        );
        temp_repo.write_str(
            "subdir/Trunk.toml",
            r#"[[proxy]]
rewrite = "api"
"#,
        );

        let detector = RustDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array([Value::new_string("rust")])
                ),
                (
                    context_keys::CRATE_NAME.to_owned(),
                    Value::new_string("my_crate")
                ),
                (
                    context_keys::FULL_NAME.to_owned(),
                    Value::new_string("John Doe")
                ),
                (
                    context_keys::FRAMEWORKS.to_owned(),
                    Value::new_array([Value::new_string("trunk")])
                ),
                (
                    context_keys::TRUNK_CONFIGS.to_owned(),
                    Value::new_array([Value::new_string("subdir/Trunk.toml")])
                ),
            ])
        );
    }

    #[test]
    fn test_detect_trunk_in_base() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str(
            "Cargo.toml",
            r"
            [package]
            name = 'my_crate'
            authors = ['John Doe <test@example.com>']",
        );
        temp_repo.write_str(
            "Trunk.toml",
            r#"[[proxy]]
rewrite = "api"
"#,
        );

        let detector = RustDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array([Value::new_string("rust")])
                ),
                (
                    context_keys::CRATE_NAME.to_owned(),
                    Value::new_string("my_crate")
                ),
                (
                    context_keys::FULL_NAME.to_owned(),
                    Value::new_string("John Doe")
                ),
                (
                    context_keys::FRAMEWORKS.to_owned(),
                    Value::new_array([Value::new_string("trunk")])
                ),
                (
                    context_keys::TRUNK_CONFIGS.to_owned(),
                    Value::new_array([Value::new_string("Trunk.toml")])
                ),
            ])
        );
    }
}
