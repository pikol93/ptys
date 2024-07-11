use color_eyre::{eyre::eyre, Result};
use num_bigint::{BigInt, Sign};

pub struct Field {
    pub starting_bit: usize,
    pub bits: usize,
}

impl Field {
    pub fn decode(&self, buffer: &[u8]) -> Result<BigInt> {
        let end_bit = self.starting_bit + self.bits - 1;
        let end_byte = end_bit / u8::BITS as usize;
        if end_byte >= buffer.len() {
            return Err(eyre!("Buffer is not large enough."));
        }

        let starting_byte = self.starting_bit / u8::BITS as usize;
        let buffer = &buffer[starting_byte..(end_byte + 1)];
        let big_int = BigInt::from_bytes_be(Sign::Plus, buffer);
        let end_bit_index = end_bit % u8::BITS as usize;
        let shift_amount = u8::BITS as usize - (end_bit_index + 1);
        let big_int = big_int >> shift_amount;

        Ok(big_int)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_decode_single_bit_value() {
        let field = Field {
            starting_bit: 0,
            bits: 1,
        };

        let buffer = [0b10000000];
        let value = field.decode(&buffer).unwrap();
        assert_eq!(value, BigInt::from(1));
    }

    #[test]
    pub fn should_decode_multiple_bits_value() {
        let field = Field {
            starting_bit: 0,
            bits: 3,
        };

        let buffer = [0b11100000];
        let value = field.decode(&buffer).unwrap();
        assert_eq!(value, BigInt::from(7));
    }

    #[test]
    pub fn should_decode_multiple_bits_value_with_non_zero_starting_bit() {
        let field = Field {
            starting_bit: 1,
            bits: 3,
        };

        let buffer = [0b01110000];
        let value = field.decode(&buffer).unwrap();
        assert_eq!(value, BigInt::from(7));
    }

    #[test]
    pub fn should_decode_multiple_bits_value_with_it_being_encoded_on_multiple_bytes() {
        let field = Field {
            starting_bit: 6,
            bits: 3,
        };

        let buffer = [0b00000011, 0b10000000];
        let value = field.decode(&buffer).unwrap();
        assert_eq!(value, BigInt::from(7));
    }

    #[test]
    pub fn should_decode_multiple_bits_value_with_it_being_encoded_between_multiple_bytes() {
        let field = Field {
            starting_bit: 7,
            bits: 10,
        };

        let buffer = [0b0000_0001, 0b1111_1111, 0b1000_0000];
        let value = field.decode(&buffer).unwrap();
        assert_eq!(value, BigInt::from(1023));
    }

    #[test]
    pub fn should_fail_decoding_when_buffer_is_not_large_enough() {
        let field = Field {
            starting_bit: 0,
            bits: 12345,
        };

        let buffer = [0b00000000];
        let result = field.decode(&buffer);
        assert!(result.is_err())
    }
}
