#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate byteorder;
extern crate winreg;

mod types;
pub use self::types::*;

pub mod registry;
pub mod serialization;
