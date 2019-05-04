use std::io;
use ::envvars;

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub http_proxy_address: Option<String>,
    pub https_proxy_address: Option<String>,
    pub bypass_list: Option<String>,
}

pub fn empty_config() -> ProxyConfig {
    ProxyConfig {
        http_proxy_address: None,
        https_proxy_address: None,
        bypass_list: None,
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
        http_proxy_address: http_proxy,
        https_proxy_address: https_proxy,
        bypass_list: no_proxy,
    })
}

pub fn get_machine() -> io::Result<ProxyConfig> {
    get(envvars::get_machine)
}

pub fn get_user() -> io::Result<ProxyConfig> {
    get(envvars::get_user)
}

pub fn get_user_with_machine_fallback() -> io::Result<ProxyConfig> {
    get(envvars::get_user_with_machine_fallback)
}
