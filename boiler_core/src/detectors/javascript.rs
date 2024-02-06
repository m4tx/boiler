use boiler_macros::FunctionMeta;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};
use crate::detectors_utils::detect_by_extension;

/// Detects if the project contains JavaScript or TypeScript files.
#[derive(Debug, FunctionMeta)]
pub struct JavascriptDetector;

impl Detector for JavascriptDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::empty_object();
        data.union(&detect_by_extension(
            repo,
            &["js", "ts", "jsx", "tsx"],
            "javascript",
        )?)
        .expect("could not merge context data");
        data.union(&detect_by_extension(repo, &["ts", "tsx"], "typescript")?)
            .expect("could not merge context data");
        data.union(&detect_by_extension(repo, &["jsx", "tsx"], "jsx")?)
            .expect("could not merge context data");
        data.union(&detect_by_extension(repo, &["tsx"], "tsx")?)
            .expect("could not merge context data");
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::javascript::JavascriptDetector;
    use crate::detectors::json::JsonDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_js() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.js", "console.log('Hello, world!');");

        let detector = JavascriptDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array([Value::new_string("javascript")])
            ),])
        );
    }

    #[test]
    fn test_detect_ts() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.ts", "console.log('Hello, world!');");

        let detector = JavascriptDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array([
                    Value::new_string("javascript"),
                    Value::new_string("typescript")
                ])
            ),])
        );
    }

    #[test]
    fn test_detect_jsx() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.jsx", "console.log('Hello, world!');");

        let detector = JavascriptDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array([Value::new_string("javascript"), Value::new_string("jsx")])
            ),])
        );
    }

    #[test]
    fn test_detect_tsx() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.tsx", "console.log('Hello, world!');");

        let detector = JavascriptDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array([
                    Value::new_string("javascript"),
                    Value::new_string("typescript"),
                    Value::new_string("jsx"),
                    Value::new_string("tsx")
                ])
            ),])
        );
    }
}
