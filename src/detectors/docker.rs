use std::collections::BTreeMap;

use thiserror::Error;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

#[derive(Debug, Error)]
pub enum DockerDetectorError {
    #[error("failed to read directory")]
    ReadDir(#[from] std::io::Error),
}

pub struct DockerDetector;

fn dockerfile_cmp(a: &String, b: &String) -> std::cmp::Ordering {
    let a = a.to_lowercase();
    let b = b.to_lowercase();
    if a == "dockerfile" {
        return std::cmp::Ordering::Less;
    }
    if b == "dockerfile" {
        return std::cmp::Ordering::Greater;
    }

    a.replace(".dockerfile", "")
        .cmp(&b.replace(".dockerfile", ""))
}

impl Detector for DockerDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut dockerfiles = vec![];
        for entry in repo
            .path()
            .read_dir()
            .map_err(DockerDetectorError::ReadDir)?
        {
            let entry = entry.map_err(DockerDetectorError::ReadDir)?;

            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            if file_name_str == "Dockerfile" || file_name_str.ends_with(".dockerfile") {
                dockerfiles.push(file_name_str.to_string());
            }
        }
        dockerfiles.sort_by(dockerfile_cmp);
        println!("dockerfiles: {:?}", dockerfiles);

        let mut data = Value::new_object(BTreeMap::new());
        if !dockerfiles.is_empty() {
            data.insert(context_keys::LANGS, vec![Value::new_string("docker")]);
            data.insert(
                context_keys::DOCKERFILES,
                Value::new_array(
                    dockerfiles
                        .iter()
                        .map(|s| Value::new_string(s))
                        .collect::<Vec<Value>>(),
                ),
            );
        }

        Ok(data)
    }
}
