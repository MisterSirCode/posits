/// Define a generic 32-bit Posit
#[derive(Copy, Clone, Debug)]
#[expect(non_camel_case_types)]
pub struct p32 {
    pub bits: u32
}

impl From<p32> for f32 {
    fn from(value: p32) -> Self {
        p32::as_float(&value) as f32
    }
}

impl From<&p32> for f32 {
    fn from(value: &p32) -> Self {
        f32::from(*value)
    }
}

impl From<p32> for f64 {
    fn from(value: p32) -> Self {
        p32::as_float(&value)
    }
}

impl From<&p32> for f64 {
    fn from(value: &p32) -> Self {
        f64::from(*value)
    }
}

impl From<u32> for p32 {
    fn from(bits: u32) -> Self {
        p32 { bits }
    }
}

impl From<&u32> for p32 {
    fn from(value: &u32) -> Self {
        p32::from(*value)
    }
}

impl p32 {
    const DES: u32 = 3; // Default e_s = 3 - Highest precise es

    /// Get the two's complement
    fn twos_comp(bits: u32) -> u32 {
        !(bits) + 1
    }

    /// Internal Sign Function
    fn int_sign(bits: u32) -> i8 {
        if bits == 0 { 0 } // 0 Case
        else if bits & 0x80000000 == 0x80000000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Get the sign of the current Posit
    pub fn sign(&self) -> i8 {
        p32::int_sign(self.bits)
    }

    /// Internal Exponent Bits Distance Function
    fn int_to_exp(bits: u32) -> u32 {
        let trunc = bits << 1; // Remove Sign
        let t: u32;
        if trunc & 0x80000000 == 0x80000000 { // Check if using leading zeroes or ones
            t = trunc.leading_ones() as u32;
        } else {
            t = trunc.leading_zeros() as u32;
        }
        t + 2 // Account for sign and regime tail
    }

    /// Get the distance to the Exponent Bits of the current Posit
    pub fn to_exp(&self) -> u32 {
        p32::int_to_exp(self.bits)
    }

    /// Internal Components Function
    fn int_components(bits: u32, es: u32) -> [u32;4] {
        let sn = p32::int_sign(bits);
        let exp_len = p32::int_to_exp(bits);
        if sn == 0 { 
            return [0u32;4]; // Ignore 0 case - Cancel and move on
        }
        let reg = exp_len - 2; // Regime is the number of 0s / 1s...
        let exp: u32; // Pull out exponent bits
        if es == 0 {
            exp = 0;
        } else {
            exp = (bits << exp_len) >> (31 - es) + 1;
        }
        let frac_shift = exp_len + es;
        let frc = (bits << frac_shift) >> frac_shift; // Pull out fractional bits
        [reg, es, exp, frc]
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn components(&self) -> [u32;4] {
        p32::int_components(self.bits, p32::DES)
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn components_es(&self, es: u32) -> [u32;4] {
        p32::int_components(self.bits, es)
    }

    /// Internal To-Float Function
    fn int_as_float(bits: u32, es: u32) -> f64 {
        let comp = p32::int_components(bits, es);
        let reg: f64;
        let exp: f64;
        if es == 0 { // Special case for es 0
            reg = 1f64;
            exp = 2f64;
        } else {
            reg = f64::powf(256f64, -((comp[0] - 1) as f64)); // 256^-regime
            exp = f64::powf(2f64, comp[2] as f64); // 2^exponent
        }
        let frc = 1f64 + (comp[3] as f64) / (f64::powf(2f64, (32 - comp[0] - 2 - comp[1]) as f64));
        reg * exp * frc
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn as_float(&self) -> f64 {
        if self.sign() == -1 {
            -p32::int_as_float(p32::twos_comp(self.bits), p32::DES)
        } else {
            p32::int_as_float(self.bits, p32::DES)
        }
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn as_float_es(&self, es: u32) -> f64 {
        if self.sign() == -1 {
            -p32::int_as_float(p32::twos_comp(self.bits), es)
        } else {
            p32::int_as_float(self.bits, es)
        }
    }
}