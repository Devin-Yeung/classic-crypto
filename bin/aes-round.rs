use cns::aes::hazmat::{key_expansion, State};

fn bytes_to_u32_array(bytes: &[u8; 16], big_endian: bool) -> [u32; 4] {
    let mut ints = [0_u32; 4];
    for i in 0..4 {
        let offset = i * 4;
        let byte_slice = &bytes[offset..offset + 4];
        ints[i] = if big_endian {
            u32::from_be_bytes(byte_slice.try_into().unwrap())
        } else {
            u32::from_le_bytes(byte_slice.try_into().unwrap())
        };
    }
    ints
}

fn main() {
    let plaintext: [u8; 16] = [
        0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
        0x00,
    ];
    let key: [u8; 16] = [
        0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
        0x02,
    ];

    let key = bytes_to_u32_array(&key, true);

    let rks = key_expansion(&key);

    let mut state = State::from_raw(plaintext);

    println!("Original State");
    println!("{}", state.matrix_view());

    println!("State After Add Round Key");
    state.add_rk(&rks[0..4]);
    println!("{}", state.matrix_view());

    println!("State After Sub Bytes");
    state.sub_bytes();
    println!("{}", state.matrix_view());

    println!("State After Shift Rows");
    state.shift_rows();
    println!("{}", state.matrix_view());

    println!("State After Mix Columns");
    state.mix_col();
    println!("{}", state.matrix_view());
}
