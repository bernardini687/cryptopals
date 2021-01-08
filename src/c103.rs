#[cfg(test)]
mod test {
    use crate::utils::{decode_hex, encode_utf8, fixed_xor, read_input, repeat};

    fn decrypt(key_candidate: &str, input: &[u8]) {
        let key = repeat(key_candidate, input.len()).into_bytes();

        let result = fixed_xor(&decode_hex(input), &key);

        // eval against score

        println!("{}", encode_utf8(result.as_slice()));

        // { [score]: text }
    }

    #[test]
    fn single_byte_xor_cipher() {
        let input = read_input("103");

        // for char in ascii_set

        decrypt("a", input.as_bytes());

        // print first 5 candidates sorted desc by the [score] key
    }

    // etaoin shrdlu
}
