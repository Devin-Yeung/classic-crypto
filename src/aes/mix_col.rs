use crate::aes::lut::{GF_MUL_2, GF_MUL_3};
use crate::aes::state::State;

impl State {
    pub fn mix_col(&mut self) {
        let mut empty = State::empty();
        for col in 0..4 {
            // S_{0, c} = S_{0, c} * 2 XOR S_{1, c} * 3 XOR S_{2, c} XOR S_{3, c}
            empty.set(
                0,
                col,
                GF_MUL_2[self.get(0, col) as usize]
                    ^ GF_MUL_3[self.get(1, col) as usize]
                    ^ self.get(2, col)
                    ^ self.get(3, col),
            );

            // S_{1, c} = S_{0, c} XOR S_{1, c} * 2 XOR S_{2, c} * 3 XOR S_{3, c}
            empty.set(
                1,
                col,
                self.get(0, col)
                    ^ GF_MUL_2[self.get(1, col) as usize]
                    ^ GF_MUL_3[self.get(2, col) as usize]
                    ^ self.get(3, col),
            );

            // S_{2, c} = S_{0, c} XOR S_{1, c} XOR S_{2, c} * 2 XOR S_{3, c} * 3
            empty.set(
                2,
                col,
                self.get(0, col)
                    ^ self.get(1, col)
                    ^ GF_MUL_2[self.get(2, col) as usize]
                    ^ GF_MUL_3[self.get(3, col) as usize],
            );

            // S_{3, c} = S_{0, c} * 3 XOR S_{1, c} XOR S_{2, c} XOR S_{3, c} * 2
            empty.set(
                3,
                col,
                GF_MUL_3[self.get(0, col) as usize]
                    ^ self.get(1, col)
                    ^ self.get(2, col)
                    ^ GF_MUL_2[self.get(3, col) as usize],
            );
        }
        self.bytes = empty.bytes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_col() {
        let data: [u8; 16] = [
            0x63, 0x53, 0xe0, 0x8c, 0x09, 0x60, 0xe1, 0x04, 0xcd, 0x70, 0xb7, 0x51, 0xba, 0xca,
            0xd0, 0xe7,
        ];

        let mut state = State::from_raw(data);
        state.mix_col();
        assert_eq!(
            state.bytes,
            [
                0x5f, 0x72, 0x64, 0x15, 0x57, 0xf5, 0xbc, 0x92, 0xf7, 0xbe, 0x3b, 0x29, 0x1d, 0xb9,
                0xf9, 0x1a
            ]
        );
    }
}
