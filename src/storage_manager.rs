use std::collections::HashMap;

use crate::{StorageProvider, GetData, SaveData};

/// Manage all storage providers.
/// 
/// # Example
/// ```
/// let mut manager = StorageManager::new();
/// ```
pub struct StorageManager {
    storage_providers:  HashMap<String, Box<dyn StorageProvider>>,
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            storage_providers: HashMap::new(),
        }
    }

    /// Add a storage provider instance to the manager
    pub fn add_storage_provider(self: &mut Self, layer_key: String, provider: Box<dyn StorageProvider>) {
        self.storage_providers.insert(layer_key, provider);
    }

    /// Remove a loaded storage provider instance
    pub fn remove_storage_provider(self: &mut Self, layer_key: &str) {
        if self.storage_providers.contains_key(layer_key) {
            self.storage_providers.remove(layer_key);
        }
    }

    /// Get all layers.
    /// 
    /// Layers are the names of storage provider instances.
    pub fn get_storage_provider_layers(self: &Self) -> Vec<&String> {
        let mut keys = vec![];
        for key in self.storage_providers.keys() {
            keys.push(key);
        }
        keys
    }

    /// Get data from a storage layer with a key.
    pub fn get(self: &Self, layer_key: &str, key: &str) -> Result<GetData, String> {
        if self.storage_providers.contains_key(layer_key) {
            return self.storage_providers.get(layer_key).unwrap().get(key);
        }
        Err("Storage provider layer not found for `get` action".to_owned())
    }

    /// Find the first data entry for the key in any storage provider.
    pub fn find(self: &Self, key: &str) -> Result<GetData, String> {
        for layer in self.storage_providers.iter() {
            if let Ok(result) = layer.1.get(key) {
                return Ok(result);
            }
        }
        Err(format!("Requested key: `{}` not found in any storage provider.", key))
    }

    /// Save data to the storage layer.
    pub fn save(self: &Self, layer_key: &str, key: &str, raw: Vec<u8>) -> Result<SaveData, String> {
        if self.storage_providers.contains_key(layer_key) {
            return self.storage_providers.get(layer_key).unwrap().save(key, raw);
        }
        Err("Storage provider layer not found for `save` action".to_owned())
    }

    /// Queue an entry for deletion in a specific layer.
    pub fn delete(self: &Self, layer_key: &str, key: &str) {
        if self.storage_providers.contains_key(layer_key) {
            return self.storage_providers.get(layer_key).unwrap().delete(key);
        }
    }

    /// Queue for deletion all entires which match the key on any layer.
    pub fn delete_all(self: &Self, key: &str) {
        for layer in self.storage_providers.iter() {
            layer.1.delete(key)
        }
    }

    /// Execute free on all layers.
    pub fn free(self: &Self) {
        for layer in self.storage_providers.iter() {
            layer.1.free();
        }
    }

    /// Executes force free on all layers.
    pub fn force_free(self: &Self, all: bool) {
        for layer in self.storage_providers.iter() {
            layer.1.force_free(all);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{filestorage::FileStorageProvider, StorageProvider};

    use super::StorageManager;

    const FILE_STORAGE: &str = "test_fstore";
    const DELETE_STORAGE: &str = "test_fdelete";
    const FILE_KEY: &str = "1234";

    fn clean_up(test_key: &str) {
        let f_path = format!("{}_{}", FILE_STORAGE, test_key);
        let d_path = format!("{}_{}", DELETE_STORAGE, test_key);
        let attr = std::fs::metadata(&f_path).unwrap();
        if attr.is_dir() {
            std::fs::remove_dir_all(&f_path).unwrap();
        }
        let attr = std::fs::metadata(&d_path).unwrap();
        if attr.is_dir() {
            std::fs::remove_dir_all(&d_path).unwrap();
        }
    }

    fn storage_provider_instance(test_key: &str) -> Box<dyn StorageProvider> {
        let f_path = format!("{}_{}", FILE_STORAGE, test_key);
        let d_path = format!("{}_{}", DELETE_STORAGE, test_key);

        Box::new(FileStorageProvider::new(f_path.to_owned(), d_path.to_owned()))
    }

    #[test]
    fn add_provider() {
        let f_key = "add_provider";
        let mut manager = StorageManager::new();
        manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
        assert_eq!(manager.get_storage_provider_layers().len(), 1);
        clean_up(f_key);
    }


    #[test]
    fn remove_provider() {
        let f_key = "remove_provider";
        let mut manager = StorageManager::new();
        manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
        manager.remove_storage_provider("layer1");
        assert_eq!(manager.get_storage_provider_layers().len(), 0);
        clean_up(f_key);
    }


    #[test]
    fn save() {
        let f_key = "save_provider";
        let mut manager = StorageManager::new();
        manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
        let save_result = manager.save("layer1", FILE_KEY, "test".to_owned().into_bytes()).unwrap();
        assert_eq!(save_result.key, FILE_KEY);
        clean_up(f_key);
    }

    #[test]
    fn get() {
        let f_key = "get_provider";
        let mut manager = StorageManager::new();
        manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
        let _save_result = manager.save("layer1", FILE_KEY, "test".to_owned().into_bytes());
        let result = manager.get("layer1", FILE_KEY).unwrap();
        assert_eq!(result.size, 4);
        assert_eq!(result.key, FILE_KEY);
        clean_up(f_key);
    }

    #[test]
    fn find() {
        let f_key = "find_provider";
        let mut manager = StorageManager::new();
        manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
        let _save_result = manager.save("layer1", FILE_KEY, "test".to_owned().into_bytes());
        let result = manager.find(FILE_KEY).unwrap();
        assert_eq!(result.size, 4);
        assert_eq!(result.key, FILE_KEY);
        clean_up(f_key);
    }

    #[test]
    fn delete() {
        let f_key = "delete_provider";
        let mut manager = StorageManager::new();
        manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
        let _save_result = manager.save("layer1", FILE_KEY, "test".to_owned().into_bytes());
        manager.delete("layer1", FILE_KEY);
        let attr = std::fs::metadata(format!("{}_{}/{}", DELETE_STORAGE, f_key, FILE_KEY)).unwrap();
        assert!(attr.is_file());
        clean_up(f_key);
    }

    #[test]
    fn free() {
        let f_key = "free_provider";
        let mut manager = StorageManager::new();
        manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
        let _save_result = manager.save("layer1", FILE_KEY, "test".to_owned().into_bytes());
        manager.delete("layer1", FILE_KEY);
        manager.free();
        let exists = std::path::Path::new(&format!("{}_{}/{}", DELETE_STORAGE, f_key, FILE_KEY)).exists();
        assert!(exists);
        clean_up(f_key);
    }

    #[test]
    fn free_force() {
        let f_key = "free_force_provider";
        let mut manager = StorageManager::new();
        manager.add_storage_provider("layer1".to_owned(), storage_provider_instance(f_key));
        let _save_result = manager.save("layer1", FILE_KEY, "test".to_owned().into_bytes());
        manager.delete("layer1", FILE_KEY);
        manager.force_free(true);
        let exists = std::path::Path::new(&format!("{}_{}/{}", DELETE_STORAGE, f_key, FILE_KEY)).exists();
        assert!(!exists);
        clean_up(f_key);
    }
}