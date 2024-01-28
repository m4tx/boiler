use std::collections::BTreeMap;

use boiler_macros::FunctionMeta;
use regex::Regex;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

/// Detects the license of the project using the LICENSE file.
#[derive(Debug, FunctionMeta)]
pub struct LicenseDetector;

impl Detector for LicenseDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::new_object(BTreeMap::new());

        let license_file = repo.path().join("LICENSE");
        if license_file.exists() {
            let license_text =
                std::fs::read_to_string(&license_file).expect("could not read LICENSE");

            if let Some(license) = self.detect_license(&license_text) {
                data.insert(context_keys::LICENSE, license);
            }
            if let Some(name) = self.detect_name(&license_text) {
                data.insert(context_keys::FULL_NAME, name);
            }
        }

        Ok(data)
    }
}

impl LicenseDetector {
    fn detect_license(&self, license_text: &str) -> Option<&str> {
        if license_text.contains("MIT License") {
            Some("MIT")
        } else if license_text.contains("GNU General Public License")
            && license_text.contains("Version 3")
        {
            Some("GNU GPL v3")
        } else {
            None
        }
    }

    fn detect_name(&self, license_text: &str) -> Option<String> {
        let regex = Regex::new(r"(?m)MIT License\n\nCopyright \(c\) [0-9-]+ (.+)$").unwrap();

        if let Some(captures) = regex.captures(license_text) {
            let name = captures.get(1).unwrap().as_str();
            Some(name.to_string())
        } else {
            None
        }
    }
}
