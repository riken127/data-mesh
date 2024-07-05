use super::{CachedStorage, FileStorage};

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::storage::StorageBackend;

    use super::*;

    #[test]
    fn test_cached_storage() {
        let test_dir = "test_storage";
        if !std::path::Path::new(test_dir).exists() {
            fs::create_dir(test_dir).unwrap();  // Handle errors in a real application
        }

        let backend = FileStorage::new(test_dir);
        let cached_storage = CachedStorage::new(backend, 2);

        let key = "test_key";
        let data = b"test_data";

        // Store data
        cached_storage.store(key, data).unwrap();

        // Retrieve data
        let retrieved_data = cached_storage.retrieve(key).unwrap();
        assert_eq!(retrieved_data, data);

        // Ensure data is cached
        let cached_data = cached_storage.cache.get(key).unwrap();
        assert_eq!(cached_data, data);

        // Delete data
        cached_storage.delete(key).unwrap();
        assert!(cached_storage.retrieve(key).is_err());
    }

    #[test]
    fn test_store_retrieve_delete_page() {
        let test_dir = "test_storage";
        if !std::path::Path::new(test_dir).exists() {
            fs::create_dir(test_dir).unwrap();  // Handle errors in a real application
        }

        let storage = FileStorage::new(test_dir);

        // Test store_page
        let key = "test_store_page.txt";
        let page_index = 0;
        let data = vec![0u8; storage.page_size];
        storage.store_page(key, page_index, &data).unwrap();

        // Test retrieve_page
        let retrieved_data = storage.retrieve_page(key, page_index).unwrap();
        assert_eq!(retrieved_data, data);

        // Test delete_page
        storage.delete_page(key, page_index).unwrap();
        assert!(storage.retrieve_page(key, page_index).is_err()); // Ensure page is deleted
    }

    #[test]
    fn test_retrieve_non_existing_file() {
        let test_dir = "test_storage";
        if !std::path::Path::new(test_dir).exists() {
            fs::create_dir(test_dir).unwrap();  // Handle errors in a real application
        }
        let storage = FileStorage::new(test_dir);
        let key = "non_existing_file.txt";

        let result = storage.retrieve(key);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), format!("File '{}' does not exist.", key));
    }

    #[test]
    fn test_retrieve_non_existing_page() {
        let test_dir = "test_storage";
        if !std::path::Path::new(test_dir).exists() {
            fs::create_dir(test_dir).unwrap();  // Handle errors in a real application
        }
        let storage = FileStorage::new(test_dir);
        let key = "existing_file_for_page.txt"; // Ensure this file exists before test
        let page_index = 10; // Assuming this page index does not exist

        let result = storage.retrieve_page(key, page_index);
        assert!(result.is_err());
        // Handle the specific error case here if desired
    }
}
