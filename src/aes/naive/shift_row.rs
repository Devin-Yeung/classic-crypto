use crate::aes::naive::state::State;

impl State {
    #[inline]
    fn shift_row_1(&mut self) {
        let tmp = self.bytes[1];
        self.bytes[1] = self.bytes[5];
        self.bytes[5] = self.bytes[9];
        self.bytes[9] = self.bytes[13];
        self.bytes[13] = tmp;
    }

    #[inline]
    fn shift_row_2(&mut self) {
        self.bytes.swap(2, 10);
        self.bytes.swap(6, 14);
    }

    #[inline]
    fn shift_row_3(&mut self) {
        let tmp = self.bytes[3];
        self.bytes[3] = self.bytes[15];
        self.bytes[15] = self.bytes[11];
        self.bytes[11] = self.bytes[7];
        self.bytes[7] = tmp;
    }

    #[inline]
    pub fn shift_rows(&mut self) {
        self.shift_row_1();
        self.shift_row_2();
        self.shift_row_3();
    }
}

#[cfg(test)]
mod tests {
    use crate::aes::naive::state::State;

    #[test]
    fn test_shift_rows() {
        // 63 ca b7 04 09 53 d0 51 cd 60 e0 e7 ba 70 e1 8c
        let data: [u8; 16] = [
            0x63, 0xca, 0xb7, 0x04, 0x09, 0x53, 0xd0, 0x51, 0xcd, 0x60, 0xe0, 0xe7, 0xba, 0x70,
            0xe1, 0x8c,
        ];
        // 63 53 e0 8c 09 60 e1 04 cd 70 b7 51 ba ca d0 e7
        let expected = [
            0x63, 0x53, 0xe0, 0x8c, 0x09, 0x60, 0xe1, 0x04, 0xcd, 0x70, 0xb7, 0x51, 0xba, 0xca,
            0xd0, 0xe7,
        ];
        let mut state = State::from_raw(data);
        state.shift_rows();
        assert_eq!(state.bytes, expected);
    }
}
