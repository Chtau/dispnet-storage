use std::{fs::File, io::{Read, Write}};

use crate::{DispnetStorageProvider, GetData, SaveData};

pub struct DispnetFileStorage {

}

impl DispnetFileStorage {
    pub fn new() -> Self {
        Self {

        }
    }

    fn internal_file_path(self: DispnetFileStorage, key: &str) -> String {
        return key.to_owned();
    }
}

impl DispnetStorageProvider for DispnetFileStorage {
    fn get(self: DispnetFileStorage, key: &str) -> Result<GetData, String> {
        let file_result = File::open(self.internal_file_path(key));
        if let Ok(mut file) = file_result {
            let mut buffer = Vec::new();
            let read_size_result = file.read_to_end(&mut buffer);
            if let Ok(f_size) = read_size_result {
                return Ok(GetData {
                    key: key.to_owned(),
                    size: f_size,
                    data: buffer   
                });
            }
        } else {
            // TODO: provide detail error to callers
            let _f_err = file_result.err();
            return Err("File open error".to_owned());
        }
        Err("Not found".to_owned())
    }

    fn save(self: DispnetFileStorage, key: &str, raw: Vec<u8>) -> Result<SaveData, String> {
        let buffer_result = File::create(self.internal_file_path(key));
        if let Ok(mut buffer) = buffer_result {
            if buffer.write_all(&raw).is_ok() {
                return Ok(SaveData {
                    key: key.to_owned(),
                    size: raw.len(),
                });
            }
        }
        Err("Could not save".to_owned())
    }

    fn delete(self: DispnetFileStorage) {

    }

    fn free(self: DispnetFileStorage) {
        
    }

    fn force_free(self: DispnetFileStorage) {

    }
}