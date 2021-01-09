extern crate base64;
extern crate hex;

use std::collections::BTreeMap;
use std::{fs, iter, str};

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

pub fn repeat(data: &str, times: usize) -> String {
    iter::repeat(data).take(times).collect::<String>()
}

/// Uses the Bhattacharyya coefficient to determine if text is likely to be English.
///
/// Higher is better.
///
/// (courtesy of https://github.com/elasticdog)
pub fn english_probability(text: &str) -> f64 {
    // count of the number of times a character occurs in the given text
    let mut letter_frequencies = BTreeMap::new();

    for letter in text.to_uppercase().chars() {
        let frequency = letter_frequencies.entry(letter).or_insert(0.0);
        *frequency += 1.0;
    }

    println!("{:?}", letter_frequencies);

    // total number of characters in the given text
    let total_letters = text.len() as f64;

    // relative frequency of letters in the English language
    let mut english_frequencies = BTreeMap::new();
    english_frequencies.insert('A', 0.0651738);
    english_frequencies.insert('B', 0.0124248);
    english_frequencies.insert('C', 0.0217339);
    english_frequencies.insert('D', 0.0349835);
    english_frequencies.insert('E', 0.1041442);
    english_frequencies.insert('F', 0.0197881);
    english_frequencies.insert('G', 0.0158610);
    english_frequencies.insert('H', 0.0492888);
    english_frequencies.insert('I', 0.0558094);
    english_frequencies.insert('J', 0.0009033);
    english_frequencies.insert('K', 0.0050529);
    english_frequencies.insert('L', 0.0331490);
    english_frequencies.insert('M', 0.0202124);
    english_frequencies.insert('N', 0.0564513);
    english_frequencies.insert('O', 0.0596302);
    english_frequencies.insert('P', 0.0137645);
    english_frequencies.insert('Q', 0.0008606);
    english_frequencies.insert('R', 0.0497563);
    english_frequencies.insert('S', 0.0515760);
    english_frequencies.insert('T', 0.0729357);
    english_frequencies.insert('U', 0.0225134);
    english_frequencies.insert('V', 0.0082903);
    english_frequencies.insert('W', 0.0171272);
    english_frequencies.insert('X', 0.0013692);
    english_frequencies.insert('Y', 0.0145984);
    english_frequencies.insert('Z', 0.0007836);
    english_frequencies.insert(' ', 0.1918182);

    // update the counts to be the relative frequency of letters in the given text
    // and then calculate the Bhattacharyya coefficient as our score
    let mut score = 0.0;

    for letter in english_frequencies.keys() {
        let relative_frequency = letter_frequencies.entry(*letter).or_insert(0.0);
        *relative_frequency /= total_letters;

        let partition_overlap = letter_frequencies[letter] * english_frequencies[letter];
        score += partition_overlap.sqrt();
    }

    println!("{}", &score);

    score
}
