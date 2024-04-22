use cns::aes::hazmat::*;
use env_logger;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let key: [u32; 4] = [0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF];
    let mut expansion: [u32; 44] = [0; 44];
    key_expansion(&key, &mut expansion);
}
