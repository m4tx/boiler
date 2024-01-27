use std::collections::BTreeMap;

use anyhow::Context;
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
use crate::detectors::json::JsonDetector;
use crate::detectors::shell_script::ShellScriptDetector;
use crate::detectors::toml::TomlDetector;
use crate::detectors::yaml::YamlDetector;
use crate::function_meta::{FunctionEnabled, FunctionMeta};

mod docker;
mod git;
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

pub static DETECTORS: Lazy<[&dyn Detector; 10]> = Lazy::new(|| {
    [
        &DockerDetector,
        &GitDetector,
        &JsonDetector,
        &LicenseDetector,
        &PythonDetector,
        &ReadmeDetector,
        &RustDetector,
        &ShellScriptDetector,
        &TomlDetector,
        &YamlDetector,
    ]
});

fn detect(repo: &Repo, detectors_enabled: &FunctionEnabled) -> DetectorResult {
    let mut data = Value::new_object(BTreeMap::new());

    for detector in *DETECTORS {
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

pub fn detect_all(repo: &Repo) -> DetectorResult {
    let detectors_enabled = create_detectors_enabled();
    let data = detect(repo, &detectors_enabled)?;

    let mut data_with_defaults = default_context_data();
    data_with_defaults.override_with(&data);

    Ok(data_with_defaults)
}

fn create_detectors_enabled() -> FunctionEnabled {
    let mut detectors_enabled = FunctionEnabled::new();

    for detector in DETECTORS.iter() {
        debug!("Running detector: {}", detector.name());
        detectors_enabled.add(detector.name().to_owned(), detector.default_enabled());
    }

    detectors_enabled
}
