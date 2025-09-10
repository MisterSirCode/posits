pub mod posit;

use posit::p16;

fn main() {
    let p = p16::manual(0b0_000_1_101_11011101);
    let comp = p.components();
    let e = f64::from(p);
    println!("\nPosit 16: {:016b}\n", p.bits);
    println!("Regime: {:?}, Exponent: {:08b}, Value: {:08b}\n", comp[0], comp[1], comp[2]);
    println!("Float 64: {e}\n");
}
