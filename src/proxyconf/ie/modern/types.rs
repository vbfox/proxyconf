#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub automatically_detect_settings: bool,
    pub use_setup_script: bool,
    pub setup_script_address: String,
    pub use_manual_proxy: bool,
    pub manual_proxy_address: String,
    pub manual_proxy_bypass_list: String,
}

pub fn empty_config() -> ProxyConfig {
    ProxyConfig {
        automatically_detect_settings: false,
        use_setup_script: false,
        setup_script_address: String::from(""),
        use_manual_proxy: false,
        manual_proxy_address: String::from(""),
        manual_proxy_bypass_list: String::from(""),
    }
}

#[derive(Debug, Clone)]
pub struct FullConfig {
    pub version: u32,
    pub counter: u32,
    pub config: ProxyConfig,
}

pub fn empty_full_config() -> FullConfig {
    FullConfig {
        version: super::IE7_VERSION,
        counter: 0,
        config: empty_config(),
    }
}