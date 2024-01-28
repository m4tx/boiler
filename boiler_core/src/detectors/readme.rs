use std::collections::BTreeMap;

use boiler_macros::FunctionMeta;
use regex::Regex;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

/// Retrieves the project name from the README.md file.
#[derive(Debug, FunctionMeta)]
pub struct ReadmeDetector;

impl Detector for ReadmeDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::new_object(BTreeMap::new());

        let readme_path = repo.path().join("README.md");
        if readme_path.exists() {
            let readme = std::fs::read_to_string(readme_path).expect("could not read README.md");
            let name_regex = Regex::new(r"(?m)^(.+)\n=+$").unwrap();
            if let Some(captures) = name_regex.captures(&readme) {
                let name = captures.get(1).unwrap().as_str();
                data.insert(context_keys::NAME, name);
            }
        }

        Ok(data)
    }
}
