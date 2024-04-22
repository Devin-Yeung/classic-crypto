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
}
