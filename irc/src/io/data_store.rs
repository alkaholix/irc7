//! JSON data store for configuration

use std::collections::HashMap;
use std::path::Path;

use async_trait::async_trait;
use parking_lot::RwLock;
use serde_json::Value;

use crate::traits::DataStore as DataStoreTrait;

/// JSON-based data store
pub struct DataStore {
    data: RwLock<HashMap<String, Value>>,
}

impl Default for DataStore {
    fn default() -> Self {
        Self::new()
    }
}

impl DataStore {
    /// Create a new empty data store
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    /// Create from a JSON file
    pub async fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let store = Self::new();
        if Path::new(path).exists() {
            let content = tokio::fs::read_to_string(path).await?;
            let data: HashMap<String, Value> = serde_json::from_str(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
            *store.data.write() = data;
        }
        Ok(store)
    }
}

#[async_trait]
impl DataStoreTrait for DataStore {
    fn get(&self, key: &str) -> Option<Value> {
        self.data.read().get(key).cloned()
    }

    fn get_string(&self, key: &str) -> Option<String> {
        self.data
            .read()
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    fn get_int(&self, key: &str) -> Option<i64> {
        self.data.read().get(key).and_then(|v| v.as_i64())
    }

    fn get_bool(&self, key: &str) -> Option<bool> {
        self.data.read().get(key).and_then(|v| v.as_bool())
    }

    fn set(&mut self, key: &str, value: Value) {
        self.data.write().insert(key.to_string(), value);
    }

    fn remove(&mut self, key: &str) -> Option<Value> {
        self.data.write().remove(key)
    }

    fn contains(&self, key: &str) -> bool {
        self.data.read().contains_key(key)
    }

    async fn load(&mut self, path: &str) -> Result<(), std::io::Error> {
        let content = tokio::fs::read_to_string(path).await?;
        let data: HashMap<String, Value> = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        *self.data.write() = data;
        Ok(())
    }

    async fn save(&self, path: &str) -> Result<(), std::io::Error> {
        // Clone data to avoid holding lock across await
        let content = {
            let data = self.data.read();
            serde_json::to_string_pretty(&*data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
        };
        tokio::fs::write(path, content).await
    }
}
