use std::collections::BTreeMap;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

pub struct LicenseDetector;

impl Detector for LicenseDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::new_object(BTreeMap::new());

        let license = repo.path().join("LICENSE");
        if license.exists() {
            let license_text = std::fs::read_to_string(&license).expect("could not read LICENSE");

            if license_text.contains("MIT License") {
                data.insert(context_keys::LICENSE, "MIT");
            } else if license_text.contains("GNU General Public License")
                && license_text.contains("Version 3")
            {
                data.insert(context_keys::LICENSE, "GNU GPL v3");
            }
        }

        Ok(data)
    }
}
