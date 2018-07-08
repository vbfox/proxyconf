//! Internet Explorer settings, considered as the system settings since Windows 8 and shown in the
//! Control Panel.
//!
//! Stored in a binary value per connection in key
//! `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\Connections`
//!
//! The default/LAN connection settings are stored in a key named `DefaultConnectionSettings`.
//!
//! The same key in HKLM can optionally be used by all users if the value `ProxySettingsPerUser`
//! in `HKEY_LOCAL_MACHINE\Software\Policies\Microsoft\Windows\CurrentVersion\Internet Settings` is 1 (DWORD)
//!
//! System services also use a `WinHttpSettings` key in HKLM that can be set by `proxycfg`/`netsh winhttp` and always
//! has version 0x28 and counter = 0.

/// An old format still used by `proxycfg`/`netsh winhttp`
pub const WINHTTP_VERSION: u32 = 0x28;

/// The format as of IE 6
pub const IE6_VERSION: u32 = 0x3C;

/// The format as of IE 7, still used in IE 11 / Windows 10
pub const IE7_VERSION: u32 = 0x46;

mod types;
pub use self::types::*;

pub mod registry;
pub mod serialization;
