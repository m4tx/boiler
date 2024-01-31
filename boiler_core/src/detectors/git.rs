use std::collections::BTreeMap;

use anyhow::Context;
use boiler_macros::FunctionMeta;
use chrono::{DateTime, Datelike, Utc};
use gix::revision::walk::Info;
use gix::traverse::commit::ancestors::Error;
use gix::{Repository, Url};
use log::warn;
use regex::Regex;

use crate::context_keys;
use crate::data::{Repo, Value};
use crate::detectors::{Detector, DetectorResult};
use crate::time::Clock;

/// Detects if the project is using git as the VCS and detects basic metadata,
/// such as repository owner/name and the activity period.
#[derive(Debug, FunctionMeta)]
pub struct GitDetector<C = Utc> {
    clock: C,
}

impl<C: Clock + Send + Sync> Detector for GitDetector<C> {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        let mut data = Value::new_object(BTreeMap::new());

        let git_dir = repo.path().join(".git");
        if git_dir.exists() {
            data.insert(context_keys::VCS, vec![Value::new_string("git")]);

            let repository = gix::open(&git_dir).with_context(|| "Could not open git repo")?;
            let (founding_datetime, last_activity_datetime) =
                self.retrieve_activity_timespan(&repository)?;
            data.insert(context_keys::FIRST_ACTIVITY_YEAR, founding_datetime.year());
            data.insert(
                context_keys::LAST_ACTIVITY_YEAR,
                last_activity_datetime.year(),
            );

            let owner_name = Self::retrieve_owner_and_name(repository)?;
            if let Some((owner, name)) = owner_name {
                data.insert(context_keys::REPO_OWNER, owner);
                data.insert(context_keys::REPO_NAME, name);
            }

            data.insert(context_keys::GIT_HAS_SUBMODULES, self.has_submodules(repo));
        }

        Ok(data)
    }
}

impl<C: Clock + Send + Sync> GitDetector<C> {
    #[must_use]
    pub const fn new(clock: C) -> Self {
        Self { clock }
    }

    fn retrieve_activity_timespan(
        &self,
        repository: &Repository,
    ) -> anyhow::Result<(DateTime<Utc>, DateTime<Utc>)> {
        let head_commit = repository.head_id();
        if let Ok(head_commit) = head_commit {
            let rev_walk = repository
                .rev_walk([head_commit])
                .sorting(gix::traverse::commit::Sorting::ByCommitTimeNewestFirst)
                .all()
                .with_context(|| "Could not run rev_walk")?;
            let commit_times = rev_walk
                .map(Self::get_commit_time)
                .collect::<anyhow::Result<Vec<_>>>()?;
            let (founding_datetime, last_activity_datetime) = commit_times.iter().fold(
                (self.clock.now(), DateTime::<Utc>::MIN_UTC),
                |(acc_min, acc_max), &commit_time| {
                    (commit_time.min(acc_min), commit_time.max(acc_max))
                },
            );

            Ok((founding_datetime, last_activity_datetime))
        } else {
            warn!("Could not get head commit, using current time as fallback");
            let now = self.clock.now();
            Ok((now, now))
        }
    }

    fn get_commit_time(commit: Result<Info, Error>) -> anyhow::Result<DateTime<Utc>> {
        let commit = commit.with_context(|| "Could not get commit")?;
        let commit_time = commit.commit_time();

        Ok(
            chrono::DateTime::from_timestamp(commit_time, 0)
                .expect("Could not convert commit time"),
        )
    }

    fn retrieve_owner_and_name(repository: Repository) -> anyhow::Result<Option<(String, String)>> {
        let remote = repository.find_default_remote(gix::remote::Direction::Fetch);

        if let Some(remote) = remote {
            let remote = remote.with_context(|| "Could not find default remote")?;
            let remote_url = remote.url(gix::remote::Direction::Fetch);
            if let Some(remote_url) = remote_url {
                Self::parse_remote_url(remote_url)
            } else {
                warn!("No fetch URL set; could not retrieve owner and repo name");
                Ok(None)
            }
        } else {
            warn!("No default remote set; could not retrieve owner and repo name");
            Ok(None)
        }
    }

    fn parse_remote_url(remote_url: &Url) -> anyhow::Result<Option<(String, String)>> {
        let remote_url_string = remote_url.to_bstring().to_string();

        let repo_regex = Regex::new(r"^git@github\.com:(\S+)/(\S+).git$").unwrap();

        if let Some(captures) = repo_regex.captures(&remote_url_string) {
            let owner = captures.get(1).unwrap().as_str();
            let name = captures.get(2).unwrap().as_str();
            Ok(Some((owner.to_string(), name.to_string())))
        } else {
            warn!("Could not parse remote URL: {}", remote_url_string);
            Ok(None)
        }
    }

    fn has_submodules(&self, repo: &Repo) -> bool {
        let submodule_file = repo.path().join(".gitmodules");
        submodule_file.exists()
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::process::Command;

    use chrono::{NaiveDate, TimeZone, Utc};

    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::git::GitDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;
    use crate::time::MockClock;

    #[test]
    fn test_detect_git_empty() {
        let temp_repo = TempRepo::new();

        let detector = create_detector();
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(data, Value::new_object([]));
    }

    #[test]
    fn test_detect_git_repo() {
        let temp_repo = TempRepo::new();
        run_git(&["init"], &temp_repo).unwrap();

        let detector = create_detector();
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::VCS.to_owned(),
                    Value::new_array(vec![Value::new_string("git")])
                ),
                (
                    context_keys::FIRST_ACTIVITY_YEAR.to_owned(),
                    Value::new_number(2023)
                ),
                (
                    context_keys::LAST_ACTIVITY_YEAR.to_owned(),
                    Value::new_number(2023)
                ),
                (
                    context_keys::GIT_HAS_SUBMODULES.to_owned(),
                    Value::new_bool(false)
                )
            ])
        );
    }

    #[test]
    fn test_detect_git_repo_with_content() {
        let temp_repo = TempRepo::new();
        {
            run_git(&["init"], &temp_repo).unwrap();
            run_git(
                &["remote", "add", "origin", "git@github.com:m4tx/boiler.git"],
                &temp_repo,
            )
            .unwrap();
            temp_repo.write_str("README.md", "hello world");
            run_git(&["add", "README.md"], &temp_repo).unwrap();
            temp_repo.write_str(
                ".gitmodules",
                r#"[submodule "libfoo"]
	path = include/foo
	url = git://foo.com/git/lib.git"#,
            );

            let envs = [
                ("GIT_CONFIG_GLOBAL", "/dev/null"),
                ("GIT_AUTHOR_NAME", "A U Thor"),
                ("GIT_AUTHOR_EMAIL", "author@example.com"),
                ("GIT_AUTHOR_DATE", "2022-11-02T12:00:00+00:00"),
                ("GIT_COMMITTER_NAME", "A U Thor"),
                ("GIT_COMMITTER_EMAIL", "author@example.com"),
                ("GIT_COMMITTER_DATE", "2022-11-02T12:00:00+00:00"),
            ];
            run_git_with_envs(&["commit", "-m", "init"], &envs, &temp_repo).unwrap();
        }

        let detector = create_detector();
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([
                (
                    context_keys::VCS.to_owned(),
                    Value::new_array(vec![Value::new_string("git")])
                ),
                (
                    context_keys::REPO_OWNER.to_owned(),
                    Value::new_string("m4tx")
                ),
                (
                    context_keys::REPO_NAME.to_owned(),
                    Value::new_string("boiler")
                ),
                (
                    context_keys::FIRST_ACTIVITY_YEAR.to_owned(),
                    Value::new_number(2022)
                ),
                (
                    context_keys::LAST_ACTIVITY_YEAR.to_owned(),
                    Value::new_number(2022)
                ),
                (
                    context_keys::GIT_HAS_SUBMODULES.to_owned(),
                    Value::new_bool(true)
                )
            ])
        );
    }

    fn create_detector() -> GitDetector<MockClock> {
        let datetime = Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(2023, 11, 2)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
        );

        GitDetector::new(MockClock::new(datetime))
    }

    fn run_git(args: &[&str], repo: &TempRepo) -> anyhow::Result<()> {
        run_git_with_envs(args, &[], repo)
    }

    fn run_git_with_envs(
        args: &[&str],
        envs: &[(&str, &str)],
        repo: &TempRepo,
    ) -> anyhow::Result<()> {
        let output = Command::new("git")
            .args(args)
            .envs(
                envs.iter()
                    .map(|(k, v)| (OsString::from(*k), OsString::from(*v))),
            )
            .current_dir(repo.repo().path())
            .output()
            .unwrap();

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "git command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}
