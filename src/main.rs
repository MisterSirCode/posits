pub mod posit;
use posit::posit64::p64;
use posit::PositUtils;

fn f32tou32(f: f32) -> u32 {
    f32::to_bits(f)
}

fn main() {
    PositUtils::unit_tests(); // Validate Operations

    let pi_0 = p64::from(0b01101001);
    let pi_1 = p64::from(0b01001101);
    let pi_2 = p64::from(0b01011001);
    let pi0_comp = pi_0.components_es(0);
    let pi1_comp = pi_1.components_es(1);
    let pi2_comp = pi_2.components();
    // Unit Tests
    println!("\nPi_0: {:032b}", pi_0.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:032b}, Value: {:032b}, f64: {:?}\n", 
        pi0_comp[0], pi0_comp[1], pi0_comp[2], pi0_comp[3], p64::as_float_es(&pi_0, 0));
    println!("\nPi_1: {:032b}", pi_1.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:032b}, Value: {:032b}, f64: {:?}\n", 
        pi1_comp[0], pi1_comp[1], pi1_comp[2], pi1_comp[3], p64::as_float_es(&pi_1, 2));
    println!("\nPi_2: {:032b}", pi_2.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:032b}, Value: {:032b}, f64: {:?}\n", 
        pi2_comp[0], pi2_comp[1], pi2_comp[2], pi2_comp[3], f64::from(pi_2));
}
