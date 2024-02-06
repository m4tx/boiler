use std::collections::HashMap;

use crate::data::Value;

#[derive(Debug)]
pub struct ContextOverrides {
    overrides: HashMap<String, Value>,
}

impl ContextOverrides {
    #[must_use]
    pub fn from_yaml_string(yaml_string: &str) -> Self {
        let overrides = serde_yaml::from_str(yaml_string).unwrap();
        Self { overrides }
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.overrides.get(key)
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
