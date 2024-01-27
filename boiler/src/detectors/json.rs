use crate::data::Repo;
use crate::detectors::{Detector, DetectorResult};
use crate::detectors_utils::detect_by_extension;

pub struct JsonDetector;

impl Detector for JsonDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        detect_by_extension(repo, &["json"], "json")
    }
}
