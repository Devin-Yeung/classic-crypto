use crate::visual::get_nth_bit_u64;
use crate::visual::Endian::{Big, Little};

#[rustfmt::skip]
const INIT_PERM_LUT: &[usize; 64] = &[
    58, 50, 42, 34, 26, 18, 10, 2,
    60, 52, 44, 36, 28, 20, 12, 4,
    62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8,
    57, 49, 41, 33, 25, 17,  9, 1,
    59, 51, 43, 35, 27, 19, 11, 3,
    61, 53, 45, 37, 29, 21, 13, 5,
    63, 55, 47, 39, 31, 23, 15, 7 ,
];

#[rustfmt::skip]
const PERMUTED_CHOICE_1: &[usize; 56] = &[
    // Select C
    57, 49, 41, 33, 25, 17,  9,
     1, 58, 50, 42, 34, 26, 18,
    10,  2, 59, 51, 43, 35, 27,
    19, 11,  3, 60, 52, 44, 36,
    // Select D
    63, 55, 47, 39, 31, 23, 15,
     7, 62, 54, 46, 38, 30, 22,
    14,  6, 61, 53, 45, 37, 29,
    21, 13,  5, 28, 20, 12,  4,
];

#[rustfmt::skip]
const PERMUTED_CHOICE_2: &[usize; 48] = &[
    14, 17, 11, 24,  1,  5,
     3, 28, 15,  6, 21, 10,
    23, 19, 12,  4, 26,  8,
    16,  7, 27, 20, 13,  2,
    41, 52, 31, 37, 47, 55,
    30, 40, 51, 45, 33, 48,
    44, 49, 39, 56, 34, 53,
    46, 42, 50, 36, 29, 32,
];

fn pc_1(data: u64) -> (u32, u32) {
    let mut c = 0u32;
    for i in 0..28 {
        // It's crazy, why big endian?
        let bit = get_nth_bit_u64(data, PERMUTED_CHOICE_1[i] - 1, Big) as u32;
        c |= bit << (27 - i);
    }

    let mut d = 0u32;
    for i in 28..56 {
        let bit = get_nth_bit_u64(data, PERMUTED_CHOICE_1[i] - 1, Big) as u32;
        d |= bit << (55 - i);
    }

    // the highest 8 bit is unused
    // which means the highest 4 bit is unused if split
    debug_assert!(c >> 28 == 0);
    debug_assert!(d >> 28 == 0);
    (c, d)
}

fn pc_2(c: u32, d: u32) -> u64 {
    let c = c as u64;
    let d = d as u64;
    debug_assert!(c >> 28 == 0);
    debug_assert!(d >> 28 == 0);
    let mut merge = 0;
    merge |= c << 28;
    merge |= d;

    merge <<= 8; // HACK! to make sure get_nth_bit works in the Big Endian way

    let mut result = 0;
    for i in 0..PERMUTED_CHOICE_2.len() {
        let bit = get_nth_bit_u64(merge, PERMUTED_CHOICE_2[i] - 1, Big);
        result |= bit << (47 - i);
    }
    // the highest 16 bit is unused
    debug_assert!(result >> 48 == 0);
    result
}

pub fn left_shift_1(data: u32) -> u32 {
    // highest 4 bit is unused
    debug_assert!(data >> 28 == 0);

    let preserve = (data & 0x0800_0000) >> 27;
    debug_assert!(preserve == 0 || preserve == 1); // only 1 bit is preserved
    return ((data << 1) | preserve) & 0x0FFF_FFFF;
}

pub fn init_perm(data: u64) -> u64 {
    let mut result = 0;
    for i in 0..64 {
        // big endian
        let bit = (get_nth_bit_u64(data, INIT_PERM_LUT[i] - 1, Big)) << (63 - i);
        result |= bit;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::des::permutation::{init_perm, left_shift_1, pc_1, pc_2};

    #[test]
    fn test_init_perm() {
        let data: u64 = 0x0246_8ace_eca8_6420;
        let expect: u64 = 0x5a00_5a00_3cf0_3c0f;
        assert_eq!(init_perm(data), expect);
    }

    #[test]
    fn test_left_shift_1() {
        let data: u32 = 0x0800_0000;
        let expect: u32 = 0x0000_0001;
        assert_eq!(left_shift_1(data), expect);
    }

    #[test]
    fn round_key_1() {
        // https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm
        let key: u64 = 0b0001_0011_0011_0100_01010111_01111001_10011011_10111100_11011111_11110001;
        let (c0, d0) = pc_1(key);
        assert_eq!(c0, 0b_1111000011001100101010101111);
        assert_eq!(d0, 0b_0101010101100110011110001111);
        let c1 = left_shift_1(c0);
        let d1 = left_shift_1(d0);
        assert_eq!(c1, 0b_1110000110011001010101011111);
        assert_eq!(d1, 0b_1010101011001100111100011110);
        let rk1 = pc_2(c1, d1);
        let expected: u64 = 0b000110_110000_001011_101111_111111_000111_000001_110010;
        assert_eq!(rk1, expected);
    }
}
