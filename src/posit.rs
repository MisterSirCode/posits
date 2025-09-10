use std::{convert, fmt::{Display, Formatter, Result}};

pub mod util;

#[derive(Copy, Clone, Debug)]
pub struct p16 {
    pub bits: u16
}

// impl From<p16> for f16 {
//     fn from(value: p16) -> Self {
//         let comp = value.components();
//         let reg = f16::powf(256f16, -(comp[0] as f16)); // 256^-regime
//         let exp = f16::powf(2f16, comp[1] as f16); // 2^exponent
//         reg * exp * (1f16 + (comp[2] as f16) / 256f16)
//     }
// }

impl From<p16> for f32 {
    fn from(value: p16) -> Self {
        let comp = value.components();
        let reg = f32::powf(256f32, -(comp[0] as f32)); // 256^-regime
        let exp = f32::powf(2f32, comp[1] as f32); // 2^exponent
        reg * exp * (1f32 + (comp[2] as f32) / 256f32)
    }
}

impl From<&p16> for f32 {
    fn from(value: &p16) -> Self {
        f32::from(*value)
    }
}

impl From<p16> for f64 {
    fn from(value: p16) -> Self {
        let comp = value.components();
        let reg = f64::powf(256f64, -(comp[0] as f64)); // 256^-regime
        let exp = f64::powf(2f64, comp[1] as f64); // 2^exponent
        reg * exp * (1f64 + (comp[2] as f64) / 256f64)
    }
}

impl From<&p16> for f64 {
    fn from(value: &p16) -> Self {
        f64::from(*value)
    }
}

impl Display for p16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:}p16", f64::from(self))
    }
}

impl p16 {
    pub fn manual(bits: u16) -> Self {
        p16 { bits }
    }

    pub fn sign(&self) -> i8 {
        if self.bits == 0 { 0 } // 0 Case
        else if self.bits & 0x4000 == 0x4000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    fn to_exp(&self) -> u16 {
        let trunc = self.bits << 1; // Remove Sign
        let t: u16;
        if trunc & 0x4000 == 0x4000 { // Check if using leading zeroes or ones
            t = trunc.leading_ones() as u16;
        } else {
            t = trunc.leading_zeros() as u16;
        }
        t + 2 // Account for sign and regime tail
    }

    pub fn components(&self) -> [u8;3] {
        let sn = self.sign();
        let len = self.to_exp();
        if sn == 0 { 
            return [0u8;3]; // Ignore 0 case - Cancel and move on
        }
        let reg: u8 = (len - 2) as u8; // Regime is the number of 0s / 1s...
        let exp: u8 = ((self.bits << len) >> len + 8) as u8; // Pull out exponent bits
        let frc: u8 = ((self.bits << 8) >> 8) as u8; // Pull out fractional bits
        [reg, exp, frc]
    }
}