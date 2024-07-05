use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub trait StorageBackend {
    fn store(&self, key: &str, data: &[u8]) -> Result<(), String>;
    fn retrieve(&self, key: &str) -> Result<Vec<u8>, String>;
    fn delete(&self, key: &str) -> Result<(), String>;
    fn store_page(&self, key: &str, page_index: usize, data: &[u8]) -> Result<(), String>;
    fn retrieve_page(&self, key: &str, page_index: usize) -> Result<Vec<u8>, String>;
    fn delete_page(&self, key: &str, page_index: usize) -> Result<(), String>;
}

pub struct FileStorage {
    base_dir: String,
    pub(crate) page_size: usize,
}

impl FileStorage {
    pub fn new(base_dir: &str) -> Self {
        FileStorage {
            base_dir: base_dir.to_string(),
            page_size: 4096,
        }
    }

    fn get_file_path(&self, key: &str) -> PathBuf {
        Path::new(&self.base_dir).join(key)
    }

    fn get_page_path(&self, key: &str, page_index: usize) -> PathBuf {
        self.get_file_path(key).with_extension(format!("page{}", page_index))
    }

    fn create_parent_directories(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn calculate_page_offset(&self, page_index: usize) -> usize {
        page_index * self.page_size
    }

    fn calculate_page_index(&self, offset: usize) -> usize {
        offset / self.page_size
    }
}

impl StorageBackend for FileStorage {
    fn store(&self, key: &str, data: &[u8]) -> Result<(), String> {
        let path = Path::new(&self.base_dir).join(key);
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(data).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<Vec<u8>, String> {
        let path = Path::new(&self.base_dir).join(key);
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        Ok(buffer)
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        let path = Path::new(&self.base_dir).join(key);
        fs::remove_file(path).map_err(|e| e.to_string())
    }

    fn store_page(&self, key: &str, page_index: usize, data: &[u8]) -> Result<(), String> {
        let path = self.get_page_path(key, page_index);
        self.create_parent_directories(&path).expect("Couldn't find path.");
        let mut file = File::create(&path).map_err(|e| e.to_string())?;
        file.write_all(data).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn retrieve_page(&self, key: &str, page_index: usize) -> Result<Vec<u8>, String> {
        let file_path = self.get_file_path(key);

        if !file_path.exists() {
            return Err(format!("File '{}' does not exist.", key));
        }

        let offset = self.calculate_page_offset(page_index);

        let mut file = File::open(&file_path).map_err(|e| e.to_string())?;

        file.seek(SeekFrom::Start(offset as u64)).map_err(|e| e.to_string())?;

        let mut buffer = vec![0; self.page_size];
        file.read_exact(&mut buffer).map_err(|e| e.to_string())?;

        Ok(buffer)
    }

    fn delete_page(&self, key: &str, page_index: usize) -> Result<(), String> {
        let page_path = self.get_page_path(key, page_index);
        fs::remove_file(&page_path).map_err(|e| e.to_string())
    }
}
