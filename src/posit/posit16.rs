/// Define a generic 16-bit Posit
#[derive(Copy, Clone, Debug)]
pub struct p16 {
    pub bits: u16
}

impl From<p16> for f32 {
    fn from(value: p16) -> Self {
        p16::as_float(&value) as f32
    }
}

impl From<&p16> for f32 {
    fn from(value: &p16) -> Self {
        f32::from(*value)
    }
}

impl From<p16> for f64 {
    fn from(value: p16) -> Self {
        p16::as_float(&value)
    }
}

impl From<&p16> for f64 {
    fn from(value: &p16) -> Self {
        f64::from(*value)
    }
}

impl From<u16> for p16 {
    fn from(bits: u16) -> Self {
        p16 { bits }
    }
}

impl From<&u16> for p16 {
    fn from(value: &u16) -> Self {
        p16::from(*value)
    }
}

impl p16 {
    const DES: u16 = 2; // Default e_s = 2 - Highest precise ES

    /// Get the two's complement
    fn twos_comp(bits: u16) -> u16 {
        !(bits) + 1
    }

    /// Internal Sign Function
    fn int_sign(bits: u16) -> i8 {
        if bits == 0 { 0 } // 0 Case
        else if bits & 0x8000 == 0x8000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Get the sign of the current Posit
    pub fn sign(&self) -> i8 {
        p16::int_sign(self.bits)
    }

    /// Internal Exponent Bits Distance Function
    fn int_to_exp(bits: u16) -> u16 {
        let trunc = bits << 1; // Remove Sign
        let t: u16;
        if trunc & 0x8000 == 0x8000 { // Check if using leading zeroes or ones
            t = trunc.leading_ones() as u16;
        } else {
            t = trunc.leading_zeros() as u16;
        }
        t + 2 // Account for sign and regime tail
    }

    /// Get the distance to the Exponent Bits of the current Posit
    pub fn to_exp(&self) -> u16 {
        p16::int_to_exp(self.bits)
    }

    /// Internal Components Function
    fn int_components(bits: u16, ES: u16) -> [u16;4] {
        let sn = p16::int_sign(bits);
        let exp_len = p16::int_to_exp(bits);
        if sn == 0 { 
            return [0u16;4]; // Ignore 0 case - Cancel and move on
        }
        let reg = exp_len - 2; // Regime is the number of 0s / 1s...
        let exp: u16; // Pull out exponent bits
        if ES == 0 {
            exp = 0;
        } else {
            exp = (bits << exp_len) >> (15 - ES) + 1;
        }
        let frac_shift = exp_len + ES;
        let frc = (bits << frac_shift) >> frac_shift; // Pull out fractional bits
        [reg, ES, exp, frc]
    }

    /// Get the Components of the current Posit with a supplied ES
    pub fn components(&self) -> [u16;4] {
        p16::int_components(self.bits, p16::DES)
    }

    /// Get the Components of the current Posit with a supplied ES
    pub fn components_es(&self, ES: u16) -> [u16;4] {
        p16::int_components(self.bits, ES)
    }

    /// Internal To-Float Function
    fn int_as_float(bits: u16, ES: u16) -> f64 {
        let comp = p16::int_components(bits, ES);
        let reg: f64;
        let exp: f64;
        if ES == 0 { // Special case for ES 0
            reg = 1f64;
            exp = 2f64;
        } else {
            reg = f64::powf(256f64, -((comp[0] - 1) as f64)); // 256^-regime
            exp = f64::powf(2f64, comp[2] as f64); // 2^exponent
        }
        let frc = 1f64 + (comp[3] as f64) / (f64::powf(2f64, (16 - comp[0] - 2 - comp[1]) as f64));
        reg * exp * frc
    }

    /// Get the Components of the current Posit with a supplied ES
    pub fn as_float(&self) -> f64 {
        if self.sign() == -1 {
            -p16::int_as_float(p16::twos_comp(self.bits), p16::DES)
        } else {
            p16::int_as_float(self.bits, p16::DES)
        }
    }

    /// Get the Components of the current Posit with a supplied ES
    pub fn as_float_es(&self, ES: u16) -> f64 {
        if self.sign() == -1 {
            -p16::int_as_float(p16::twos_comp(self.bits), ES)
        } else {
            p16::int_as_float(self.bits, ES)
        }
    }
}