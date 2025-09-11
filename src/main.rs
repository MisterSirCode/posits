pub mod posit;
use posit::posit16::p16;
// use posit::posit32::p32;

fn f32tou32(f: f32) -> u32 {
    f32::to_bits(f)
}

fn main() {
    let pi_0 = p16::from(0b0_11_0_100100100010);
    let pi_1 = p16::from(0b0_1_0_1_100100100010);
    let pi_2 = p16::from(0b1010010111000100);
    let pi0_comp = pi_0.components_es(0);
    let pi1_comp = pi_1.components_es(1);
    let pi2_comp = pi_2.components();

    println!("\nPi_0: {:016b}", pi_0.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:016b}, Value: {:016b}, f64: {:?}\n", 
        pi0_comp[0], pi0_comp[1], pi0_comp[2], pi0_comp[3], p16::as_float_es(&pi_0, 0));

    println!("\nPi_1: {:016b}", pi_1.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:016b}, Value: {:016b}, f64: {:?}\n", 
        pi1_comp[0], pi1_comp[1], pi1_comp[2], pi1_comp[3], p16::as_float_es(&pi_1, 1));

    println!("\nPi_2: {:016b}", pi_2.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:016b}, Value: {:016b}, f64: {:?}\n", 
        pi2_comp[0], pi2_comp[1], pi2_comp[2], pi2_comp[3], f64::from(pi_2));
}
