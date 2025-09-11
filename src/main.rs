pub mod posit;
use posit::posit8::p8;
use posit::posit16::p16;
use posit::posit32::p32;
use posit::util::PositUtils;

fn f32tou32(f: f32) -> u32 {
    f32::to_bits(f)
}

fn main() {
    PositUtils::unit_tests(); // Validate Operations

    let pi_0 = p8::from(0b01101001);
    let pi_1 = p32::from(0b01011001001000011111101101010100);
    let pi_2 = p32::from(0b01000110010010000111111011010101);
    let pi0_comp = pi_0.components_es(0);
    let pi1_comp = pi_1.components_es(1);
    let pi2_comp = pi_2.components();
    // Unit Tests
    println!("\nPi_0: {:032b}", pi_0.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:032b}, Value: {:032b}, f64: {:?}\n", 
        pi0_comp[0], pi0_comp[1], pi0_comp[2], pi0_comp[3], p8::as_float_es(&pi_0, 0));
    println!("\nPi_1: {:032b}", pi_1.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:032b}, Value: {:032b}, f64: {:?}\n", 
        pi1_comp[0], pi1_comp[1], pi1_comp[2], pi1_comp[3], p32::as_float_es(&pi_1, 1));
    println!("\nPi_2: {:032b}", pi_2.bits);
    println!("Regime: {:?}, ES: {:?}, Exponent: {:032b}, Value: {:032b}, f64: {:?}\n", 
        pi2_comp[0], pi2_comp[1], pi2_comp[2], pi2_comp[3], f64::from(pi_2));
}
