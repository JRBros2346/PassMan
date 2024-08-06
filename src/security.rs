use aes::*;
use sha3::*;

#[derive(PartialEq, Eq)]
pub enum KeyType {
    Normal,
    Hex,
}

pub fn encrypt(key: String, source: String, encode: bool, key_type: KeyType) {
    let source: Vec<u16> = source.encode_utf16().collect();
    if key_type == KeyType::Hex {

    }
}