use proxyconf::ie;

use write_config;

fn set_ie_modern_config(config: &ie::modern::ProxyConfig) {
    let location = ie::modern::registry::get_current_user_location();
    ie::modern::registry::write(&location, config.clone()).unwrap();

    println!("Configuration changed to: ");
    write_config::ie_modern(&config);
}

pub fn set_server(server: &str, bypass_list: &str) {
    set_ie_modern_config(&ie::modern::ProxyConfig {
        use_manual_proxy: true,
        manual_proxy_address: server.into(),
        manual_proxy_bypass_list: bypass_list.into(),
        ..ie::modern::empty_config()
    });
}

pub fn set_setup_script(setup_script: &str) {
    set_ie_modern_config(&ie::modern::ProxyConfig {
        use_setup_script: true,
        setup_script_address: setup_script.into(),
        ..ie::modern::empty_config()
    });
}

pub fn set_auto_detect() {
    set_ie_modern_config(&ie::modern::ProxyConfig {
        automatically_detect_settings: true,
        ..ie::modern::empty_config()
    });
}

pub fn set_no_proxy() {
    set_ie_modern_config(&ie::modern::ProxyConfig {
        ..ie::modern::empty_config()
    });
}

pub fn winhttp_show() {
    let location = ie::modern::registry::get_winhttp_location();
    let conf = ie::modern::registry::read(&location).unwrap();
    write_config::ie_modern(&conf);
}

pub fn ie_modern_show() {
    let location = ie::modern::registry::get_current_user_location();
    let conf = ie::modern::registry::read(&location).unwrap();
    write_config::ie_modern(&conf);
}

pub fn show() {
    println!("Internet explorer: ");
    ie_modern_show();

    println!();
    println!("Internet explorer (legacy): ");
    let legacy_conf = ie::legacy::registry::read().unwrap();
    write_config::ie_legacy(&legacy_conf);

    println!();
    println!("WinHTTP (System wide): ");
    winhttp_show();
}

fn set_winhttp_config(config: &ie::modern::ProxyConfig) {
    let location = ie::modern::registry::get_winhttp_location();
    let full_config = ie::modern::FullConfig {
        version: ie::modern::WINHTTP_VERSION,
        counter: 0,
        config: config.clone(),
    };
    ie::modern::registry::write_full(&location, &full_config).unwrap();

    println!("Configuration changed to: ");
    write_config::ie_modern(&config);
}

pub fn winhttp_set_server(server: &str, bypass_list: &str) {
    set_winhttp_config(&ie::modern::ProxyConfig {
        use_manual_proxy: true,
        manual_proxy_address: server.into(),
        manual_proxy_bypass_list: bypass_list.into(),
        ..ie::modern::empty_config()
    });
}

pub fn winhttp_set_no_proxy() {
    set_winhttp_config(&ie::modern::empty_config());
}