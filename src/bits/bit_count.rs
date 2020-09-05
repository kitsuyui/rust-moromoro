pub fn bit_count(n: i8) -> u32 {
    n.count_ones()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    pub fn test_bit_count() {
        assert_eq!(bit_count(0), 0);
        assert_eq!(bit_count(1), 1);
        assert_eq!(bit_count(2), 1);
        assert_eq!(bit_count(4), 1);
        assert_eq!(bit_count(8), 1);
        assert_eq!(bit_count(16), 1);
        assert_eq!(bit_count(32), 1);
        assert_eq!(bit_count(64), 1);

        assert_eq!(bit_count(3), 2);
        assert_eq!(bit_count(5), 2);
    }
}
