use proxyconf::ie;
use proxyconf::winhttp;

use write_config;

fn set_ie_modern_config(config: &ie::modern::ProxyConfig) {
    ie::modern::registry::write(config.clone()).unwrap();

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

pub fn set_setup_script(setupscript: &str) {
    set_ie_modern_config(&ie::modern::ProxyConfig {
        use_setup_script: true,
        setup_script_address: setupscript.into(),
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
    let winhttp_conf = winhttp::registry::read().unwrap();
    write_config::winhttp(&winhttp_conf);
}

pub fn show() {
    let conf = ie::modern::registry::read().unwrap();
    println!("Internet explorer: ");
    write_config::ie_modern(&conf);

    println!("");
    println!("Internet explorer (legacy): ");
    let legacy_conf = ie::legacy::registry::read().unwrap();
    write_config::ie_legacy(&legacy_conf);

    println!("");
    println!("WinHTTP (System wide): ");
    winhttp_show();
}

fn set_winhttp_config(config: &winhttp::ProxyConfig) {
    winhttp::registry::write(&config).unwrap();

    println!("Configuration changed to: ");
    write_config::winhttp(&config);
}

pub fn winhttp_set_server(server: &str, bypass_list: &str) {
    set_winhttp_config(&winhttp::ProxyConfig {
        use_manual_proxy: true,
        manual_proxy_address: server.into(),
        manual_proxy_bypass_list: bypass_list.into(),
        ..winhttp::empty_config()
    });
}

pub fn winhttp_set_no_proxy() {
    set_winhttp_config(&winhttp::ProxyConfig {
        ..winhttp::empty_config()
    });
}