// Re-export public traits and functions
pub use super::{compress_with_deflate, compress_with_zstd, decompress_with_deflate, decompression_with_zstd};
pub use super::{compress_with_gzip, compress_with_lzma, decompress_with_gzip, decompress_with_lzma};

#[cfg(test)]
mod tests {
    use compressor::Compressor;

    use crate::compression::compressor;

    use super::*;

    #[test]
    fn test_deflate_compression() {
        let compressor = Compressor;

        let original_data = b"Hello, world!";
        let compressed_data = compress_with_deflate(&compressor, original_data);
        let decompressed_data = decompress_with_deflate(&compressor, &compressed_data);
        assert_eq!(decompressed_data, original_data);
    }

    #[test]
    fn test_zstd_compression() {
        let compressor = Compressor;

        let original_data = b"Hello, world!";
        let compressed_data = compress_with_zstd(&compressor, original_data);
        let decompressed_data = decompression_with_zstd(&compressor, &compressed_data);

        assert_eq!(decompressed_data, original_data);
    }

    #[test]
    fn test_gzip_compression() {
        let compressor = Compressor;

        let original_data = b"Hello, world!";
        let compressed_data = compress_with_gzip(&compressor, original_data);
        let decompressed_data = decompress_with_gzip(&compressor, &compressed_data);

        assert_eq!(decompressed_data, original_data);
    }

    #[test]
    fn test_lzma_compression() {
        let compressor = Compressor;

        let original_data = b"Hello, world!";
        let compressed_data = compress_with_lzma(&compressor, original_data);
        let decompressed_data = decompress_with_lzma(&compressor, &compressed_data);
        println!("{:?} {:?}", compressed_data, decompressed_data);

        assert_eq!(decompressed_data, original_data);
    }
}