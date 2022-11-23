
use std::io::Read;

use flate2::{bufread::{ZlibEncoder, ZlibDecoder}, Compression};
use hex::ToHex;
use sha1::{Sha1, Digest};

pub fn zlib_encode(content: &str) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(content.as_bytes(), Compression::fast());
    let mut buffer = Vec::new();
    let _ = encoder.read_to_end(&mut buffer);
    buffer
}

pub fn zlib_decode(content: &Vec<u8>) -> String {
    let mut decoder = ZlibDecoder::new(&content[..]);
    let mut rst = String::new();
    let _ = decoder.read_to_string(&mut rst);

    rst
}

pub fn sha1_encode(content: &Vec<u8>) -> String {
    let mut encoder = Sha1::new();
    encoder.update(String::from_utf8(content.clone()).unwrap());
    let e_rst: Vec<u8> = encoder.finalize()[..].to_vec();
    e_rst.encode_hex()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_zlib_encode() {

    }
    #[test]
    fn test_zlib_decode() {

    }

    #[test]
    fn test_sha1_encode() {

    }
}