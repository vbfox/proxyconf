#[macro_use]
extern crate error_chain;
extern crate winreg;
extern crate byteorder;

mod types;
pub use self::types::*;

pub mod registry;
pub mod serialization;