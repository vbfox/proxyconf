#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate byteorder;
extern crate winreg;

mod registry_helpers;
pub mod ie;
pub mod winhttp;
