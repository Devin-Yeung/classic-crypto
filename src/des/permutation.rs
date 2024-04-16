const INIT_PERM_LUT: &[u8; 64] = &[
    58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61,
    53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
];

pub fn init_perm(data: u64) -> u64 {
    let mut result = 0;
    for i in 0..64 {
        // little endian
        let bit = ((data >> (INIT_PERM_LUT[i] as usize - 1)) & 0x1) << i;
        result |= bit;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::des::permutation::init_perm;

    #[test]
    fn test_init_perm() {
        let data: u64 = 0x02468aceeca86420;
        let expect: u64 = 0x5a005a003cf03c0f;
        assert_eq!(init_perm(data), expect);
    }
}
