extern crate proxyconf;

#[macro_use]
extern crate clap;

mod write_config {
    use proxyconf;
    use std::io;
    use std::io::Write;

    fn to_writer(writer: &mut Write, config: &proxyconf::ProxyConfig) {
        if config.automatically_detect_settings {
            writeln!(writer, "Automatically detect settings").unwrap();;
        }
        else if config.use_setup_script && config.setup_script_address.len() > 0 {
            writeln!(writer, "Setup script address: {}", config.setup_script_address).unwrap();;
        }
        else if config.use_manual_proxy && config.manual_proxy_address.len() > 0{
            writeln!(writer, "Manual proxy: {}", config.manual_proxy_address).unwrap();;
            writeln!(writer, "Exceptions: {}", config.manual_proxy_overrides).unwrap();;
        }
        else {
            writeln!(writer, "No proxy configured").unwrap();
        }
    }

    pub fn to_stdout(config: &proxyconf::ProxyConfig) {
        let mut stdout = io::stdout();
        to_writer(&mut stdout, &config);
    }
}

mod args {
    use clap::{Arg, App, SubCommand};

    fn config_args<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b>{
        app
            .arg(
                Arg::with_name("autodetect")
                .short("a")
                .long("auto-detect")
                .help("Automatically detect settings")
                .conflicts_with("setupscript")
                .conflicts_with("server"))
            .arg(
                Arg::with_name("setupscript")
                .short("S")
                .long("setup-script")
                .value_name("SCRIPT_URL")
                .help("Setup script address")
                .takes_value(true)
                .conflicts_with("autodetect")
                .conflicts_with("server"))
            .arg(
                Arg::with_name("server")
                .short("s")
                .long("server")
                .value_name("ADDRESS:PORT")
                .help("Use a manual proxy with the specified address and port")
                .takes_value(true)
                .conflicts_with("autodetect")
                .conflicts_with("setupscript"))
            .arg(
                Arg::with_name("overrides")
                .short("o")
                .long("overrides")
                .value_name("OVERRIDES")
                .help("List of addresses that don't use the proxy (Separated by semicolons)")
                .takes_value(true)
                .conflicts_with("autodetect")
                .conflicts_with("setupscript"))
    }

    pub fn get<'a, 'b>() -> App<'a, 'b> {
        return
            App::new("ProxyConf")
            .version(crate_version!())
            .author("Julien Roncaglia <julien@roncaglia.fr>")
            .about("Windows proxy configuration from the command line")
            .subcommand(
                config_args(
                    SubCommand::with_name("set")
                    .about("Set the proxy configuration")))
            .subcommand(
                SubCommand::with_name("show")
                .about("Show the current proxy configuration"));
    }
}

mod commands {
    use proxyconf;
    use write_config;

    fn set_config(config: &proxyconf::ProxyConfig) {
        proxyconf::registry::write(config.clone()).unwrap();

        print!("Configuration changed to: ");
        write_config::to_stdout(&config);
    }

    pub fn set_server(server: &str, overrides: &str) {
        set_config(
            &proxyconf::ProxyConfig {
                use_manual_proxy: true,
                manual_proxy_address: server.into(),
                manual_proxy_overrides: overrides.into(),
                ..proxyconf::empty()
            }
        );
    }

    pub fn set_setupscript(setupscript: &str) {
        set_config(
            &proxyconf::ProxyConfig {
                use_setup_script: true,
                setup_script_address: setupscript.into(),
                ..proxyconf::empty()
            }
        );
    }

    pub fn set_autodetect() {
        set_config(
            &proxyconf::ProxyConfig {
                automatically_detect_settings: true,
                ..proxyconf::empty()
            }
        );
    }
}

fn main() {
    let matches = args::get().get_matches();

    if let Some(_matches) = matches.subcommand_matches("show") {
        let conf = proxyconf::registry::read().unwrap();
        print!("Proxy configuration: ");
        write_config::to_stdout(&conf);
    } else if let Some(set_matches) = matches.subcommand_matches("set") {
        if let Some(server) = set_matches.value_of("server") {
            let overrides = set_matches.value_of("overrides").unwrap_or("<local>");
            commands::set_server(server, overrides);
        }
        else if let Some(setupscript) = set_matches.value_of("setupscript") {
            commands::set_setupscript(setupscript);
        }
        else if set_matches.is_present("autodetect") {
            commands::set_autodetect();
        } else {
            println!("ERROR: No configuration specified.");
        }
    } else {
        println!("Nope");

    }
}
