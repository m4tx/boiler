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
        let mut data = Value::empty_object();

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
        const GPL_HEADER_LENGTH: usize = 1024;

        let text_lower = license_text.to_lowercase();
        if text_lower.contains("mit license") {
            Some("MIT")
        } else if text_lower[..GPL_HEADER_LENGTH].contains("gnu affero general public license")
            && text_lower.contains("version 3")
        {
            Some("GNU AGPL v3")
        } else if text_lower[..GPL_HEADER_LENGTH].contains("gnu general public license")
            && text_lower.contains("version 3")
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

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::license::LicenseDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_license_mit() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str(
            "LICENSE",
            r#"MIT License

Copyright (c) 2024 John Paul

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#,
        );

        let detector = LicenseDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (context_keys::LICENSE.to_owned(), Value::new_string("MIT")),
                (
                    context_keys::FULL_NAME.to_owned(),
                    Value::new_string("John Paul")
                )
            ])
        );
    }
    #[test]
    fn test_detect_license_gpl_v3() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("LICENSE", include_str!("testdata/gpl_v3_license.txt"));

        let detector = LicenseDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LICENSE.to_owned(),
                Value::new_string("GNU GPL v3")
            )])
        );
    }
    #[test]
    fn test_detect_license_agpl_v3() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str(
            "LICENSE",
            r#"                    GNU AFFERO GENERAL PUBLIC LICENSE
                       Version 3, 19 November 2007

 Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/>
 Everyone is permitted to copy and distribute verbatim copies
 of this license document, but changing it is not allowed.
"#,
        );

        let detector = LicenseDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LICENSE.to_owned(),
                Value::new_string("GNU AGPL v3")
            )])
        );
    }
}
