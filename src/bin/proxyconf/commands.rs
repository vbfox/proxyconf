pub mod winhttp {
    use crate::command_result::CommandResult;
    use crate::write_config;
    use proxyconf::internet_settings::modern::registry::RegistryError;
    use proxyconf::internet_settings::modern::{
        empty_config, registry, FullConfig, ProxyConfig, WINHTTP_VERSION,
    };

    pub fn show() {
        let location = registry::get_winhttp_location();
        let conf = registry::read(&location);
        match conf {
            Ok(conf) => write_config::ie_modern(&conf),
            Err(e) => println!("    Error: {}", e),
        }
    }

    fn is_access_denied(err: &RegistryError) -> bool {
        match err {
            RegistryError::Io(io_err) => io_err.kind() == std::io::ErrorKind::PermissionDenied,
            _ => false,
        }
    }

    fn set_config(config: &ProxyConfig) -> CommandResult {
        let location = registry::get_winhttp_location();
        let full_config = FullConfig {
            version: WINHTTP_VERSION,
            counter: 0,
            config: config.clone(),
        };

        match registry::write_full(&location, &full_config) {
            Ok(()) => {
                println!("Configuration changed to: ");
                write_config::ie_modern(&config);
                CommandResult::Ok
            }
            Err(e) => {
                if is_access_denied(&e) {
                    println!("Access denied, you need to run this operation as administrator");
                    CommandResult::AccessDenied
                } else {
                    println!("Error: {}", e);
                    CommandResult::Error
                }
            }
        }
    }

    pub fn set_server(server: &str, bypass_list: &str) -> CommandResult {
        set_config(&ProxyConfig {
            use_manual_proxy: true,
            manual_proxy_address: server.into(),
            manual_proxy_bypass_list: bypass_list.into(),
            ..empty_config()
        })
    }

    pub fn set_no_proxy() -> CommandResult {
        set_config(&empty_config())
    }
}

pub mod envvars {
    use proxyconf::envvars_settings::{get_machine, get_user, ProxyConfig};
    use std::io;

    fn show_config(config: &ProxyConfig) {
        if config.http_proxy_address.is_none()
            && config.https_proxy_address.is_none()
            && config.bypass_list.is_none()
        {
            println!("    Direct access (no proxy server).");
        } else {
            match &config.http_proxy_address {
                Some(value) => println!("    Http proxy  : {}", value),
                None => {}
            }
            match &config.https_proxy_address {
                Some(value) => println!("    Https proxy : {}", value),
                None => {}
            }
            match &config.bypass_list {
                Some(value) => println!("    Bypass list : {}", value),
                None => {}
            }
        }
    }

    fn try_show_config(config: &io::Result<ProxyConfig>) {
        match config {
            Ok(config) => show_config(config),
            Err(e) => println!("    Error: {}", e),
        }
    }

    pub fn show_user() {
        try_show_config(&get_user())
    }

    pub fn show_machine() {
        try_show_config(&get_machine())
    }
}

pub mod main {
    use super::envvars;
    use super::winhttp;
    use crate::command_result::CommandResult;
    use crate::write_config;
    use proxyconf::internet_settings::{legacy, modern};

    fn set_legacy_config(config: &legacy::ProxyConfig) {
        legacy::registry::write(&config).unwrap();
    }

    fn set_modern_config(config: &modern::ProxyConfig) {
        let location = modern::registry::get_current_user_location();
        modern::registry::write(&location, config.clone()).unwrap();
    }

    fn set_config_and_show(config: &modern::ProxyConfig) {
        set_modern_config(&config);
        set_legacy_config(&config.to_legacy());

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
        let conf = modern::registry::read(&location);
        match &conf {
            Ok(conf) => write_config::ie_modern(&conf),
            Err(e) => println!("    Error: {}", e),
        }
    }

    pub fn show() -> CommandResult {
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

        CommandResult::Ok
    }
}
