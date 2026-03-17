pub fn convert_to_binary(num: u64) -> String {
    let mut result = String::new();
    let mut remainder: u64 = num;
    for exponent in (0..=63).rev() {
        if remainder >= 2_u64.pow(exponent) {
            result.push_str("1");
            remainder -= 2_u64.pow(exponent);
        } else {
            if result.len() != 0 {
                result.push_str("0");
            }
        }
    }

    if result.len() == 0 {
        return String::from("0");
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn output_correctness() {
        assert_eq!(convert_to_binary(0), String::from("0"));
        assert_eq!(convert_to_binary(1), String::from("1"));
        assert_eq!(convert_to_binary(5), String::from("101"));
    }
}
