use crate::aes::lut::SBOX;
use log::debug;

pub fn rot_words(words: u32) -> u32 {
    let reserve = words >> 24;
    return ((words << 8) & 0xFFFFFF00) | reserve;
}

pub fn sub_words(words: u32) -> u32 {
    let mut ret = 0;
    for i in 0..4 {
        let byte = (words >> (i * 8)) as u8;
        let sbox_byte = SBOX[byte as usize];
        ret |= (sbox_byte as u32) << (i * 8);
    }
    ret
}

pub fn rcon(round: usize) -> u32 {
    match round {
        1 => 0x01000000,
        2 => 0x02000000,
        3 => 0x04000000,
        4 => 0x08000000,
        5 => 0x10000000,
        6 => 0x20000000,
        7 => 0x40000000,
        8 => 0x80000000,
        9 => 0x1b000000,
        10 => 0x36000000,
        _ => panic!("Invalid round number"),
    }
}

pub fn key_expansion(key: &[u32; 4]) -> [u32; 44] {
    let mut i = 0;
    let mut w = [0u32; 44];

    while i < 4 {
        w[i] = key[i];
        debug!("w[{}] = {:08x}", i, key[i]);
        i += 1;
    }

    while i < 44 {
        let temp = w[i - 1];
        if i % 4 == 0 {
            w[i] = sub_words(rot_words(temp)) ^ rcon(i / 4) ^ w[i - 4];
            debug!(
                "w[{}] = sub_words(rot_words(w[{}])) ^ rcon({}) ^ w[{}] = {:08x}",
                i,
                i - 1,
                i / 4,
                i - 4,
                w[i]
            );
        } else {
            debug!(
                "w[{}] = w[{}] ^ w[{}] = {:08x} ^ {:08x}",
                i,
                i - 4,
                i - 1,
                w[i - 4],
                w[i - 1]
            );
            w[i] = w[i - 4] ^ temp;
        }
        i += 1;
    }

    return w;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn key_expansion_test() {
        let key: [u32; 4] = [0x2b7e_1516, 0x28ae_d2a6, 0xabf7_1588, 0x09cf_4f3c];
        let w = key_expansion(&key);
        assert_eq!(w[43], 0xb6630ca6);
    }

    #[test]
    fn test_rot_words() {
        let words: u32 = 0x_09_cf_4f_3c;
        let expected: u32 = 0x_cf_4f_3c_09;
        assert_eq!(rot_words(words), expected);
    }

    #[test]
    fn test_sub_words() {
        let words: u32 = 0x_cf4f_3c09;
        let expected: u32 = 0x_8a84_eb01;
        assert_eq!(sub_words(words), expected);
    }

    #[test]
    fn test_rcon() {
        assert_eq!(rcon(1), 0x01000000);
        assert_eq!(rcon(2), 0x02000000);
        assert_eq!(rcon(3), 0x04000000);
        assert_eq!(rcon(4), 0x08000000);
        assert_eq!(rcon(5), 0x10000000);
        assert_eq!(rcon(6), 0x20000000);
        assert_eq!(rcon(7), 0x40000000);
        assert_eq!(rcon(8), 0x80000000);
        assert_eq!(rcon(9), 0x1b000000);
        assert_eq!(rcon(10), 0x36000000);
    }
}
