use std::cmp::PartialEq;
use std::fmt::Debug;
use std::mem::size_of;
use std::ops::Add;

trait Bits: Sized + Add<Output = Self> + PartialEq + Debug + Copy {
    fn length(self) -> Self;
    fn count_one(self) -> Self;
    fn count_zero(self) -> Self;
}

macro_rules! impl_Bits {
    ($T:ty) => {
        impl Bits for $T {
            #[inline]
            fn length(self) -> Self {
                (size_of::<Self>() * 8) as Self
            }
            #[inline]
            fn count_one(self) -> Self {
                self.count_ones() as $T
            }
            #[inline]
            fn count_zero(self) -> Self {
                self.count_zeros() as $T
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

    fn assert_count_one_or_zero_equals_to_length<B: Bits>(n: B) {
        // 0 または 1 の数を合計すると、ビットの数全体に等しい
        assert_eq!(n.count_one() + n.count_zero(), n.length());
    }

    #[test]
    fn test_count_one_or_zero_equals_to_length() {
        // u8 は小さいので、全部試せる
        for n in 0..std::u8::MAX {
            assert_count_one_or_zero_equals_to_length(n as u8);
        }
        // ほかは 255 くらいまで試しておく
        for n in 0..255 {
            assert_count_one_or_zero_equals_to_length(n as u16);
            assert_count_one_or_zero_equals_to_length(n as u32);
            assert_count_one_or_zero_equals_to_length(n as u64);
            assert_count_one_or_zero_equals_to_length(n as u128);
        }
        // 最大値付近も要チェック
        for n in (std::u16::MAX - 255)..(std::u16::MAX) {
            assert_count_one_or_zero_equals_to_length(n as u16);
        }
        for n in (std::u32::MAX - 255)..(std::u32::MAX) {
            assert_count_one_or_zero_equals_to_length(n as u64);
        }
        for n in (std::u64::MAX - 255)..(std::u64::MAX) {
            assert_count_one_or_zero_equals_to_length(n as u64);
        }
        for n in (std::u128::MAX - 255)..(std::u128::MAX) {
            assert_count_one_or_zero_equals_to_length(n as u128);
        }
    }

    #[test]
    pub fn test_count_one() {
        assert_eq!(0b00000000u8.count_one(), 0);
        assert_eq!(0b00000001u8.count_one(), 1);
        assert_eq!(0b00000010u8.count_one(), 1);
        assert_eq!(0b00000100u8.count_one(), 1);
        assert_eq!(0b00001000u8.count_one(), 1);
        assert_eq!(0b00010000u8.count_one(), 1);
        assert_eq!(0b00100000u8.count_one(), 1);
        assert_eq!(0b01000000u8.count_one(), 1);
        assert_eq!(0b10000000u8.count_one(), 1);
        assert_eq!(0b11110000u8.count_one(), 4);
        assert_eq!(0b11111111u8.count_one(), 8);

        assert_eq!(0b0000000000000000u16.count_one(), 0);
        assert_eq!(0b0010000000000001u16.count_one(), 2);
    }
}
