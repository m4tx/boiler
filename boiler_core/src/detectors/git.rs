use std::collections::BTreeMap;

use boiler_macros::FunctionMeta;
use chrono::Datelike;
use gix::revision::walk::Info;
use gix::traverse::commit::ancestors::Error;
use regex::Regex;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};

/// Detects if the project is using git as the VCS and detects basic metadata,
/// such as repository owner/name and the activity period.
#[derive(Debug, FunctionMeta)]
pub struct GitDetector;

impl Detector for GitDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::new_object(BTreeMap::new());

        let git_dir = repo.path().join(".git");
        if git_dir.exists() {
            data.insert(context_keys::VCS, vec![Value::new_string("git")]);

            let repository = gix::open(&git_dir).expect("Could not open git repo");
            let rev_walk = repository
                .rev_walk([repository.head_id().expect("No head commit")])
                .sorting(gix::traverse::commit::Sorting::ByCommitTimeNewestFirst)
                .all()
                .expect("Could not run rev_walk");
            let (founding_datetime, last_activity_datetime) = rev_walk.fold(
                (chrono::Utc::now(), chrono::DateTime::<chrono::Utc>::MIN_UTC),
                |(acc_min, acc_max), commit| {
                    let commit_time = Self::get_commit_time(commit);
                    (commit_time.min(acc_min), commit_time.max(acc_max))
                },
            );

            data.insert(context_keys::FIRST_ACTIVITY_YEAR, founding_datetime.year());
            data.insert(
                context_keys::LAST_ACTIVITY_YEAR,
                last_activity_datetime.year(),
            );

            let remote = repository
                .find_default_remote(gix::remote::Direction::Fetch)
                .expect("No default remote")
                .expect("Could not find default remote");
            let remote_url = remote
                .url(gix::remote::Direction::Fetch)
                .expect("Could not get remote url");
            let remote_url_string = remote_url.to_bstring().to_string();
            data.insert(context_keys::REPO_URL, remote_url_string.clone());

            let repo_regex = Regex::new(r"^git@github\.com:(\S+)/(\S+).git$").unwrap();
            if let Some(captures) = repo_regex.captures(&remote_url_string) {
                let owner = captures.get(1).unwrap().as_str();
                let name = captures.get(2).unwrap().as_str();
                data.insert(context_keys::REPO_OWNER, owner);
                data.insert(context_keys::REPO_NAME, name);
            }
        }

        Ok(data)
    }
}

impl GitDetector {
    fn get_commit_time(commit: Result<Info, Error>) -> chrono::DateTime<chrono::Utc> {
        let commit = commit.expect("Could not get commit");
        let commit_time = commit.commit_time();

        chrono::DateTime::from_timestamp(commit_time, 0).unwrap()
    }
}
