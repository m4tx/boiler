use std::collections::BTreeMap;

use boiler_macros::FunctionMeta;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

#[derive(Debug, FunctionMeta)]
pub struct PythonDetector;

impl Detector for PythonDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::new_object(BTreeMap::new());

        let pyproject_toml = repo.path().join("pyproject.toml");
        let requirements_txt = repo.path().join("requirements.txt");
        if pyproject_toml.exists() {
            data.insert(context_keys::PYTHON_PACKAGE_MANAGERS, "poetry");
            data.insert(context_keys::LANGS, vec![Value::new_string("python")]);
        } else if requirements_txt.exists() {
            data.insert(context_keys::PYTHON_PACKAGE_MANAGERS, "pip");
            data.insert(context_keys::LANGS, vec![Value::new_string("python")]);
        }

        let manage_py = repo.path().join("manage.py");
        if manage_py.exists() {
            data.insert(context_keys::FRAMEWORKS, vec![Value::new_string("django")]);
        }

        Ok(data)
    }
}
