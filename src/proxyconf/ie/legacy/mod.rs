//! Legacy Internet Explorer settings, not used anymore by Microsoft but still filled in the
//! registry to maintain compatibility with applications that might use it.
//!
//! Stored in values under
//! `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Internet Settings`

mod types;
pub use self::types::*;

pub mod registry;
