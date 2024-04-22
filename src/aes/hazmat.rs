use log::debug;

#[rustfmt::skip]
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

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

pub fn key_expansion(key: &[u32; 4], w: &mut [u32; 44]) {
    let mut i = 0;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn key_expansion_test() {
        let key: [u32; 4] = [0x2b7e_1516, 0x28ae_d2a6, 0xabf7_1588, 0x09cf_4f3c];
        let mut w: [u32; 44] = [0; 44];
        key_expansion(&key, &mut w);
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
