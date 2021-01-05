extern crate base64;
extern crate hex;

pub fn solve(input: &str) -> String {
    let decoded_hex =
        hex::decode(input.as_bytes()).expect("Error while decoding expected hex string");

    println!("{:?}", decoded_hex);

    base64::encode(decoded_hex)
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
