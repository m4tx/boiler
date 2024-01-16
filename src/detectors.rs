use std::collections::BTreeMap;

use docker::DockerDetector;
use git::GitDetector;
use license::LicenseDetector;
use python::PythonDetector;
use readme::ReadmeDetector;
use rust::RustDetector;

use crate::context::default_context_data;
use crate::data::{Repo, Value};

mod docker;
mod git;
mod license;
mod python;
mod readme;
mod rust;

type DetectorResult = anyhow::Result<Value>;

trait Detector {
    fn detect(&self, repo: &Repo) -> DetectorResult;
}

#[allow(non_snake_case)]
struct DetectorsEnabled {
    RustDetector: bool,
    PythonDetector: bool,
    DockerDetector: bool,
    LicenseDetector: bool,
    GitDetector: bool,
    ReadmeDetector: bool,
}

fn detect(repo: &Repo, detectors: &DetectorsEnabled) -> DetectorResult {
    let mut data = default_context_data();

    if detectors.RustDetector {
        data.union(&RustDetector.detect(repo)?);
    }
    if detectors.PythonDetector {
        data.union(&PythonDetector.detect(repo)?);
    }
    if detectors.DockerDetector {
        data.union(&DockerDetector.detect(repo)?);
    }
    if detectors.LicenseDetector {
        data.union(&LicenseDetector.detect(repo)?);
    }
    if detectors.GitDetector {
        data.union(&GitDetector.detect(repo)?);
    }
    if detectors.ReadmeDetector {
        data.union(&ReadmeDetector.detect(repo)?);
    }

    Ok(data)
}

pub fn detect_all(repo: &Repo) -> DetectorResult {
    let mut data = Value::new_object(BTreeMap::new());

    let detectors_enabled = DetectorsEnabled {
        RustDetector: true,
        PythonDetector: true,
        DockerDetector: true,
        LicenseDetector: true,
        GitDetector: true,
        ReadmeDetector: true,
    };
    data.union(&detect(repo, &detectors_enabled)?);

    Ok(data)
}
