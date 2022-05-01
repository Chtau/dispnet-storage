use std::collections::HashMap;

use crate::{StorageProvider, GetData, SaveData};

pub struct StorageManager {
    storage_providers:  HashMap<String, Box<dyn StorageProvider>>,
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            storage_providers: HashMap::new(),
        }
    }

    pub fn set_storage_layer(self: &mut Self, layer_key: String, provider: Box<dyn StorageProvider>) {
        self.storage_providers.insert(layer_key, provider);
    }

    pub fn get(self: &Self, layer_key: &str, key: &str) -> Result<GetData, String> {
        if self.storage_providers.contains_key(layer_key) {
            return self.storage_providers.get(layer_key).unwrap().get(key);
        }
        Err("Storage provider layer not found for `get` action".to_owned())
    }

    pub fn save(self: &Self, layer_key: &str, key: &str, raw: Vec<u8>) -> Result<SaveData, String> {
        if self.storage_providers.contains_key(layer_key) {
            return self.storage_providers.get(layer_key).unwrap().save(key, raw);
        }
        Err("Storage provider layer not found for `save` action".to_owned())
    }

    pub fn delete(self: &Self, layer_key: &str, key: &str) {
        if self.storage_providers.contains_key(layer_key) {
            return self.storage_providers.get(layer_key).unwrap().delete(key);
        }
    }

    pub fn free(self: &Self, layer_key: &str) {
        if self.storage_providers.contains_key(layer_key) {
            return self.storage_providers.get(layer_key).unwrap().free();
        }
    }

    pub fn force_free(self: &Self, layer_key: &str, all: bool) {
        if self.storage_providers.contains_key(layer_key) {
            return self.storage_providers.get(layer_key).unwrap().force_free(all);
        }
    }
}