#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;
extern crate byteorder;
extern crate winreg;

pub mod internet_settings;
pub mod envvars_settings;
mod registry_helpers;
mod envvars;
mod string_serialization;

#[cfg(test)]
mod hex;
