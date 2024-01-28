use std::collections::BTreeMap;

use boiler_macros::FunctionMeta;
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
        let mut data = Value::new_object(BTreeMap::new());

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

        Ok(data)
    }
}
