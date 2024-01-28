use boiler_macros::FunctionMeta;

use crate::data::Repo;
use crate::detectors::{Detector, DetectorResult};
use crate::detectors_utils::detect_by_extension;

/// Detects if the project contains TOML files.
#[derive(Debug, FunctionMeta)]
pub struct TomlDetector;

impl Detector for TomlDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        detect_by_extension(repo, &["toml"], "toml")
    }
}
