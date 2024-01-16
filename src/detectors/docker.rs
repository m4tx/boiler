use std::collections::BTreeMap;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

pub struct DockerDetector;

impl Detector for DockerDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::new_object(BTreeMap::new());

        let dockerfile = repo.path().join("Dockerfile");
        if dockerfile.exists() {
            data.insert(context_keys::LANGS, vec![Value::new_string("docker")]);
        }

        Ok(data)
    }
}
