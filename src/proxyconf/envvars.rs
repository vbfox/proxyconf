//! Allow access to the user and machine environment variable block

use super::registry_helpers;
use std::io;
use winreg::enums::*;
use winreg::RegKey;

const MACHINE_PATH: &'static str =
    "System\\CurrentControlSet\\Control\\Session Manager\\Environment";

const USER_PATH: &'static str = "Environment";

/// Get an environment variable from the machine store
pub fn get_machine(name: &str) -> io::Result<Option<String>> {
    let key = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(MACHINE_PATH, KEY_READ)?;
    return registry_helpers::get_optional_string(&key, name);
}

/// Get an environment variable from the current user store
pub fn get_user(name: &str) -> io::Result<Option<String>> {
    let key = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(USER_PATH, KEY_READ)?;
    return registry_helpers::get_optional_string(&key, name);
}

/// Get an environment variable from the current user store falling back to the machine store if not
/// found
pub fn get_user_with_machine_fallback(name: &str) -> io::Result<Option<String>> {
    let user_value = get_user(name)?;
    match user_value {
        Some(value) => return Ok(Some(value)),
        None => return get_machine(name),
    }
}
