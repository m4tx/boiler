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

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::toml::TomlDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_toml() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("rustfmt.toml", "unstable_features = true");

        let detector = TomlDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array(vec![Value::new_string("toml")])
            ),])
        );
    }
}
