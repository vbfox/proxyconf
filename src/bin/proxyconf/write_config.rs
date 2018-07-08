use proxyconf::ie;

pub fn ie_modern(config: &ie::modern::ProxyConfig) {
    if config.automatically_detect_settings {
        println!("    Automatically detect settings");
    } else if config.use_setup_script && config.setup_script_address.len() > 0 {
        println!("    Setup script address: {}", config.setup_script_address);
    } else if config.use_manual_proxy && config.manual_proxy_address.len() > 0 {
        println!("    Manual proxy: {}", config.manual_proxy_address);
        println!("    Bypass list: {}", config.manual_proxy_bypass_list);
    } else {
        println!("    No proxy configured");
    }
}

pub fn ie_legacy(config: &ie::legacy::ProxyConfig) {
    match &config.setup_script_address {
        Some(address) => println!("    Setup script address: {}", address),
        None => {
            if config.use_manual_proxy && config.manual_proxy_address.len() > 0 {
                println!("    Manual proxy: {}", config.manual_proxy_address);
                println!("    Bypass list: {}", config.manual_proxy_bypass_list);
            } else {
                println!("    No proxy configured");
            }
        }
    }
}
