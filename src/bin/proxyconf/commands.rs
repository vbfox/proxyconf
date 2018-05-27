use proxyconf;
use write_config;

fn set_config(config: &proxyconf::ProxyConfig) {
    proxyconf::registry::write(config.clone()).unwrap();

    print!("Configuration changed to: ");
    write_config::to_stdout(&config);
}

pub fn set_server(server: &str, bypass_list: &str) {
    set_config(&proxyconf::ProxyConfig {
        use_manual_proxy: true,
        manual_proxy_address: server.into(),
        manual_proxy_bypass_list: bypass_list.into(),
        ..proxyconf::empty()
    });
}

pub fn set_setup_script(setupscript: &str) {
    set_config(&proxyconf::ProxyConfig {
        use_setup_script: true,
        setup_script_address: setupscript.into(),
        ..proxyconf::empty()
    });
}

pub fn set_auto_detect() {
    set_config(&proxyconf::ProxyConfig {
        automatically_detect_settings: true,
        ..proxyconf::empty()
    });
}

pub fn set_no_proxy() {
    set_config(&proxyconf::ProxyConfig {
        ..proxyconf::empty()
    });
}
