use thiserror::Error;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

#[derive(Debug, Error)]
pub enum DockerDetectorError {
    #[error("failed to read directory")]
    ReadDir(#[from] std::io::Error),
}

/// Detects the existence of Dockerfiles.
#[derive(Debug, boiler_macros::FunctionMeta)]
pub struct DockerDetector;

fn dockerfile_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let a = a.to_lowercase();
    let b = b.to_lowercase();
    if a == "dockerfile" {
        return std::cmp::Ordering::Less;
    }
    if b == "dockerfile" {
        return std::cmp::Ordering::Greater;
    }

    a.replace(".dockerfile", "")
        .cmp(&b.replace(".dockerfile", ""))
}

impl Detector for DockerDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut dockerfiles = vec![];
        for entry in repo
            .path()
            .read_dir()
            .map_err(DockerDetectorError::ReadDir)?
        {
            let entry = entry.map_err(DockerDetectorError::ReadDir)?;

            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            let file_name_lower = file_name_str.to_lowercase();
            if file_name_lower == "dockerfile" || file_name_lower.ends_with(".dockerfile") {
                dockerfiles.push(file_name_str.to_string());
            }
        }
        dockerfiles.sort_by(|a, b| dockerfile_cmp(a, b));

        let mut data = Value::empty_object();
        if !dockerfiles.is_empty() {
            data.insert(context_keys::LANGS, vec![Value::new_string("docker")]);
            data.insert(
                context_keys::DOCKERFILES,
                Value::new_array(
                    dockerfiles
                        .iter()
                        .map(Value::new_string)
                        .collect::<Vec<Value>>(),
                ),
            );
        }

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::docker::DockerDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_docker() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("Dockerfile", "FROM nginx");
        temp_repo.write_str("test.dockerfile", "FROM nginx");

        let detector = DockerDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array(vec![Value::new_string("docker")])
                ),
                (
                    context_keys::DOCKERFILES.to_owned(),
                    Value::new_array(vec![
                        Value::new_string("Dockerfile"),
                        Value::new_string("test.dockerfile")
                    ])
                ),
            ])
        );
    }
}
