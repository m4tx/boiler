use std::collections::BTreeMap;

use chrono::Datelike;
use gix::revision::walk::Info;
use gix::traverse::commit::ancestors::Error;
use regex::Regex;
use serde::Deserialize;

use crate::data::{Number, Repo, Value};

trait Detector {
    fn detect(&self, repo: &Repo) -> Value;
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Option<CargoPackage>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: Option<String>,
    authors: Option<Vec<String>>,
}

struct RustDetector;

impl Detector for RustDetector {
    fn detect(&self, repo: &Repo) -> Value {
        let mut data = Value::new_object(BTreeMap::new());

        let cargo_toml = repo.path().join("Cargo.toml");
        if cargo_toml.exists() {
            data.insert(
                "langs".to_string(),
                Value::new_array(vec![Value::new_string("rust".to_string())]),
            );

            let cargo_toml: CargoToml = toml::from_str(
                &std::fs::read_to_string(&cargo_toml).expect("could not read Cargo.toml"),
            )
            .expect("could not parse Cargo.toml");
            if let Some(package) = &cargo_toml.package {
                if let Some(name) = &package.name {
                    data.insert(
                        "crate_name".to_string(),
                        Value::new_string(name.to_string()),
                    );
                }
                if let Some(authors) = &package.authors {
                    if let Some(author) = authors.first() {
                        let full_name = author
                            .find('<')
                            .map(|index| &author[..index])
                            .unwrap_or(author)
                            .trim();
                        data.insert(
                            "full_name".to_string(),
                            Value::new_string(full_name.to_string()),
                        );
                    }
                }
            }
        }

        data
    }
}

struct PythonDetector;

impl Detector for PythonDetector {
    fn detect(&self, repo: &Repo) -> Value {
        let mut data = Value::new_object(BTreeMap::new());

        let pyproject_toml = repo.path().join("pyproject.toml");
        let requirements_txt = repo.path().join("requirements.txt");
        if pyproject_toml.exists() {
            data.insert(
                "python_package_manager".to_string(),
                Value::new_string("poetry".to_string()),
            );
            data.insert(
                "langs".to_string(),
                Value::new_array(vec![Value::new_string("python".to_string())]),
            );
        } else if requirements_txt.exists() {
            data.insert(
                "python_package_manager".to_string(),
                Value::new_string("pip".to_string()),
            );
            data.insert(
                "langs".to_string(),
                Value::new_array(vec![Value::new_string("python".to_string())]),
            );
        }

        let manage_py = repo.path().join("manage.py");
        if manage_py.exists() {
            data.insert(
                "frameworks".to_string(),
                Value::new_array(vec![Value::new_string("django".to_string())]),
            );
        }

        data
    }
}

struct DockerDetector;

impl Detector for DockerDetector {
    fn detect(&self, repo: &Repo) -> Value {
        let mut data = Value::new_object(BTreeMap::new());

        let dockerfile = repo.path().join("Dockerfile");
        if dockerfile.exists() {
            data.insert(
                "langs".to_string(),
                Value::new_array(vec![Value::new_string("docker".to_string())]),
            );
        }

        data
    }
}

struct LicenseDetector;

impl Detector for LicenseDetector {
    fn detect(&self, repo: &Repo) -> Value {
        let mut data = Value::new_object(BTreeMap::new());

        let license = repo.path().join("LICENSE");
        if license.exists() {
            let license_text =
                std::fs::read_to_string(&license).expect("could not read LICENSE.j2");

            if license_text.contains("MIT License") {
                data.insert("license".to_string(), Value::new_string("MIT".to_string()));
            } else if license_text.contains("GNU General Public License")
                && license_text.contains("Version 3")
            {
                data.insert(
                    "license".to_string(),
                    Value::new_string("GNU GPL v3".to_string()),
                );
            }
        }

        data
    }
}

struct GitDetector;

impl Detector for GitDetector {
    fn detect(&self, repo: &Repo) -> Value {
        let mut data = Value::new_object(BTreeMap::new());

        let git_dir = repo.path().join(".git");
        if git_dir.exists() {
            data.insert(
                "vcs".to_string(),
                Value::new_array(vec![Value::new_string("git".to_string())]),
            );

            let repository = gix::open(&git_dir).expect("could not open git repo");
            let rev_walk = repository
                .rev_walk([repository.head_id().expect("no head commit")])
                .sorting(gix::traverse::commit::Sorting::ByCommitTimeNewestFirst)
                .all()
                .expect("could not run rev_walk");
            let (founding_datetime, last_activity_datetime) = rev_walk.fold(
                (chrono::Utc::now(), chrono::DateTime::<chrono::Utc>::MIN_UTC),
                |(acc_min, acc_max), commit| {
                    let commit_time = Self::get_commit_time(commit);
                    (commit_time.min(acc_min), commit_time.max(acc_max))
                },
            );

            data.insert(
                "first_activity_year".to_string(),
                Value::new_number(Number::Integer(founding_datetime.year() as i64)),
            );
            data.insert(
                "last_activity_year".to_string(),
                Value::new_number(Number::Integer(last_activity_datetime.year() as i64)),
            );

            let remote = repository
                .find_default_remote(gix::remote::Direction::Fetch)
                .expect("no default remote")
                .expect("could not find default remote");
            let remote_url = remote
                .url(gix::remote::Direction::Fetch)
                .expect("could not get remote url");
            let remote_url_string = remote_url.to_bstring().to_string();
            data.insert(
                "repo_url".to_string(),
                Value::new_string(remote_url_string.clone()),
            );

            let repo_regex = Regex::new(r"^git@github\.com:(\S+)/(\S+).git$").unwrap();
            if let Some(captures) = repo_regex.captures(&remote_url_string) {
                let owner = captures.get(1).unwrap().as_str();
                let name = captures.get(2).unwrap().as_str();
                data.insert(
                    "repo_owner".to_string(),
                    Value::new_string(owner.to_string()),
                );
                data.insert("repo_name".to_string(), Value::new_string(name.to_string()));
            }
        }

        data
    }
}

impl GitDetector {
    fn get_commit_time(commit: Result<Info, Error>) -> chrono::DateTime<chrono::Utc> {
        let commit = commit.expect("could not get commit");
        let commit_time = commit.commit_time();

        chrono::DateTime::from_timestamp(commit_time, 0).unwrap()
    }
}

struct ReadmeDetector;

impl Detector for ReadmeDetector {
    fn detect(&self, repo: &Repo) -> Value {
        let mut data = Value::new_object(BTreeMap::new());

        let readme_path = repo.path().join("README.md");
        if readme_path.exists() {
            let readme = std::fs::read_to_string(readme_path).expect("could not read README.md");
            let name_regex = Regex::new(r"(?m)^(.+)\n=+$").unwrap();
            if let Some(captures) = name_regex.captures(&readme) {
                let name = captures.get(1).unwrap().as_str();
                data.insert("name".to_string(), Value::new_string(name.to_string()));
            }
        }

        data
    }
}

#[allow(non_snake_case)]
struct DetectorsEnabled {
    RustDetector: bool,
    PythonDetector: bool,
    DockerDetector: bool,
    LicenseDetector: bool,
    GitDetector: bool,
    ReadmeDetector: bool,
}

fn default_context_data() -> Value {
    let mut data = Value::new_object(BTreeMap::new());

    data.insert(
        "license".to_string(),
        Value::new_string("LicenseRef-proprietary".to_string()),
    );
    data.insert(
        "gh_actions_rust_versions".to_string(),
        Value::new_array(vec![
            Value::new_string("stable".to_string()),
            Value::new_string("nightly".to_string()),
        ]),
    );
    data.insert(
        "gh_actions_rust_os".to_string(),
        Value::new_array(vec![
            Value::new_string("ubuntu-latest".to_string()),
            Value::new_string("macos-latest".to_string()),
            Value::new_string("windows-latest".to_string()),
        ]),
    );
    data.insert(
        "gh_actions_rust_features".to_string(),
        Value::new_array(vec![Value::new_string("default".to_string())]),
    );

    data
}

fn detect(repo: &Repo, detectors: &DetectorsEnabled) -> Value {
    let mut data = default_context_data();

    if detectors.RustDetector {
        data.union(&RustDetector.detect(repo));
    }
    if detectors.PythonDetector {
        data.union(&PythonDetector.detect(repo));
    }
    if detectors.DockerDetector {
        data.union(&DockerDetector.detect(repo));
    }
    if detectors.LicenseDetector {
        data.union(&LicenseDetector.detect(repo));
    }
    if detectors.GitDetector {
        data.union(&GitDetector.detect(repo));
    }
    if detectors.ReadmeDetector {
        data.union(&ReadmeDetector.detect(repo));
    }

    data
}

pub fn detect_all(repo: &Repo) -> Value {
    let mut data = Value::new_object(BTreeMap::new());

    let detectors_enabled = DetectorsEnabled {
        RustDetector: true,
        PythonDetector: true,
        DockerDetector: true,
        LicenseDetector: true,
        GitDetector: true,
        ReadmeDetector: true,
    };
    data.union(&detect(repo, &detectors_enabled));

    data
}
