use std::path::PathBuf;

use log::debug;

use crate::data::Repo;

#[derive(Debug, thiserror::Error)]
pub enum ActionIoError {
    #[error("Could not create directory: {path}")]
    CreateDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Could not write file: {path}")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

impl ActionIoError {
    #[must_use]
    fn new_create_dir(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::CreateDir {
            path: path.into(),
            source,
        }
    }

    #[must_use]
    fn new_write_file(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::WriteFile {
            path: path.into(),
            source,
        }
    }
}

pub fn write_file<T: Into<PathBuf>>(
    repo: &Repo,
    path: T,
    content: &str,
) -> Result<(), ActionIoError> {
    let full_path = repo.path().join(path.into());
    let parent_dir = full_path.parent().expect("file path has no parent");

    std::fs::create_dir_all(parent_dir)
        .map_err(|e| ActionIoError::new_create_dir(parent_dir, e))?;
    if let Ok(old_content) = std::fs::read_to_string(&full_path) {
        if old_content == content {
            debug!("File {} unchanged", full_path.display());
            return Ok(());
        }
    }

    debug!("Writing {} bytes to {}", content.len(), full_path.display());
    std::fs::write(&full_path, content).map_err(|e| ActionIoError::new_write_file(full_path, e))?;

    Ok(())
}
