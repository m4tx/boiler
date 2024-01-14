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
