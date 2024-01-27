use std::collections::BTreeMap;

use ignore::Walk;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::DetectorResult;

pub fn detect_by_extension(repo: &Repo, extensions: &[&str], lang: &str) -> DetectorResult {
    for entry in Walk::new(repo.path()) {
        if let Some(ext) = entry?.path().extension() {
            if extensions.contains(&ext.to_ascii_lowercase().to_string_lossy().as_ref()) {
                let mut data = Value::new_object(BTreeMap::new());
                data.insert(context_keys::LANGS, vec![Value::new_string(lang)]);
                return Ok(data);
            }
        }
    }

    Ok(Value::new_object(BTreeMap::new()))
}
