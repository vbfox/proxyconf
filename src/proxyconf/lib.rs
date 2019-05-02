#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;
extern crate byteorder;
extern crate winreg;

pub mod internet_settings;
mod registry_helpers;
mod string_serialization;

#[cfg(test)]
mod hex;
