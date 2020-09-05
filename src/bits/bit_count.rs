trait Bits {
    fn bit_count(self) -> u32;
}

macro_rules! impl_Bits {
    ($T:ty) => {
        impl Bits for $T {
            fn bit_count(self) -> u32 {
                self.count_ones()
            }
        }
    };
}

impl_Bits!(u8);
impl_Bits!(u16);
impl_Bits!(u32);
impl_Bits!(u64);
impl_Bits!(u128);

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn test_bit_count() {
        assert_eq!(0b00000000u8.bit_count(), 0);
        assert_eq!(0b00000001u8.bit_count(), 1);
        assert_eq!(0b00000010u8.bit_count(), 1);
        assert_eq!(0b00000100u8.bit_count(), 1);
        assert_eq!(0b00001000u8.bit_count(), 1);
        assert_eq!(0b00010000u8.bit_count(), 1);
        assert_eq!(0b00100000u8.bit_count(), 1);
        assert_eq!(0b01000000u8.bit_count(), 1);
        assert_eq!(0b10000000u8.bit_count(), 1);
        assert_eq!(0b11110000u8.bit_count(), 4);
        assert_eq!(0b11111111u8.bit_count(), 8);

        assert_eq!(0b0000000000000000u16.bit_count(), 0);
        assert_eq!(0b0000000000000001u16.bit_count(), 1);
        assert_eq!(0b0010000000000001u16.bit_count(), 2);
        assert_eq!(0b0000000011111111u16.bit_count(), 8);
        assert_eq!(0b1111111111111111u16.bit_count(), 16);
    }
}
