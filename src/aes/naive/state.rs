use tabled::builder::Builder;
use tabled::settings::Style;

#[repr(transparent)]
pub struct State {
    pub bytes: [u8; 16],
}

impl State {
    /// Method to get the value of a specific byte in the State.
    ///
    /// # Arguments
    ///
    /// * `row` - index of the row, should be less than 4.
    /// * `col` - index of the column, should be less than 4.
    ///
    /// # Returns
    ///
    /// The byte (u8) at the specified row and column.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> u8 {
        debug_assert!(row < 4);
        debug_assert!(col < 4);
        // (0, 0) -> 0
        // (1, 0) -> 1
        // (2, 0) -> 2
        // (3, 0) -> 3
        // (0, 1) -> 4
        self.bytes[col * 4 + row]
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        debug_assert!(row < 4);
        debug_assert!(col < 4);
        self.bytes[col * 4 + row] = value;
    }

    pub fn from_raw(raw: [u8; 16]) -> Self {
        Self { bytes: raw }
    }

    pub fn to_raw(self) -> [u8; 16] {
        self.bytes
    }

    pub(crate) fn empty() -> Self {
        Self { bytes: [0; 16] }
    }

    pub fn matrix_view(&self) -> String {
        let mut builder = Builder::default();
        for row in 0..4 {
            let record = (0..4)
                .map(|col| format!("0x{:02x}", self.get(row, col)))
                .collect::<Vec<_>>();
            builder.push_record(record);
        }
        builder.build().with(Style::modern()).to_string()
    }

    pub fn add_rk(&mut self, rk: &[u32]) {
        debug_assert!(rk.len() == 4);
        for col in 0..4 {
            let rk_col = rk[col].to_be_bytes();
            for row in 0..4 {
                self.set(row, col, self.get(row, col) ^ rk_col[row]);
            }
        }
    }

    pub fn sub_bytes(&mut self) {
        for byte in self.bytes.iter_mut() {
            *byte = crate::aes::naive::lut::SBOX[*byte as usize];
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_add_rk() {
        // 00112233445566778899aabbccddeeff
        let data: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let rk = [0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f];
        let mut state = super::State::from_raw(data);
        state.add_rk(&rk);
        assert_eq!(
            state.bytes,
            [
                0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0,
                0xe0, 0xf0
            ]
        )
    }
}
