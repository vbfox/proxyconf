use failure::Fail;

#[derive(Debug, Fail)]
pub enum RegistryError {
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] ::std::io::Error),
}

impl From<::std::io::Error> for RegistryError {
    fn from(error: ::std::io::Error) -> RegistryError {
        RegistryError::Io(error)
    }
}

use super::types;
use crate::registry_helpers::*;
use winreg::enums::*;
use winreg::RegKey;

const KEY_PATH: &'static str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";

fn open_key(write: bool) -> Result<RegKey, RegistryError> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let access = if write { KEY_ALL_ACCESS } else { KEY_READ };
    let key = hkcu.open_subkey_with_flags(KEY_PATH, access)?;
    return Ok(key);
}

fn bool_to_u32(b: bool) -> u32 {
    return match b {
        false => 0u32,
        true => 1u32,
    };
}

pub fn write(config: &types::ProxyConfig) -> Result<(), RegistryError> {
    let key = open_key(true)?;
    key.set_value("ProxyEnable", &bool_to_u32(config.use_manual_proxy))?;

    match config.manual_proxy_address.as_ref() {
        "" => {
            let _ = key.delete_value("ProxyServer");
        }
        _ => key.set_value("ProxyServer", &config.manual_proxy_address)?,
    }

    match config.manual_proxy_bypass_list.as_ref() {
        "" => {
            let _ = key.delete_value("ProxyOverride");
        }
        _ => key.set_value("ProxyOverride", &config.manual_proxy_bypass_list)?,
    }

    match &config.setup_script_address {
        Some(address) => key.set_value("AutoConfigURL", address)?,
        None => {
            let _ = key.delete_value("AutoConfigURL");
        }
    }

    // The seem to indicate if the proxy first time setup been made. Might have been applicable when
    // IE wasn't delivered in Windows as now it's always 1.
    key.set_value("MigrateProxy", &1u32)?;

    return Ok(());
}

pub fn read() -> Result<types::ProxyConfig, RegistryError> {
    let key = open_key(false)?;

    let proxy_enable = get_optional_u32(&key, "ProxyEnable")?;
    let proxy_server = get_optional_string(&key, "ProxyServer")?;
    let proxy_override = get_optional_string(&key, "ProxyOverride")?;
    let setup_script_address = get_optional_string(&key, "AutoConfigURL")?;

    return Ok(types::ProxyConfig {
        setup_script_address,
        use_manual_proxy: match proxy_enable.unwrap_or(0) {
            0 => false,
            _ => true,
        },
        manual_proxy_address: proxy_server.unwrap_or(String::from("")),
        manual_proxy_bypass_list: proxy_override.unwrap_or(String::from("")),
    });
}
