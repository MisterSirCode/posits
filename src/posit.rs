pub mod util;

#[derive(Copy, Clone, Debug)]
pub struct p16 {
    b: u16
}

impl From<f16> for p16 {
    fn from(value: f16) -> Self {
        
    }
}

impl p16 {
    pub fn sign(&self) -> i8 {
        if self.b == 0 { 0 }
        else if self.b & 0x10000 > 0 { -1 }
        else { 1 }
    }

    fn leading_zeroes(&self) -> u16 {
        self.b.leading_zeros().try_into().unwrap()
    }

    fn leading_ones(&self) -> u16 {
        self.b.leading_ones().try_into().unwrap()
    }

    fn to_exp(&self) -> u16 {

    }

    fn exponent(&self) -> u16 {
        let s = self.sign();
        if s == 0 {
            0
        } else if s == -1 {
            (self.b << self.leading_ones())
        } else {

        }
    }
}