use dispnet_shared::Package;

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

pub enum PolicyTrigger {
    BeforeGet = 0,
    AfterGet = 1,
    BeforeSave = 2,
    AfterSave = 3,
    BeforeDelete = 4,
    AfterDelete = 5,
    BeforeFree = 6,
}

pub trait Policy {
    fn get_validation_conditions() -> Vec<PolicyTrigger>;
    fn validate(package: Package, client: String) -> bool;
}

pub trait StorageProvider {
    fn get(self: &Self, key: &str) -> Result<GetData, String>;
    fn save(self: &Self, key: &str, raw: Vec<u8>) -> Result<SaveData, String>;
    fn delete(self: &Self, key: &str);
    fn free(self: &Self);
    fn force_free(self: &Self, all: bool);
}