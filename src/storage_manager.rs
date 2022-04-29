use crate::{Storage, StorageProvider};

pub struct StorageManager {

}

impl Storage for StorageManager {
    fn set_storage_layer<T>(self: &Self, layer_key: String, provider: Box<T>) where T: StorageProvider {
        todo!()
    }

    fn set_policy<T>(self: &Self, policy: Box<T>) where T: crate::policy::Policy {
        todo!()
    }
}

impl StorageProvider for StorageManager {
    fn get(self: &Self, key: &str) -> Result<crate::GetData, String> {
        todo!()
    }

    fn save(self: &Self, key: &str, raw: Vec<u8>) -> Result<crate::SaveData, String> {
        todo!()
    }

    fn delete(self: &Self, key: &str) {
        todo!()
    }

    fn free(self: &Self) {
        todo!()
    }

    fn force_free(self: &Self, all: bool) {
        todo!()
    }
}