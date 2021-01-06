#[cfg(test)]
mod test {
    use crate::utils::{decode_hex, encode_hex, encode_utf8, fixed_xor, read_input};

    #[test]
    fn fixed_xor_works() {
        let inputs = read_input("102");
        let mut lines = inputs.lines();
        let a = lines.next().expect("Line 1 missing");
        let b = lines.next().expect("Line 2 missing");

        let decoded_a = decode_hex(a.as_bytes());
        let decoded_b = decode_hex(b.as_bytes());

        let result = fixed_xor(&decoded_a, &decoded_b);

        println!("{}", encode_utf8(decoded_a.as_slice())); // KSSP
        println!("{}", encode_utf8(decoded_b.as_slice())); // hit the bull's eye
        println!("{}", encode_utf8(result.as_slice())); //    the kid don't play

        assert_eq!(encode_hex(&result), "746865206b696420646f6e277420706c6179");
    }
}
