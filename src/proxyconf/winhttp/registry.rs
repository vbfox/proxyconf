mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }

        links {
            Serialization(super::serialization::Error, super::serialization::ErrorKind);
        }

        errors {
            InvalidValueType {
                description("invalid registry value type"),
                display("invalid registry value type"),
            }
        }
    }
}

pub use self::errors::*;

use super::serialization;
use super::types;
use winreg::enums::*;
use winreg::{RegKey, RegValue};

const KEY_PATH: &'static str =
    "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Internet Settings\\Connections";
const KEY_PATH_WOW6432: &'static str =
    "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Internet Settings\\Connections";
const VALUE_NAME: &'static str = "WinHttpSettings";

fn open_key(write: bool, wow6432: bool) -> Result<RegKey> {
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let access = if write { KEY_ALL_ACCESS } else { KEY_READ };
    let key_path = if wow6432 { KEY_PATH_WOW6432 } else { KEY_PATH };

    let key = hkcu.open_subkey_with_flags(key_path, access)?;

    return Ok(key);
}

fn write_raw(bytes: &Vec<u8>, wow6432: bool) -> Result<()> {
    let value = RegValue {
        vtype: REG_BINARY,
        bytes: bytes.to_owned(),
    };

    let key = open_key(true, wow6432)?;
    key.set_raw_value(VALUE_NAME, &value)?;

    return Ok(());
}

pub fn write(config: &types::ProxyConfig) -> Result<()> {
    let mut bytes = Vec::new();
    serialization::serialize(config, &mut bytes)?;

    write_raw(&bytes, true)?;
    write_raw(&bytes, false)?;

    return Ok(());
}

pub fn read_raw() -> Result<Vec<u8>> {
    let key = open_key(false, false)?;
    let value = key.get_raw_value(VALUE_NAME)?;

    match value.vtype {
        REG_BINARY => Ok(value.bytes),
        _ => Err(ErrorKind::InvalidValueType.into()),
    }
}

pub fn read() -> Result<types::ProxyConfig> {
    let bytes = read_raw()?;
    let conf = serialization::deserialize(&bytes[..])?;
    return Ok(conf);
}
