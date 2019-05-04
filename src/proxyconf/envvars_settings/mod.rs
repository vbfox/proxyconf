use std::io;
use ::envvars;

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub http_proxy_address: String,
    pub https_proxy_address: String,
    pub bypass_list: String,
}

pub fn empty_config() -> ProxyConfig {
    ProxyConfig {
        http_proxy_address: String::from(""),
        https_proxy_address: String::from(""),
        bypass_list: String::from(""),
    }
}

const HTTP_PROXY: &'static str = "HTTP_PROXY";
const HTTPS_PROXY: &'static str = "HTTPS_PROXY";
const NO_PROXY: &'static str = "NO_PROXY";

fn get<G>(getter: G) -> io::Result<ProxyConfig>
where
    G: Fn(&str) -> io::Result<Option<String>>
{
    let http_proxy = getter(HTTP_PROXY)?;
    let https_proxy = getter(HTTPS_PROXY)?;
    let no_proxy = getter(NO_PROXY)?;

    Ok(ProxyConfig {
        http_proxy_address: http_proxy.unwrap_or(String::from("")),
        https_proxy_address: https_proxy.unwrap_or(String::from("")),
        bypass_list: no_proxy.unwrap_or(String::from("")),
    })
}

pub fn get_user() -> io::Result<ProxyConfig> {
    get(envvars::get_user_with_machine_fallback)
}

pub fn get_machine() -> io::Result<ProxyConfig> {
    get(envvars::get_machine)
}