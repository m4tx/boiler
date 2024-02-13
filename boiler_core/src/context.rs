use std::collections::{BTreeSet, HashMap};

use serde::Deserialize;
use thiserror::Error;

use crate::actions::{Action, ActionName};
use crate::data::Value;
use crate::function_meta::FunctionEnabled;

#[derive(Debug, Deserialize)]
pub struct ReposConfig {
    repos: HashMap<String, RepoConfig>,
}

impl ReposConfig {
    #[must_use]
    pub fn from_yaml_string(yaml_string: &str) -> Self {
        let repos = serde_yaml::from_str(yaml_string).unwrap();
        Self { repos }
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&RepoConfig> {
        self.repos.get(key)
    }
}

#[derive(Debug, Error)]
pub enum RepoConfigError {
    #[error("Invalid action name: {0}")]
    InvalidActionName(ActionName),
}

#[derive(Debug, Deserialize)]
pub struct RepoConfig {
    #[serde(default)]
    actions_excluded: BTreeSet<ActionName>,
    #[serde(default = "Value::empty_object")]
    context: Value,
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            actions_excluded: BTreeSet::new(),
            context: Value::empty_object(),
        }
    }
}

impl RepoConfig {
    #[must_use]
    pub fn actions_excluded(&self) -> &BTreeSet<ActionName> {
        &self.actions_excluded
    }

    pub fn filter_actions<'a>(
        &self,
        actions: &[&'a dyn Action],
    ) -> Result<Vec<&'a dyn Action>, RepoConfigError> {
        for action_name in &self.actions_excluded {
            if !actions
                .iter()
                .any(|action| action.name() == action_name.name())
            {
                return Err(RepoConfigError::InvalidActionName(action_name.clone()));
            }
        }

        Ok(actions
            .iter()
            .filter(|action| {
                !self
                    .actions_excluded
                    .contains(&ActionName::new(action.name().to_string()))
            })
            .copied()
            .collect())
    }

    pub fn create_actions_enabled(
        &self,
        actions_enabled: &FunctionEnabled,
    ) -> Result<FunctionEnabled, RepoConfigError> {
        for action_excluded in &self.actions_excluded {
            if !actions_enabled
                .function_names()
                .any(|action_name| action_name == action_excluded.name())
            {
                return Err(RepoConfigError::InvalidActionName(action_excluded.clone()));
            }
        }

        let mut actions_enabled = actions_enabled.clone();
        for action_excluded in &self.actions_excluded {
            actions_enabled.set_enabled(action_excluded.name().to_owned(), false);
        }

        Ok(actions_enabled)
    }

    #[must_use]
    pub fn override_with(&mut self, other: &RepoConfig) -> Self {
        let actions_excluded = self
            .actions_excluded
            .union(&other.actions_excluded)
            .cloned()
            .collect();
        let context = {
            let mut context = self.context.clone();
            context.override_with(&other.context);
            context
        };

        Self {
            actions_excluded,
            context,
        }
    }

    #[must_use]
    pub fn context(&self) -> &Value {
        &self.context
    }
}

pub fn default_context_data() -> Value {
    let mut data = Value::empty_object();

    data.insert(crate::context_keys::CRATE_PUBLISHED, true);
    data.insert(crate::context_keys::COVERAGE_ENABLED, true);
    data.insert(crate::context_keys::LICENSE, "LicenseRef-proprietary");
    data.insert(crate::context_keys::LANGS, []);
    data.insert(crate::context_keys::FRAMEWORKS, []);
    data.insert(
        crate::context_keys::GH_ACTIONS_RUST_VERSIONS,
        [Value::new_string("stable"), Value::new_string("nightly")],
    );
    data.insert(
        crate::context_keys::GH_ACTIONS_RUST_OS,
        [
            Value::new_string("ubuntu-latest"),
            Value::new_string("macos-latest"),
            Value::new_string("windows-latest"),
        ],
    );
    data.insert(crate::context_keys::GH_ACTIONS_RUST_FEATURES, []);
    data.insert(crate::context_keys::TRUNK_CONFIGS, []);

    data
}
