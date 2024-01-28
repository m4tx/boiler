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

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::json::JsonDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_json() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.json", "{}");

        let detector = JsonDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array(vec![Value::new_string("json")])
            ),])
        );
    }
}
