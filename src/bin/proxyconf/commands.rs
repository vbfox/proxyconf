pub mod winhttp {
    use proxyconf::internet_settings::modern::{ProxyConfig, FullConfig, empty_config, registry, WINHTTP_VERSION};
    use write_config;

    pub fn show() {
        let location = registry::get_winhttp_location();
        let conf = registry::read(&location).unwrap();
        write_config::ie_modern(&conf);
    }

    fn set_config(config: &ProxyConfig) {
        let location = registry::get_winhttp_location();
        let full_config = FullConfig {
            version: WINHTTP_VERSION,
            counter: 0,
            config: config.clone(),
        };
        registry::write_full(&location, &full_config).unwrap();

        println!("Configuration changed to: ");
        write_config::ie_modern(&config);
    }

    pub fn set_server(server: &str, bypass_list: &str) {
        set_config(&ProxyConfig {
            use_manual_proxy: true,
            manual_proxy_address: server.into(),
            manual_proxy_bypass_list: bypass_list.into(),
            ..empty_config()
        });
    }

    pub fn set_no_proxy() {
        set_config(&empty_config());
    }
}

pub mod envvars {
    use proxyconf::envvars_settings::{ProxyConfig, get_user, get_machine};

    fn show_config(config: &ProxyConfig) {
        if config.http_proxy_address.is_none() && config.https_proxy_address.is_none() && config.bypass_list.is_none() {
            println!("    Direct access (no proxy server).");
        } else {
            match &config.http_proxy_address {
                Some(value) => println!("    Http proxy  : {}", value),
                None => {},
            }
            match &config.https_proxy_address {
                Some(value) => println!("    Https proxy : {}", value),
                None => {},
            }
            match &config.bypass_list {
                Some(value) => println!("    Bypass list : {}", value),
                None => {},
            }
        }
    }

    pub fn show_user() {
        show_config(&get_user().unwrap())
    }

    pub fn show_machine() {
        show_config(&get_machine().unwrap())
    }
}

pub mod main {
    use super::winhttp;
    use super::envvars;
    use proxyconf::internet_settings::{legacy, modern};
    use write_config;

    fn set_legacy_config(config: &legacy::ProxyConfig) {
        legacy::registry::write(&config).unwrap();
    }

    fn set_modern_config(config: &modern::ProxyConfig) {
        let location = modern::registry::get_current_user_location();
        modern::registry::write(&location, config.clone()).unwrap();
    }

    fn set_config_and_show(config: &modern::ProxyConfig) {
        set_modern_config(&config);
        set_legacy_config(&config.to_owned().to_legacy());

        println!("Configuration changed to: ");
        write_config::ie_modern(&config);
    }

    pub fn set_server(server: &str, bypass_list: &str) {
        set_config_and_show(&modern::ProxyConfig {
            use_manual_proxy: true,
            manual_proxy_address: server.into(),
            manual_proxy_bypass_list: bypass_list.into(),
            ..modern::empty_config()
        });
    }

    pub fn set_setup_script(setup_script: &str) {
        set_config_and_show(&modern::ProxyConfig {
            use_setup_script: true,
            setup_script_address: setup_script.into(),
            ..modern::empty_config()
        });
    }

    pub fn set_auto_detect() {
        set_config_and_show(&modern::ProxyConfig {
            automatically_detect_settings: true,
            ..modern::empty_config()
        });
    }

    pub fn set_no_proxy() {
        set_config_and_show(&modern::ProxyConfig {
            ..modern::empty_config()
        });
    }

    fn modern_show() {
        let location = modern::registry::get_current_user_location();
        let conf = modern::registry::read(&location).unwrap();
        write_config::ie_modern(&conf);
    }

    pub fn show() {
        println!("Internet explorer: ");
        modern_show();

        println!();
        println!("WinHTTP (System wide): ");
        winhttp::show();

        println!();
        println!("Environment variables: ");
        envvars::show_user();

        println!();
        println!("Environment variables (System wide): ");
        envvars::show_machine();
    }
}
