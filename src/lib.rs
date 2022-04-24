pub mod filestorage;

pub struct GetData {
    pub key: String,
    pub size: usize,
    pub data: Vec<u8>
}

pub struct SaveData {
    pub key: String,
    pub size: usize,
}

pub trait DispnetStorageProvider {
    fn get(self: Self, key: &str) -> Result<GetData, String>;
    fn save(self: Self, key: &str, raw: Vec<u8>) -> Result<SaveData, String>;
    fn delete(self: Self);
    fn free(self: Self);
    fn force_free(self: Self);
}