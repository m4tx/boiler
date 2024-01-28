use std::fs;
use std::path::Path;

use assert_fs::fixture::{FileWriteStr, PathChild};
use assert_fs::TempDir;

use crate::data::Repo;

#[derive(Debug)]
pub struct TempRepo {
    temp_dir: TempDir,
}

impl TempRepo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().unwrap(),
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    #[must_use]
    pub fn repo(&self) -> Repo {
        Repo::new(self.path())
    }

    pub fn write_str(&self, path: &str, content: &str) {
        self.temp_dir.child(path).write_str(content).unwrap();
    }

    pub fn is_empty(&self) -> bool {
        for entry in self.path().read_dir().unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                return false;
            }
        }
        true
    }

    pub fn file_not_empty(&self, path: &str) -> bool {
        let file = self.temp_dir.child(path);
        file.exists() && fs::metadata(file.path()).unwrap().len() > 0
    }
}
