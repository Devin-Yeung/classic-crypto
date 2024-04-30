use cns::des;

fn main() {
    let data: u64 = 0x0123456789ABCDEF;
    let key: u64 = 0x0123456789ABCDEF;

    println!("1st Round Key");
    let (c0, d0) = des::hazmat::pc_1(key);
    println!("Permuted Choice 1");
    println!("c0: {:028b}", c0);
    println!("d0: {:028b}", d0);
    println!("Left Circular Shift");
    let c1 = des::hazmat::left_shift_1(c0);
    let d1 = des::hazmat::left_shift_1(d0);
    println!("c1: {:028b}", c1);
    println!("d1: {:028b}", d1);
    println!("Permuted Choice 2");
    let rk1 = des::hazmat::pc_2(c1, d1);
    println!("rk1: {:048b}", rk1);

    println!("Initial Permutation");
    let permuted = des::hazmat::init_perm(data);
    println!("P: {:064b}", permuted);
    let l0 = (permuted >> 32) as u32;
    println!("L0: {:032b}", l0);
    let r0 = permuted as u32;
    println!("R0: {:032b}", r0);

    println!("Expansion Permutation");
    let expanded = des::hazmat::expansion_perm(r0);
    println!("E[R0]: {:048b}", expanded);

    println!("XOR with Round Key");
    let xor = expanded ^ rk1;
    println!("E[R0] ^ rk1: {:048b}", xor);
}
