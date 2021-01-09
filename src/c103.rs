#[cfg(test)]
mod test {
    use crate::utils::{
        decode_hex, encode_utf8, english_probability, fixed_xor, read_input, repeat,
    };
    use std::{collections::BTreeMap, iter::FromIterator};

    fn decrypt(key_candidate: &str, input: &[u8]) -> String {
        let key = repeat(key_candidate, input.len()).into_bytes();

        let result = fixed_xor(&decode_hex(input), &key);

        encode_utf8(result.as_slice()).to_string()
    }

    #[test]
    fn single_byte_xor_cipher() {
        let input = read_input("103");

        let mut scores = BTreeMap::new();

        // for char in ascii_set
        for letter in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
            let secret = letter.to_string();
            let plaintext = decrypt(&secret, input.as_bytes());
            let score = english_probability(&plaintext);

            scores.insert(secret, score);
        }

        let mut v = Vec::from_iter(scores);
        v.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());

        println!("{:?}", v[0]); // X is the secret!
        println!("{:?}", v[1]);
        println!("{:?}", v[2]);
    }
}
