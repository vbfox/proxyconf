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
    "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings\\Connections";
const KEY_PATH_WOW6432: &'static str =
    "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Internet Settings\\Connections";
pub const DEFAULT_CONNECTION_NAME: &'static str = "DefaultConnectionSettings";
pub const WINHTTP_CONNECTION_NAME: &'static str = "WinHttpSettings";

#[derive(Debug, Clone)]
pub enum Target {
    System,
    CurrentUser
}

#[derive(Debug, Clone)]
pub struct Location {
    pub target: Target,
    pub connection_name: String,
}

fn open_key(target: &Target, write: bool, wow6432: bool) -> Result<RegKey> {
    let root_key = match target {
        Target::System => RegKey::predef(HKEY_LOCAL_MACHINE),
        Target::CurrentUser =>  RegKey::predef(HKEY_CURRENT_USER),
    };
    let access = if write { KEY_ALL_ACCESS } else { KEY_READ };
    let key_path = if wow6432 { KEY_PATH_WOW6432 } else { KEY_PATH };
    let key = root_key.open_subkey_with_flags(key_path, access)?;
    return Ok(key);
}

fn write_raw(location: &Location, bytes: &Vec<u8>, wow6432: bool) -> Result<()> {
    let value = RegValue {
        vtype: REG_BINARY,
        bytes: bytes.to_owned(),
    };
    let key = open_key(&location.target, true, wow6432)?;
    key.set_raw_value(&location.connection_name, &value)?;
    return Ok(());
}

pub fn write_full(location: &Location, config: &types::FullConfig) -> Result<()> {
    let mut bytes = Vec::new();
    serialization::serialize(config, &mut bytes)?;

    write_raw(location, &bytes, true)?;
    write_raw(location, &bytes, false)?;

    return Ok(());
}

fn read_raw(location: &Location) -> Result<Vec<u8>> {
    let key = open_key(&location.target, false, false)?;
    let value = key.get_raw_value(&location.connection_name)?;

    match value.vtype {
        REG_BINARY => Ok(value.bytes),
        _ => Err(ErrorKind::InvalidValueType.into()),
    }
}

pub fn read_full(location: &Location) -> Result<types::FullConfig> {
    let bytes = read_raw(location)?;
    let conf = serialization::deserialize(&bytes[..])?;
    return Ok(conf);
}

pub fn get_next_counter(location: &Location) -> u32 {
    let full_result = read_full(location);
    match full_result {
        Ok(full) => full.counter + 1,
        _ => 0
    }
}

pub fn read(location: &Location) -> Result<types::ProxyConfig> {
    return Ok(read_full(location)?.config);
}

pub fn write(location: &Location, config: types::ProxyConfig) -> Result<()> {
    let full_before = read_full(location)?;
    let full_after = types::FullConfig {
        version: super::IE7_VERSION,
        counter: full_before.counter + 1,
        config,
    };
    write_full(location, &full_after)?;

    Ok(())
}

pub fn update<F>(location: &Location, updater: F) -> Result<()>
where
    F: FnOnce(types::ProxyConfig) -> types::ProxyConfig,
{
    let full_before = read_full(location)?;
    let after = updater(full_before.config);

    let full_after = types::FullConfig {
        version: super::IE7_VERSION,
        counter: full_before.counter + 1,
        config: after,
    };
    write_full(location, &full_after)?;

    Ok(())
}
