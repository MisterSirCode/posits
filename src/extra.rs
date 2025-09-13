use core::f64;
use super::posit8::p8;
use super::posit16::p16;
use super::posit32::p32;
use super::posit64::p64;

/// Run test cases of every major variation of every posit conversion type
pub fn unit_tests() {
    // Posit 8 Unit Tests
    assert_eq!(f64::from(p8::PI), 3.125);                    // p8_1 Default
    assert_eq!(f64::from(p8::from(0b10100111)), -3.125);     // p8_1 Default Negative
    assert_eq!(p8::from(0b01101001).as_float_es(0), 3.125);  // p8_0 Custom ES Special-Case
    assert_eq!(p8::from(0b10010111).as_float_es(0), -3.125); // p8_0 Custom ES Special-Case Negative
    assert_eq!(p8::from(0b01001101).as_float_es(2), 3.25);   // p8_2 Custom ES
    assert_eq!(p8::from(0b10110011).as_float_es(2), -3.25);  // p8_2 Custom ES Negative
    // Posit 16 Unit Tests
    assert_eq!(f64::from(p16::PI), 3.1416015625f64);                            // p16_2 Default
    assert_eq!(f64::from(p16::from(0b1011001101101111)), -3.1416015625f64);     // p16_2 Default Negative
    assert_eq!(p16::from(0b0110100100100010).as_float_es(0), 3.1416015625f64);  // p16_0 Custom ES Special-Case
    assert_eq!(p16::from(0b1001011011011110).as_float_es(0), -3.1416015625f64); // p16_0 Custom ES Special-Case Negative
    assert_eq!(p16::from(0b0101100100100010).as_float_es(1), 3.1416015625f64);  // p16_1 Custom ES
    assert_eq!(p16::from(0b1010011011011110).as_float_es(1), -3.1416015625f64); // p16_1 Custom ES Negative
    // Posit 32 Unit Tests
    assert_eq!(f64::from(p32::PI), 3.141592651605606);                                            // p32_3 Default
    assert_eq!(f64::from(p32::from(0b10111001101101111000000100101011)), -3.141592651605606);     // p32_3 Default Negative
    assert_eq!(p32::from(0b01101001001000011111101101010100).as_float_es(0), 3.141592651605606);  // p32_0 Custom ES Special-Case
    assert_eq!(p32::from(0b10010110110111100000010010101100).as_float_es(0), -3.141592651605606); // p32_0 Custom ES Special-Case Negative
    assert_eq!(p32::from(0b01011001001000011111101101010100).as_float_es(1), 3.141592651605606);  // p32_1 Custom ES
    assert_eq!(p32::from(0b10100110110111100000010010101100).as_float_es(1), -3.141592651605606); // p32_1 Custom ES Negative
    // Posit 64 Unit Tests
    assert_eq!(f64::from(p64::PI), f64::consts::PI);                                                                             // p64_4 Default
    assert_eq!(f64::from(p64::from(0b10111100110110111100000010010101_01110111011110100101110100000000)), -f64::consts::PI);     // p64_4 Default Negative
    assert_eq!(p64::from(0b01101001001000011111101101010100_01000100001011010001100000000000).as_float_es(0), f64::consts::PI);  // p64_0 Custom ES Special-Case
    assert_eq!(p64::from(0b10010110110111100000010010101011_10111011110100101110100000000000).as_float_es(0), -f64::consts::PI); // p64_0 Custom ES Special-Case Negative
    assert_eq!(p64::from(0b01001100100100001111110110101010_00100010000101101000110000000000).as_float_es(2), f64::consts::PI);  // p64_2 Custom ES
    assert_eq!(p64::from(0b10110011011011110000001001010101_11011101111010010111010000000000).as_float_es(2), -f64::consts::PI); // p64_2 Custom ES Negative
}