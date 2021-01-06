#[cfg(test)]
mod test {
    use crate::utils::{decode_hex, encode_b64, encode_utf8, read_input};

    #[test]
    fn convert_hex_to_base64() {
        let input = read_input("101");

        // we only accept raw bytes (0-255) instead of relying on the input to be a hex-encoded string
        let decoded = decode_hex(input.as_bytes());

        let result = encode_b64(&decoded);

        println!("{}", encode_utf8(decoded.as_slice())); // I'm killing your brain like a poisonous mushroom

        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
