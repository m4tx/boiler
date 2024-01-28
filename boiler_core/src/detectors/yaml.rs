use boiler_macros::FunctionMeta;

use crate::data::Repo;
use crate::detectors::{Detector, DetectorResult};
use crate::detectors_utils::detect_by_extension;

/// Detects if the project contains YAML files.
#[derive(Debug, FunctionMeta)]
pub struct YamlDetector;

impl Detector for YamlDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        detect_by_extension(repo, &["yaml", "yml"], "yaml")
    }
}

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::yaml::YamlDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_yaml() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.yaml", "test: true");

        let detector = YamlDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array(vec![Value::new_string("yaml")])
            ),])
        );
    }

    #[test]
    fn test_detect_yml() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.yml", "test: true");

        let detector = YamlDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array(vec![Value::new_string("yaml")])
            ),])
        );
    }
}
