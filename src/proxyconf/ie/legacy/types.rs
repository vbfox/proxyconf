#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub setup_script_address: Option<String>,
    pub use_manual_proxy: bool,
    pub manual_proxy_address: String,
    pub manual_proxy_bypass_list: String,
}

pub fn empty_config() -> ProxyConfig {
    ProxyConfig {
        setup_script_address: None,
        use_manual_proxy: false,
        manual_proxy_address: String::from(""),
        manual_proxy_bypass_list: String::from(""),
    }
}
