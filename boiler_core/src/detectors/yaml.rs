use boiler_macros::FunctionMeta;

use crate::data::Repo;
use crate::detectors::{Detector, DetectorResult};
use crate::detectors_utils::detect_by_extension;

#[derive(Debug, FunctionMeta)]
pub struct YamlDetector;

impl Detector for YamlDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        detect_by_extension(repo, &["yaml", "yml"], "yaml")
    }
}
