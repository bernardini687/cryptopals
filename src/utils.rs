extern crate base64;
extern crate hex;

use std::{fs, str};

pub fn decode_hex(bytes: &[u8]) -> Vec<u8> {
    hex::decode(bytes).expect("Error while decoding expected hex bytes")
}

pub fn encode_hex(bytes: &Vec<u8>) -> String {
    hex::encode(bytes)
}

pub fn encode_b64(bytes: &Vec<u8>) -> String {
    base64::encode(bytes)
}

pub fn encode_utf8(bytes: &[u8]) -> &str {
    str::from_utf8(bytes).expect("Error while encoding expected utf-8 bytes")
}

pub fn fixed_xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(aa, bb)| aa ^ bb).collect()
}

pub fn read_input(id: &str) -> String {
    fs::read_to_string(format!("inputs/i{}", id))
        .expect(&format!("Did not find `i{}` in `inputs/`", id))
}
