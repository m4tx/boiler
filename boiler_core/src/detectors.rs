use anyhow::Context;
use chrono::Utc;
use docker::DockerDetector;
use git::GitDetector;
use license::LicenseDetector;
use log::debug;
use once_cell::sync::Lazy;
use python::PythonDetector;
use readme::ReadmeDetector;
use rust::RustDetector;

use crate::context::default_context_data;
use crate::data::{Repo, Value};
use crate::detectors::javascript::JavascriptDetector;
use crate::detectors::json::JsonDetector;
use crate::detectors::shell_script::ShellScriptDetector;
use crate::detectors::toml::TomlDetector;
use crate::detectors::yaml::YamlDetector;
use crate::function_meta::{FunctionEnabled, FunctionMeta};

mod docker;
mod git;
mod javascript;
mod json;
mod license;
mod python;
mod readme;
mod rust;
mod shell_script;
mod toml;
mod yaml;

pub(crate) type DetectorResult = anyhow::Result<Value>;

pub trait Detector: FunctionMeta + Send + Sync {
    fn detect(&self, repo: &Repo) -> DetectorResult;
}

pub static DETECTORS: Lazy<[Box<dyn Detector>; 11]> = Lazy::new(|| {
    [
        Box::new(DockerDetector),
        Box::new(GitDetector::new(Utc)),
        Box::new(JavascriptDetector),
        Box::new(JsonDetector),
        Box::new(LicenseDetector),
        Box::new(PythonDetector),
        Box::new(ReadmeDetector),
        Box::new(RustDetector),
        Box::new(ShellScriptDetector),
        Box::new(TomlDetector),
        Box::new(YamlDetector),
    ]
});

pub fn detect(repo: &Repo, detectors_enabled: &FunctionEnabled) -> DetectorResult {
    let mut data = Value::empty_object();

    for detector in DETECTORS.iter() {
        if detectors_enabled.is_enabled(detector.name()) {
            let detector_result = detector
                .detect(repo)
                .with_context(|| format!("Failed to run detector: {}", detector.name()))?;
            data.union(&detector_result).with_context(|| {
                format!("Failed to combine detector result: {}", detector.name())
            })?;
        }
    }

    Ok(data)
}

pub fn detect_with_defaults(repo: &Repo, detectors_enabled: &FunctionEnabled) -> DetectorResult {
    let data = detect(repo, detectors_enabled)?;

    let mut data_with_defaults = default_context_data();
    data_with_defaults.override_with(&data);

    Ok(data_with_defaults)
}

pub fn create_detectors_enabled() -> FunctionEnabled {
    let mut detectors_enabled = FunctionEnabled::new();

    for detector in DETECTORS.iter() {
        debug!("Running detector: {}", detector.name());
        detectors_enabled.set_enabled(detector.name().to_owned(), detector.default_enabled());
    }

    detectors_enabled
}

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::json::JsonDetector;
    use crate::detectors::{create_detectors_enabled, detect, detect_with_defaults};
    use crate::function_meta::FunctionMeta;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_none_enabled() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.json", "{}");

        let mut detectors_enabled = create_detectors_enabled();
        for (_fn_name, enabled) in detectors_enabled.iter_mut() {
            *enabled = false;
        }

        let data = detect(&temp_repo.repo(), &detectors_enabled).unwrap();
        assert_eq!(data, Value::empty_object());
    }

    #[test]
    fn test_detect_with_defaults() {
        let temp_repo = TempRepo::new();

        let mut detectors_enabled = create_detectors_enabled();
        for (_fn_name, enabled) in detectors_enabled.iter_mut() {
            *enabled = false;
        }

        let data = detect_with_defaults(&temp_repo.repo(), &detectors_enabled).unwrap();
        assert_ne!(data, Value::empty_object());
    }

    #[test]
    fn test_detect_json_enabled() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("test.json", "{}");

        let mut detectors_enabled = create_detectors_enabled();
        for (_fn_name, enabled) in detectors_enabled.iter_mut() {
            *enabled = false;
        }
        detectors_enabled.set_enabled(JsonDetector.name().to_owned(), true);

        let data = detect(&temp_repo.repo(), &detectors_enabled).unwrap();
        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array(vec![Value::new_string("json")])
            ),])
        );
    }
}
