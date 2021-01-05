extern crate base64;
extern crate hex;

pub fn solve(input: &str) -> String {
    // we turn the input string into an array of bytes (0-255) to work with
    // raw bytes instead of relying on the string to be hex-encoded
    let decoded_bytes =
        hex::decode(input.as_bytes()).expect("Error while decoding expected hex string");

    println!("{:?}", decoded_bytes);
    println!("{}", std::str::from_utf8(decoded_bytes.as_slice()).unwrap());

    base64::encode(decoded_bytes)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn convert_hex_to_base64() {
        let input = read_input("101");
        let expected_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        dbg!(&input);

        assert_eq!(solve(&input), expected_output);
    }
}
