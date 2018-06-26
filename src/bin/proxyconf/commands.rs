use write_config;
use proxyconf::ie;

fn set_config(config: &ie::ProxyConfig) {
    ie::registry::write(config.clone()).unwrap();

    print!("Configuration changed to: ");
    write_config::to_stdout(&config);
}

pub fn set_server(server: &str, bypass_list: &str) {
    set_config(&ie::ProxyConfig {
        use_manual_proxy: true,
        manual_proxy_address: server.into(),
        manual_proxy_bypass_list: bypass_list.into(),
        ..ie::empty_config()
    });
}

pub fn set_setup_script(setupscript: &str) {
    set_config(&ie::ProxyConfig {
        use_setup_script: true,
        setup_script_address: setupscript.into(),
        ..ie::empty_config()
    });
}

pub fn set_auto_detect() {
    set_config(&ie::ProxyConfig {
        automatically_detect_settings: true,
        ..ie::empty_config()
    });
}

pub fn set_no_proxy() {
    set_config(&ie::ProxyConfig {
        ..ie::empty_config()
    });
}
