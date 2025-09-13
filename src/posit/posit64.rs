/// Define a generic 64-bit Posit
#[expect(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct p64 {
    pub bits: u64
}

impl From<p64> for f32 {
    fn from(value: p64) -> Self {
        p64::as_float(&value) as f32
    }
}

impl From<&p64> for f32 {
    fn from(value: &p64) -> Self {
        f32::from(*value)
    }
}

impl From<p64> for f64 {
    fn from(value: p64) -> Self {
        p64::as_float(&value)
    }
}

impl From<&p64> for f64 {
    fn from(value: &p64) -> Self {
        f64::from(*value)
    }
}

impl From<u64> for p64 {
    fn from(bits: u64) -> Self {
        p64 { bits }
    }
}

impl From<&u64> for p64 {
    fn from(value: &u64) -> Self {
        p64::from(*value)
    }
}

impl p64 {
    const DES: u64 = 4; // Default e_s = 4 - Highest precise es

    /// Get the two's complement
    fn twos_comp(bits: u64) -> u64 {
        !(bits) + 1
    }

    /// Internal Sign Function
    fn int_sign(bits: u64) -> i8 {
        if bits == 0 { 0 } // 0 Case
        else if bits & 0x8000000000000000 == 0x8000000000000000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Get the sign of the current Posit
    pub fn sign(&self) -> i8 {
        p64::int_sign(self.bits)
    }

    /// Internal Exponent Bits Distance Function
    fn int_to_exp(bits: u64) -> u64 {
        let trunc = bits << 1; // Remove Sign
        let t: u64;
        if trunc & 0x8000000000000000 == 0x8000000000000000 { // Check if using leading zeroes or ones
            t = trunc.leading_ones() as u64;
        } else {
            t = trunc.leading_zeros() as u64;
        }
        t + 2 // Account for sign and regime tail
    }

    /// Get the distance to the Exponent Bits of the current Posit
    pub fn to_exp(&self) -> u64 {
        p64::int_to_exp(self.bits)
    }

    /// Internal Components Function
    fn int_components(bits: u64, es: u64) -> [u64;4] {
        let sn = p64::int_sign(bits);
        let exp_len = p64::int_to_exp(bits);
        if sn == 0 { 
            return [0u64;4]; // Ignore 0 case - Cancel and move on
        }
        let reg = exp_len - 2; // Regime is the number of 0s / 1s...
        let exp: u64; // Pull out exponent bits
        if es == 0 {
            exp = 0;
        } else {
            exp = (bits << exp_len) >> (63 - es) + 1;
        }
        let frac_shift = exp_len + es;
        let frc = (bits << frac_shift) >> frac_shift; // Pull out fractional bits
        [reg, es, exp, frc]
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn components(&self) -> [u64;4] {
        p64::int_components(self.bits, p64::DES)
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn components_es(&self, es: u64) -> [u64;4] {
        p64::int_components(self.bits, es)
    }

    /// Internal To-Float Function
    fn int_as_float(bits: u64, es: u64) -> f64 {
        let comp = p64::int_components(bits, es);
        let reg: f64;
        let exp: f64;
        if es == 0 { // Special case for es 0
            reg = 1f64;
            exp = 2f64;
        } else {
            reg = f64::powf(256f64, -((comp[0] - 1) as f64)); // 256^-regime
            exp = f64::powf(2f64, comp[2] as f64); // 2^exponent
        }
        let frc = 1f64 + (comp[3] as f64) / (f64::powf(2f64, (64 - comp[0] - 2 - comp[1]) as f64));
        reg * exp * frc
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn as_float(&self) -> f64 {
        if self.sign() == -1 {
            -p64::int_as_float(p64::twos_comp(self.bits), p64::DES)
        } else {
            p64::int_as_float(self.bits, p64::DES)
        }
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn as_float_es(&self, es: u64) -> f64 {
        if self.sign() == -1 {
            -p64::int_as_float(p64::twos_comp(self.bits), es)
        } else {
            p64::int_as_float(self.bits, es)
        }
    }
}