use proxyconf::ie;
use write_config;

fn set_config(config: &ie::modern::ProxyConfig) {
    ie::modern::registry::write(config.clone()).unwrap();

    print!("Configuration changed to: ");
    write_config::ie_modern(&config);
}

pub fn set_server(server: &str, bypass_list: &str) {
    set_config(&ie::modern::ProxyConfig {
        use_manual_proxy: true,
        manual_proxy_address: server.into(),
        manual_proxy_bypass_list: bypass_list.into(),
        ..ie::modern::empty_config()
    });
}

pub fn set_setup_script(setupscript: &str) {
    set_config(&ie::modern::ProxyConfig {
        use_setup_script: true,
        setup_script_address: setupscript.into(),
        ..ie::modern::empty_config()
    });
}

pub fn set_auto_detect() {
    set_config(&ie::modern::ProxyConfig {
        automatically_detect_settings: true,
        ..ie::modern::empty_config()
    });
}

pub fn set_no_proxy() {
    set_config(&ie::modern::ProxyConfig {
        ..ie::modern::empty_config()
    });
}
