use num_bigint::BigUint;

pub fn big_uint_to_usize(big_uint: &BigUint) -> Option<usize> {
    if big_uint.bits() > usize::BITS as u64 {
        return None;
    }

    big_uint
        .iter_u64_digits()
        .next()
        .map(|value| value as usize)
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::*;

    #[test]
    pub fn should_convert_single_byte_numbers() {
        let expected = 0xAB;
        let big_uint = BigUint::from(expected);
        let actual = big_uint_to_usize(&big_uint);

        assert_eq!(Some(expected), actual);
    }

    #[test]
    pub fn should_not_convert_if_integer_is_too_large() {
        let big_uint = BigUint::new(vec![0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF]);
        let actual = big_uint_to_usize(&big_uint);

        assert_eq!(None, actual);
    }

    #[test]
    pub fn should_not_convert_if_integer_has_no_data() {
        let big_uint = BigUint::new(vec![]);
        let actual = big_uint_to_usize(&big_uint);

        assert_eq!(None, actual);
    }
}
