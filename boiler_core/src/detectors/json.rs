use boiler_macros::FunctionMeta;

use crate::data::Repo;
use crate::detectors::{Detector, DetectorResult};
use crate::detectors_utils::detect_by_extension;

/// Detects if the project contains JSON files.
#[derive(Debug, FunctionMeta)]
pub struct JsonDetector;

impl Detector for JsonDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        detect_by_extension(repo, &["json"], "json")
    }
}
