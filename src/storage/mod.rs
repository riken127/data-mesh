pub use backend::{FileStorage, StorageBackend};
pub use cache::Cache;

pub mod backend;
pub mod cache;
mod tests;

pub struct CachedStorage<B: StorageBackend> {
    backend: B,
    cache: Cache,
}

impl<B: StorageBackend> CachedStorage<B> {
    pub fn new(backend: B, cache_capacity: usize) -> Self {
        CachedStorage {
            backend,
            cache: Cache::new(cache_capacity),
        }
    }

    pub fn store(&self, key: &str, data: &[u8]) -> Result<(), String> {
        self.cache.insert(key, data.to_vec());
        self.backend.store(key, data)
    }

    pub fn retrieve(&self, key: &str) -> Result<Vec<u8>, String> {
        if let Some(cached_data) = self.cache.get(key) {
            return Ok(cached_data);
        }
        let data = self.backend.retrieve(key)?;
        self.cache.insert(key, data.clone());
        Ok(data)
    }

    pub fn delete(&self, key: &str) -> Result<(), String> {
        self.cache.delete(key);
        self.backend.delete(key)
    }
}
