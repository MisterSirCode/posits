mod posit8;
mod posit16;
mod posit32;
mod posit64;
pub use posit8::p8;
pub use posit16::p16;
pub use posit32::p32;
pub use posit64::p64;

pub struct PositUtils { }

impl PositUtils {
    pub fn unit_tests() {
        // Posit 8 Unit Tests
        assert_eq!(f64::from(p8::from(0b01011001)), 3.125);      // p8_1 Default
        assert_eq!(f64::from(p8::from(0b10100111)), -3.125);     // p8_1 Default Negative
        assert_eq!(p8::from(0b01101001).as_float_es(0), 3.125);  // p8_0 Custom ES Special-Case
        assert_eq!(p8::from(0b10010111).as_float_es(0), -3.125); // p8_0 Custom ES Special-Case Negative
        assert_eq!(p8::from(0b01001101).as_float_es(2), 3.25);   // p8_2 Custom ES
        assert_eq!(p8::from(0b10110011).as_float_es(2), -3.25);  // p8_2 Custom ES Negative
        // Posit 16 Unit Tests
        assert_eq!(f64::from(p16::from(0b0100110010010001)), 3.1416015625f64);      // p16_2 Default
        assert_eq!(f64::from(p16::from(0b1011001101101111)), -3.1416015625f64);     // p16_2 Default Negative
        assert_eq!(p16::from(0b0110100100100010).as_float_es(0), 3.1416015625f64);  // p16_0 Custom ES Special-Case
        assert_eq!(p16::from(0b1001011011011110).as_float_es(0), -3.1416015625f64); // p16_0 Custom ES Special-Case Negative
        assert_eq!(p16::from(0b0101100100100010).as_float_es(1), 3.1416015625f64);  // p16_1 Custom ES
        assert_eq!(p16::from(0b1010011011011110).as_float_es(1), -3.1416015625f64); // p16_1 Custom ES Negative
        // Posit 32 Unit Tests
        assert_eq!(f64::from(p32::from(0b01000110010010000111111011010101)), 3.141592651605606);      // p32_3 Default
        assert_eq!(f64::from(p32::from(0b10111001101101111000000100101011)), -3.141592651605606);     // p32_3 Default Negative
        assert_eq!(p32::from(0b01101001001000011111101101010100).as_float_es(0), 3.141592651605606);  // p32_0 Custom ES Special-Case
        assert_eq!(p32::from(0b10010110110111100000010010101100).as_float_es(0), -3.141592651605606); // p32_0 Custom ES Special-Case Negative
        assert_eq!(p32::from(0b01011001001000011111101101010100).as_float_es(1), 3.141592651605606);  // p32_1 Custom ES
        assert_eq!(p32::from(0b10100110110111100000010010101100).as_float_es(1), -3.141592651605606); // p32_1 Custom ES Negative
        // assert_eq!(f64::from(p16::from()), );
        // assert_eq!(f64::from(p16::from()), );
        // assert_eq!(f64::from(p16::from()), );
        // assert_eq!(f64::from(p16::from()), );
        // assert_eq!(f64::from(p16::from()), );
        // assert_eq!(f64::from(p16::from()), );
        // assert_eq!(f64::from(p16::from()), );
        // assert_eq!(f64::from(p16::from()), );
    }
}