pub mod posit;
use posit::p64;
use posit::PositUtils;

fn f32tou32(f: f32) -> u32 {
    f32::to_bits(f)
}

fn main() {
    PositUtils::unit_tests(); // Validate Operations
}
