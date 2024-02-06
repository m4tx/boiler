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
        let mut data = Value::empty_object();

        let readme_path = repo.path().join("README.md");
        if readme_path.exists() {
            let readme = std::fs::read_to_string(readme_path).expect("could not read README.md");
            let name_regex_setext = Regex::new(r"(?m)^(.+)\n=+$").unwrap();
            let name_regex_header = Regex::new(r"(?m)^# (.+)$").unwrap();

            if let Some(captures) = name_regex_setext
                .captures(&readme)
                .or(name_regex_header.captures(&readme))
            {
                let name = captures.get(1).unwrap().as_str();
                data.insert(context_keys::NAME, name);
            }
        }

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::readme::ReadmeDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_name() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("README.md", "Project Name\n============");

        let detector = ReadmeDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::NAME.to_owned(),
                Value::new_string("Project Name")
            ),])
        );
    }

    #[test]
    fn test_detect_name_alternate_markdown() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("README.md", "# Project Name");

        let detector = ReadmeDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::NAME.to_owned(),
                Value::new_string("Project Name")
            ),])
        );
    }
}
