#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub http_proxy_address: String,
    pub https_proxy_address: String,
    pub manual_proxy_bypass_list: String,
}

pub fn empty_config() -> ProxyConfig {
    ProxyConfig {
        http_proxy_address: String::from(""),
        https_proxy_address: String::from(""),
        manual_proxy_bypass_list: String::from(""),
    }
}

