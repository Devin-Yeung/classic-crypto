use std::fmt::Write;
use std::mem;

pub enum Endian {
    Big,
    Little,
}

macro_rules! impl_get_nth_bit {
    ($func_name: ident, $type: ty) => {
        pub fn $func_name(data: $type, nth: usize, endian: Endian) -> $type {
            let width: usize = mem::size_of::<$type>() * 8;
            assert!(nth < width, "nth should be < {}", width);
            let ret = match endian {
                Endian::Big => {
                    let shifted = data >> (width - nth - 1);
                    shifted & 0x1
                }
                Endian::Little => {
                    let shifted = data >> nth;
                    shifted & 0x1
                }
            };
            debug_assert!(ret == 0 || ret == 1);
            ret
        }
    };
}

impl_get_nth_bit!(get_nth_bit_u32, u32);

pub fn get_nth_bit_u64(data: u64, nth: usize, endian: Endian) -> u64 {
    let width: usize = mem::size_of::<u64>() * 8;
    assert!(nth < width, "nth should be < {}", width);

    let ret = match endian {
        Endian::Big => {
            let shifted = data >> (width - nth - 1);
            shifted & 0x1
        }
        Endian::Little => {
            let shifted = data >> nth;
            shifted & 0x1
        }
    };
    debug_assert!(ret == 0 || ret == 1);
    ret
}

fn visualize_n_th_bit_u8(data: u8, nth: u8, endian: Endian) {
    let mut buffer = String::new();

    // 0b10101010
    //     ^ 5th bit [Little Endian]
    // 0b10101010
    //        ^ 5th bit [Big Endian]
    writeln!(&mut buffer, "0b{:08b}", data).expect("Failed to write to buffer");
    let pad = match endian {
        Endian::Big => nth as usize + 2,
        Endian::Little => mem::size_of::<u8>() * 8 + 1 - nth as usize,
    };
    for _ in 0..pad {
        write!(&mut buffer, " ").expect("Failed to write to buffer");
    }
    writeln!(
        &mut buffer,
        "^ {}th bit [{} Endian]",
        nth,
        match endian {
            Endian::Big => "Big",
            Endian::Little => "Little",
        }
    )
    .expect("Failed to write to buffer");

    println!("{}", buffer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_bit() {
        let a: u64 = 0xAAAA_AAAA_AAAA_AAAA;
        let b: u64 = 0x5555_5555_5555_5555;
        for i in 0..64 {
            assert_eq!(
                get_nth_bit_u64(a, i, Endian::Big),
                get_nth_bit_u64(b, i, Endian::Little)
            );
        }
    }

    #[test]
    fn test_n_th_bit_u8() {
        visualize_n_th_bit_u8(0b10101010, 5, Endian::Little);
        visualize_n_th_bit_u8(0b10101010, 0, Endian::Little);
        visualize_n_th_bit_u8(0b10101010, 5, Endian::Big);
        visualize_n_th_bit_u8(0b10101010, 0, Endian::Big);
    }
}
