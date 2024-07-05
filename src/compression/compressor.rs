use std::io::{Read, Write};
use std::io::Cursor;

use flate2::{Compression as Flate2Compression, read::GzDecoder, write::GzEncoder};
use xz2::{read::XzDecoder, write::XzEncoder};
use zstd::stream::{decode_all, encode_all};

use super::*;

pub struct Compressor;

impl DeflateCompression for Compressor {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut encoder = flate2::write::DeflateEncoder::new(Vec::new(), Flate2Compression::default());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        let mut decoder = flate2::read::DeflateDecoder::new(data);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer).unwrap();
        buffer
    }
}

impl ZstdCompression for Compressor {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        encode_all(Cursor::new(data), 0).unwrap()
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        decode_all(Cursor::new(data)).unwrap()
    }
}

impl LzmaCompression for Compressor {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut encoder = XzEncoder::new(Vec::new(), 6);
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        let mut decoder = XzDecoder::new(data);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer).unwrap();
        buffer
    }
}

impl GzipCompression for Compressor {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Flate2Compression::default());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        let mut decoder = GzDecoder::new(data);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer).unwrap();
        buffer
    }
}