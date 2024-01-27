use std::collections::{BTreeMap, HashMap};

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
    let mut data = Value::new_object(BTreeMap::new());

    data.insert(crate::context_keys::LICENSE, "LicenseRef-proprietary");
    data.insert(
        crate::context_keys::GH_ACTIONS_RUST_VERSIONS,
        vec![Value::new_string("stable"), Value::new_string("nightly")],
    );
    data.insert(
        crate::context_keys::GH_ACTIONS_RUST_OS,
        vec![
            Value::new_string("ubuntu-latest"),
            Value::new_string("macos-latest"),
            Value::new_string("windows-latest"),
        ],
    );
    data.insert(crate::context_keys::GH_ACTIONS_RUST_FEATURES, vec![]);

    data
}
