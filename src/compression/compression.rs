pub trait DeflateCompression {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
    fn decompress(&self, data: &[u8]) -> Vec<u8>;
}

pub trait ZstdCompression {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
    fn decompress(&self, data: &[u8]) -> Vec<u8>;
}

pub trait GzipCompression {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
    fn decompress(&self, data: &[u8]) -> Vec<u8>;
}

pub trait LzmaCompression {
    fn compress(&self, data: &[u8; 13]) -> Vec<u8>;
    fn decompress(&self, data: &[u8]) -> Vec<u8>;
}

pub fn compress_with_gzip<C: GzipCompression>(compressor: &C, data: &[u8]) -> Vec<u8> {
    compressor.compress(data)
}

pub fn decompress_with_gzip<C: GzipCompression>(compressor: &C, data: &[u8]) -> Vec<u8> {
    compressor.decompress(data)
}

pub fn compress_with_lzma<C: LzmaCompression>(compressor: &C, data: &[u8; 13]) -> Vec<u8> {
    compressor.compress(data)
}

pub fn decompress_with_lzma<C: LzmaCompression>(compressor: &C, data: &[u8]) -> Vec<u8> {
    compressor.decompress(data)
}

pub fn compress_with_deflate<C: DeflateCompression>(compressor: &C, data: &[u8]) -> Vec<u8> {
    compressor.compress(data)
}

pub fn decompress_with_deflate<C: DeflateCompression>(compressor: &C, data: &[u8]) -> Vec<u8> {
    compressor.decompress(data)
}

pub fn compress_with_zstd<C: ZstdCompression>(compressor: &C, data: &[u8]) -> Vec<u8> {
    compressor.compress(data)
}

pub fn decompression_with_zstd<C: ZstdCompression>(compressor: &C, data: &[u8]) -> Vec<u8> {
    compressor.decompress(data)
}