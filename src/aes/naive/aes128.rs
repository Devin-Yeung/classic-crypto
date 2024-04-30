use crate::aes::naive::key_expansion::key_expansion;
use crate::aes::naive::state::State;

pub fn aes128_encryption(plaintext: [u8; 16], key: &[u32; 4]) -> [u8; 16] {
    let mut state = State::from_raw(plaintext);
    let rks = key_expansion(key);

    state.add_rk(&rks[0..4]);
    for round in 1..10 {
        state.sub_bytes();
        state.shift_rows();
        state.mix_col();
        state.add_rk(&rks[round * 4..(round + 1) * 4]);
    }
    state.sub_bytes();
    state.shift_rows();
    state.add_rk(&rks[40..44]);
    state.bytes
}

#[cfg(test)]
mod test {
    use crate::aes::naive::aes128_encryption;

    #[test]
    fn test_aes128_encryption() {
        let data: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let rk: [u32; 4] = [0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f];
        let cipher = aes128_encryption(data, &rk);
        let expected = [
            0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4,
            0xc5, 0x5a,
        ];
        assert_eq!(cipher, expected)
    }
}
