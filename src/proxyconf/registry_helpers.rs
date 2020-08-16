//! Small helpers for winreg that could be contributed one day

use std::io;
use winreg::RegKey;

pub fn get_optional_string(key: &RegKey, name: &str) -> io::Result<Option<String>> {
    let raw: io::Result<String> = key.get_value(name);
    match raw {
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
        Ok(x) => Ok(Some(x)),
    }
}

pub fn get_optional_u32(key: &RegKey, name: &str) -> io::Result<Option<u32>> {
    let raw: io::Result<u32> = key.get_value(name);
    match raw {
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
        Ok(x) => Ok(Some(x)),
    }
}
