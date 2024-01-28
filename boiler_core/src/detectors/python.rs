use std::collections::BTreeMap;

use boiler_macros::FunctionMeta;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

/// Detects if the project contains Python files and checks which package
/// manager the project is using.
#[derive(Debug, FunctionMeta)]
pub struct PythonDetector;

impl Detector for PythonDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::new_object(BTreeMap::new());

        let mut package_managers = Vec::new();

        let pyproject_toml = repo.path().join("pyproject.toml");
        let requirements_txt = repo.path().join("requirements.txt");
        if pyproject_toml.exists() {
            package_managers.push(Value::new_string("poetry"));
        } else if requirements_txt.exists() {
            package_managers.push(Value::new_string("pip"));
        }
        if !package_managers.is_empty() {
            data.insert(context_keys::PYTHON_PACKAGE_MANAGERS, package_managers);
            data.insert(context_keys::LANGS, vec![Value::new_string("python")]);
        }

        let mut frameworks = Vec::new();

        let manage_py = repo.path().join("manage.py");
        if manage_py.exists() {
            frameworks.push(Value::new_string("django"));
        }
        if !frameworks.is_empty() {
            data.insert(context_keys::FRAMEWORKS, frameworks);
        }

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::python::PythonDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_python_requirements() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("requirements.txt", "django=*");

        let detector = PythonDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array(vec![Value::new_string("python")])
                ),
                (
                    context_keys::PYTHON_PACKAGE_MANAGERS.to_owned(),
                    Value::new_array(vec![Value::new_string("pip")])
                ),
            ])
        );
    }

    #[test]
    fn test_detect_python_framework_django() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("requirements.txt", "django=*");
        temp_repo.write_str(
            "manage.py",
            "#!/usr/bin/env python\nprint('Hello, world!')\n",
        );

        let detector = PythonDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array(vec![Value::new_string("python")])
                ),
                (
                    context_keys::PYTHON_PACKAGE_MANAGERS.to_owned(),
                    Value::new_array(vec![Value::new_string("pip")])
                ),
                (
                    context_keys::FRAMEWORKS.to_owned(),
                    Value::new_array(vec![Value::new_string("django")])
                ),
            ])
        );
    }

    #[test]
    fn test_detect_python_poetry() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("pyproject.toml", "[tool.poetry]\nname = \"test\"");

        let detector = PythonDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::LANGS.to_owned(),
                    Value::new_array(vec![Value::new_string("python")])
                ),
                (
                    context_keys::PYTHON_PACKAGE_MANAGERS.to_owned(),
                    Value::new_array(vec![Value::new_string("poetry")])
                ),
            ])
        );
    }
}
