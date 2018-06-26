//! Internet Explorer settings, considered as the system settings since Windows 8 and shown in the
//! Control Panel.
//!
//! Stored in a binary value named `DefaultConnectionSettings` in key
//! `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings\Connections`

mod types;
pub use self::types::*;

pub mod registry;
pub mod serialization;
