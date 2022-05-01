use policy::{Policy, PolicyRule};

pub mod filestorage;
pub mod policy;
pub mod storage_manager;

pub struct GetData {
    pub key: String,
    pub size: usize,
    pub data: Vec<u8>
}

pub struct SaveData {
    pub key: String,
    pub size: usize,
}

pub trait StorageProvider {
    fn get(self: &Self, key: &str) -> Result<GetData, String>;
    fn save(self: &Self, key: &str, raw: Vec<u8>) -> Result<SaveData, String>;
    fn delete(self: &Self, key: &str);
    fn free(self: &Self);
    fn force_free(self: &Self, all: bool);
}
