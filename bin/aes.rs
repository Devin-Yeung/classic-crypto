use cns::aes::naive::aes128_encryption;
use rand;
use rand::Rng;
use std::hint::black_box;

#[allow(non_upper_case_globals)]
const MiB: usize = 1024 * 1024;

pub fn gen_bench_data(mib: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    (0..mib * MiB).map(|_| rng.gen()).collect::<Vec<u8>>()
}

pub fn gen_bench_key() -> [u32; 4] {
    let mut key = [0u32; 4];
    let mut rng = rand::thread_rng();
    for i in 0..4 {
        key[i] = rng.gen();
    }
    key
}

fn main() {
    let data = gen_bench_data(16);
    let key = gen_bench_key();
    // set timer
    let start = std::time::Instant::now();
    debug_assert!(data.len() % 16 == 0);
    for i in 0..data.len() / 16 {
        let block = &data[i * 16..(i + 1) * 16];
        let x = aes128_encryption(block.try_into().unwrap(), &key);
        black_box(x);
    }
    let elapsed = start.elapsed();
    println!(
        "Time elapsed in aes128_encryption() is: {:?} ms",
        elapsed.as_millis()
    );
}
