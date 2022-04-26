use std::{fs::{File, self}, io::{Read, Write}, time::SystemTime};

use crate::{DispnetStorageProvider, GetData, SaveData};

const DAY_IN_SECONDS: u64 = 86_400;

pub struct DispnetFileStorageProvider {
    folder: String,
    delete: String,
}

impl DispnetFileStorageProvider {
    pub fn new(storage_folder: String, delete_folder: String) -> Self {
        let _result = std::fs::create_dir_all(&storage_folder);
        let _result = std::fs::create_dir_all(&delete_folder);
        Self {
            folder: storage_folder,
            delete: delete_folder,
        }
    }

    fn internal_file_path(self: &DispnetFileStorageProvider, key: &str) -> String {
        return format!("{}/{}", self.folder, key.to_owned());
    }

    fn internal_file_delete_path(self: &DispnetFileStorageProvider, key: &str) -> String {
        return format!("{}/{}", self.delete, key.to_owned());
    }

    fn delete_files_older_then(self: &DispnetFileStorageProvider, seconds: u64) {
        for entry in fs::read_dir(self.delete.to_owned()).unwrap() {
            if let Ok(entry_result) = entry {
                let entry_path = entry_result.path();
                if entry_path.is_file() {
                    if seconds == 0 {
                        let _delete_result = fs::remove_file(entry_path);
                    } else {
                        if let Ok(meta) = entry_path.metadata() {
                            if let Ok(mod_time) = meta.modified() {
                                let time_dif = SystemTime::now().duration_since(mod_time).expect("Current time minus file time must be positive");
                                if time_dif.as_secs() > seconds {
                                    let _delete_result = fs::remove_file(entry_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl DispnetStorageProvider for DispnetFileStorageProvider {
    fn get(self: &DispnetFileStorageProvider, key: &str) -> Result<GetData, String> {
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
            // TODO: check if in deleted queue and restore if possible
            // TODO: provide detail error to callers
            let _f_err = file_result.err();
            return Err("File open error".to_owned());
        }
        Err("Not found".to_owned())
    }

    fn save(self: &DispnetFileStorageProvider, key: &str, raw: Vec<u8>) -> Result<SaveData, String> {
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

    fn delete(self: &DispnetFileStorageProvider, key: &str) {
        let from = self.internal_file_path(key).to_owned();
        let to = self.internal_file_delete_path(key).to_owned();
        let _result = fs::rename(from, to);
    }

    fn free(self: &DispnetFileStorageProvider) {
        self.delete_files_older_then(DAY_IN_SECONDS * 15);
    }

    fn force_free(self: &DispnetFileStorageProvider, all: bool) {
        if all {
            self.delete_files_older_then(0);
        } else {
            self.delete_files_older_then(DAY_IN_SECONDS * 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DispnetStorageProvider;

    use super::DispnetFileStorageProvider;

    const FILE_STORAGE: &str = "test_fstore";
    const DELETE_STORAGE: &str = "test_fdelete";
    const FILE_KEY: &str = "1234";

    fn clean_up(f_path: &str, d_path: &str) {
        let attr = std::fs::metadata(f_path).unwrap();
        if attr.is_dir() {
            std::fs::remove_dir_all(f_path).unwrap();
        }
        let attr = std::fs::metadata(d_path).unwrap();
        if attr.is_dir() {
            std::fs::remove_dir_all(d_path).unwrap();
        }
    }

    #[test]
    fn instance() {
        let f_path = format!("{}/{}", FILE_STORAGE, "instance");
        let d_path = format!("{}/{}", DELETE_STORAGE, "instance");

        let _file_storage = DispnetFileStorageProvider::new(f_path.to_owned(), d_path.to_owned());
        let attr = std::fs::metadata(f_path.to_owned()).unwrap();
        assert!(attr.is_dir());
        let attr = std::fs::metadata(d_path.to_owned()).unwrap();
        assert!(attr.is_dir());
        clean_up(&f_path, &d_path);
    }

    #[test]
    fn save() {
        let f_path = format!("{}/{}", FILE_STORAGE, "save");
        let d_path = format!("{}/{}", DELETE_STORAGE, "save");

        let file_storage = DispnetFileStorageProvider::new(f_path.to_owned(), d_path.to_owned());
        let result = file_storage.save(FILE_KEY, "test".to_owned().into_bytes()).unwrap();
        assert_eq!(result.key, FILE_KEY);
        clean_up(&f_path, &d_path);
    }

    #[test]
    fn get() {
        let f_path = format!("{}/{}", FILE_STORAGE, "get");
        let d_path = format!("{}/{}", DELETE_STORAGE, "get");

        let file_storage = DispnetFileStorageProvider::new(f_path.to_owned(), d_path.to_owned());
        file_storage.save(FILE_KEY, "test".to_owned().into_bytes()).unwrap();
        let result = file_storage.get(FILE_KEY).unwrap();
        assert_eq!(result.size, 4);
        assert_eq!(result.key, FILE_KEY);
        clean_up(&f_path, &d_path);
    }

    #[test]
    fn delete() {
        let f_path = format!("{}/{}", FILE_STORAGE, "delete");
        let d_path = format!("{}/{}", DELETE_STORAGE, "delete");

        let file_storage = DispnetFileStorageProvider::new(f_path.to_owned(), d_path.to_owned());
        file_storage.save(FILE_KEY, "test".to_owned().into_bytes()).unwrap();
        file_storage.delete(FILE_KEY);
        let attr = std::fs::metadata(format!("{}/{}", d_path, FILE_KEY)).unwrap();
        assert!(attr.is_file());
        clean_up(&f_path, &d_path);
    }

    
    #[test]
    fn force_free() {
        let f_path = format!("{}/{}", FILE_STORAGE, "force_free");
        let d_path = format!("{}/{}", DELETE_STORAGE, "force_free");

        let file_storage = DispnetFileStorageProvider::new(f_path.to_owned(), d_path.to_owned());
        file_storage.save(FILE_KEY, "test".to_owned().into_bytes()).unwrap();
        file_storage.delete(FILE_KEY);
        file_storage.force_free(true);
        let exists = std::path::Path::new(&format!("{}/{}", d_path, FILE_KEY)).exists();
        assert!(!exists);
        clean_up(&f_path, &d_path);
    }
}