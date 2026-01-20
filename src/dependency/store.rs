use crate::dependency::Dependency;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Dependency key not found: {0}")]
    DependencyNotFound(String),
}

pub struct DependencyStore {
    dependencies: HashMap<String, Box<dyn Dependency>>,
}

impl Default for DependencyStore {
    fn default() -> Self {
        Self {
            dependencies: HashMap::new(),
        }
    }
}

impl DependencyStore {
    pub fn register(&mut self, key: &str, dependency: Box<dyn Dependency>) {
        self.dependencies.insert(key.to_string(), dependency);
    }

    pub fn get(&self, key: &str) -> Result<&Box<dyn Dependency>, Error> {
        self.dependencies
            .iter()
            .find(|(needle, value)| *needle == key)
            .ok_or_else(|| Error::DependencyNotFound(key.to_string()))
            .map(|(_, dep)| dep)
    }
}
