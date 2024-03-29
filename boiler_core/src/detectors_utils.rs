use std::io::Read;
use std::path::Path;

use ignore::Walk;

use crate::actions::ActionData;
use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::DetectorResult;

pub trait ActionDataExt {
    fn has_lang(&self, lang: &str) -> bool;
}

impl ActionDataExt for ActionData {
    fn has_lang(&self, lang: &str) -> bool {
        self.context
            .as_object()
            .expect("context is not an object")
            .get(context_keys::LANGS)
            .unwrap_or(&Value::new_array([]))
            .as_array()
            .expect("langs is not an array")
            .contains(&Value::new_string(lang))
    }
}

pub fn detect_by_extension(repo: &Repo, extensions: &[&str], lang: &str) -> DetectorResult {
    detect_by_predicate(repo, lang, |path| {
        if let Some(ext) = path.extension() {
            if extensions.contains(&ext.to_ascii_lowercase().to_string_lossy().as_ref()) {
                return Ok(true);
            }
        }
        Ok(false)
    })
}

pub fn detect_by_header(repo: &Repo, headers: &[&[u8]], lang: &str) -> DetectorResult {
    const BUFFER_SIZE: usize = 256;
    let mut buf = [0; BUFFER_SIZE];

    detect_by_predicate(repo, lang, |path| {
        let mut file = std::fs::File::open(path)?;
        let _ = file.read(&mut buf)?;
        for header in headers {
            if buf.starts_with(header) {
                return Ok(true);
            }
        }
        Ok(false)
    })
}

fn detect_by_predicate(
    repo: &Repo,
    lang: &str,
    mut predicate: impl FnMut(&Path) -> std::io::Result<bool>,
) -> DetectorResult {
    for entry in Walk::new(repo.path()) {
        let path = entry?.path().to_owned();
        if path.is_file() && predicate(&path)? {
            let mut data = Value::empty_object();
            data.insert(context_keys::LANGS, vec![Value::new_string(lang)]);
            return Ok(data);
        }
    }

    Ok(Value::empty_object())
}
