//! Allow access to the user and machine environment variable block

use std::io;
use winreg::RegKey;
use winreg::enums::*;
use super::registry_helpers;

const MACHINE_PATH: &'static str =
    "System\\CurrentControlSet\\Control\\Session Manager\\Environment";

const USER_PATH: &'static str =
    "Environment";

pub fn get_machine(name: &str) -> io::Result<Option<String>> {
    let key = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(MACHINE_PATH, KEY_READ)?;
    return registry_helpers::get_optional_string(&key, name);
}

pub fn get_user(name: &str) -> io::Result<Option<String>> {
    let key = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(USER_PATH, KEY_READ)?;
    return registry_helpers::get_optional_string(&key, name);
}