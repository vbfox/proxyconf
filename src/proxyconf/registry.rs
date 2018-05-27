mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }

        links {
            Serialization(::serialization::Error, ::serialization::ErrorKind);
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

use serialization;
use types;
use winreg::enums::*;
use winreg::{RegKey, RegValue};

const KEY_PATH: &'static str =
    "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings\\Connections";
const VALUE_NAME: &'static str = "DefaultConnectionSettings";

fn open_key(write: bool) -> Result<RegKey> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let access = if write { KEY_ALL_ACCESS } else { KEY_READ };
    let key = hkcu.open_subkey_with_flags(KEY_PATH, access)?;
    return Ok(key);
}

fn write_raw(bytes: Vec<u8>) -> Result<()> {
    let value = RegValue {
        vtype: REG_BINARY,
        bytes,
    };
    let key = open_key(true)?;
    key.set_raw_value(VALUE_NAME, &value)?;
    return Ok(());
}

pub fn write_full(config: &types::FullConfig) -> Result<()> {
    let mut bytes = Vec::new();
    serialization::serialize(config, &mut bytes)?;
    write_raw(bytes)?;
    return Ok(());
}

fn read_raw() -> Result<Vec<u8>> {
    let key = open_key(false)?;
    let value = key.get_raw_value(VALUE_NAME)?;

    match value.vtype {
        REG_BINARY => Ok(value.bytes),
        _ => Err(ErrorKind::InvalidValueType.into()),
    }
}

pub fn read_full() -> Result<types::FullConfig> {
    let bytes = read_raw()?;
    let conf = serialization::deserialize(&bytes[..])?;
    return Ok(conf);
}

pub fn read() -> Result<types::ProxyConfig> {
    return Ok(read_full()?.config);
}

pub fn write(config: types::ProxyConfig) -> Result<()> {
    let full_before = read_full()?;
    let full_after = types::FullConfig {
        counter: full_before.counter + 1,
        config,
    };
    write_full(&full_after)?;

    Ok(())
}

pub fn update<F>(updater: F) -> Result<()>
where
    F: FnOnce(types::ProxyConfig) -> types::ProxyConfig,
{
    let full_before = read_full()?;
    let after = updater(full_before.config);

    let full_after = types::FullConfig {
        counter: full_before.counter + 1,
        config: after,
    };
    write_full(&full_after)?;

    Ok(())
}
