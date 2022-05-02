pub mod filestorage;
pub mod policy;
pub mod storage_manager;

/// Successful result on the storage provider `get` function.
pub struct GetData {
    /// Key of the entry.
    pub key: String,
    /// Byte size of the entry.
    pub size: usize,
    /// Raw data of the entry.
    pub data: Vec<u8>
}

/// Successful result on the storage provider save function.
pub struct SaveData {
    /// Key of the saved entry.
    pub key: String,
    /// Byte size of the entry.
    pub size: usize,
}

/// This `trait`must be implemented to use a `struct` as a storage provider.
pub trait StorageProvider {
    /// Get data from a storage provider with a key.
    fn get(self: &Self, key: &str) -> Result<GetData, String>;
    /// Save data to the storage provider.
    fn save(self: &Self, key: &str, raw: Vec<u8>) -> Result<SaveData, String>;
    /// Queue an entry for deletion in the storage provider.
    fn delete(self: &Self, key: &str);
    /// Execute free.
    fn free(self: &Self);
    /// Executes force free.
    fn force_free(self: &Self, all: bool);
}
