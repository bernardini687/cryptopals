pub fn solve(input: &str) -> &str {
    input
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::read_input;

    #[test]
    fn convert_hex_to_base64() {
        let input = read_input("101");

        dbg!(&input);

        assert_eq!(
            solve(&input),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
