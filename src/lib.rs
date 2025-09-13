//#![deny(missing_docs)]
/// The `extra` module organizes the types and additional functions involved with the use of Posits
pub mod extra;
mod posit8;
mod posit16;
mod posit32;
mod posit64;
// Expose p8-p64 to top-level crate
pub use posit8::p8;
pub use posit16::p16;
pub use posit32::p32;
pub use posit64::p64;