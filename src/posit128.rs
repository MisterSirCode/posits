/// Define a generic 64-bit Posit
/// Warning: p128's do not have a float128 conversion - Rust f128 support is experimental
#[expect(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub struct p128 {
    /// The raw bits of the posit
    pub bits: u128
}

impl From<p128> for f32 {
    fn from(value: p128) -> Self {
        p128::as_float(&value) as f32
    }
}

impl From<&p128> for f32 {
    fn from(value: &p128) -> Self {
        f32::from(*value)
    }
}

impl From<p128> for f64 {
    fn from(value: p128) -> Self {
        p128::as_float(&value)
    }
}

impl From<&p128> for f64 {
    fn from(value: &p128) -> Self {
        f64::from(*value)
    }
}

impl From<u128> for p128 {
    fn from(bits: u128) -> Self {
        p128 { bits }
    }
}

impl From<&u128> for p128 {
    fn from(value: &u128) -> Self {
        p128::from(*value)
    }
}

impl p128 {
    /// Default es value. Highest precision es available for this type
    pub const DES: u128 = 5;
    /// The Posit equivalent of Pi for this type
    pub const PI: p128 = p128 { bits: 0b01000011001001000011111101101010_10001000100001011010001100000000 };

    /// Get the two's complement
    fn twos_comp(bits: u128) -> u128 {
        !(bits) + 1
    }

    /// Internal Sign Function
    fn int_sign(bits: u128) -> i8 {
        if bits == 0 { 0 } // 0 Case
        else if bits & 0x80000000000000000000000000000000 == 0x80000000000000000000000000000000 { -1 } // Match MSB - Negative
        else { 1 } // Positive
    }

    /// Get the sign of the current Posit
    pub fn sign(&self) -> i8 {
        p128::int_sign(self.bits)
    }

    /// Internal Exponent Bits Distance Function
    fn int_to_exp(bits: u128) -> u128 {
        let trunc = bits << 1; // Remove Sign
        let t: u128 = if trunc & 0x80000000000000000000000000000000 == 0x80000000000000000000000000000000 { // Check if using leading zeroes or ones
            trunc.leading_ones() as u128
        } else {
            trunc.leading_zeros() as u128
        };
        t + 2 // Account for sign and regime tail
    }

    /// Get the distance to the Exponent Bits of the current Posit
    pub fn to_exp(&self) -> u128 {
        p128::int_to_exp(self.bits)
    }

    /// Internal Components Function
    fn int_components(bits: u128, es: u128) -> [u128;4] {
        let sn = p128::int_sign(bits);
        let exp_len = p128::int_to_exp(bits);
        if sn == 0 { 
            return [0u128;4]; // Ignore 0 case - Cancel and move on
        }
        let reg = exp_len - 2; // Regime is the number of 0s / 1s...
        let exp: u128 = if es == 0 { 0 } else {
            (bits << exp_len) >> ((127 - es) + 1) // Pull out exponent bits
        };
        let frac_shift = exp_len + es;
        let frc = (bits << frac_shift) >> frac_shift; // Pull out fractional bits
        [reg, es, exp, frc]
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn components(&self) -> [u128;4] {
        p128::int_components(self.bits, p128::DES)
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn components_es(&self, es: u128) -> [u128;4] {
        p128::int_components(self.bits, es)
    }

    /// Internal To-Float Function
    fn int_as_float(bits: u128, es: u128) -> f64 {
        let comp = p128::int_components(bits, es);
        let reg: f64;
        let exp: f64;
        if es == 0 { // Special case for es 0
            reg = 1f64;
            exp = 2f64;
        } else {
            reg = f64::powf(256f64, -((comp[0] - 1) as f64)); // 256^-regime
            exp = f64::powf(2f64, comp[2] as f64); // 2^exponent
        }
        let frc = 1f64 + (comp[3] as f64) / (f64::powf(2f64, (128 - comp[0] - 2 - comp[1]) as f64));
        reg * exp * frc
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn as_float(&self) -> f64 {
        if self.sign() == -1 {
            -p128::int_as_float(p128::twos_comp(self.bits), p128::DES)
        } else {
            p128::int_as_float(self.bits, p128::DES)
        }
    }

    /// Get the Components of the current Posit with a supplied es
    pub fn as_float_es(&self, es: u128) -> f64 {
        if self.sign() == -1 {
            -p128::int_as_float(p128::twos_comp(self.bits), es)
        } else {
            p128::int_as_float(self.bits, es)
        }
    }
}