#![recursion_limit = "1024"]

mod envvars;
pub mod envvars_settings;
pub mod internet_settings;
mod registry_helpers;
mod string_serialization;

#[cfg(test)]
mod hex;
