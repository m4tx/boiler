use std::collections::BTreeMap;

pub trait FunctionMeta {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn default_enabled(&self) -> bool;
}

impl<T: FunctionMeta> FunctionMeta for &T {
    fn name(&self) -> &'static str {
        (*self).name()
    }

    fn description(&self) -> &'static str {
        (*self).description()
    }

    fn default_enabled(&self) -> bool {
        (*self).default_enabled()
    }
}

#[derive(Debug, Clone)]
pub struct FunctionEnabled {
    enabled_map: BTreeMap<String, bool>,
}

impl Default for FunctionEnabled {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionEnabled {
    pub fn new() -> Self {
        Self {
            enabled_map: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, function_name: String, enabled: bool) {
        self.enabled_map.insert(function_name, enabled);
    }

    pub fn is_enabled(&self, function_name: &str) -> bool {
        *self
            .enabled_map
            .get(function_name)
            .expect("Function not found")
    }

    pub fn function_names(&self) -> impl Iterator<Item = &String> {
        self.enabled_map.keys()
    }
}
