extern crate proxyconf;

extern crate clap;
use clap::{Arg, App, SubCommand};
use std::io;
use std::io::Write;

fn write_config(writer: &mut Write, config: &proxyconf::ProxyConfig) {
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
        writeln!(writer, "No proxy configured").unwrap();;
    }
}

fn main() {
    let matches =
        App::new("ProxyConf")
            .version("1.0")
            .author("Julien Roncaglia <julien@roncaglia.fr>")
            .about("Windows proxy configuration from the command line")
            /*.arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true))
            .arg(Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1))
            .arg(Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"))*/
            .arg(Arg::with_name("server")
                .short("s")
                .long("server")
                .value_name("address:port")
                .help("Set the address and port of the manual proxy")
                .takes_value(true))
            .subcommand(SubCommand::with_name("show")
                        .about("Show the current proxy configuration"))
            /*
            .subcommand(SubCommand::with_name("update")
                .arg(Arg::with_name("server")
                    .short("s")
                    .long("server")
                    .value_name("address:port")
                    .help("Set the address and port of the manual proxy")
                    .takes_value(true))
                .about("Update part of the current config"))*/
            .get_matches();

    if let Some(_matches) = matches.subcommand_matches("show") {
        let conf = proxyconf::registry::read().unwrap();
        print!("Proxy configuration: ");
        let mut stdout = io::stdout();
        write_config(&mut stdout, &conf.config);
    } else {
        if let Some(server) = matches.value_of("server") {
            proxyconf::registry::update(|conf| {
                return proxyconf::ProxyConfig {
                    use_manual_proxy: true,
                    manual_proxy_address: server.into(),
                    ..conf
                }
            }).unwrap();
        } else {
            let conf = proxyconf::registry::read().unwrap();
            println!("conf = {:?}", conf);
            let mut bytes = Vec::new();
            proxyconf::serialization::serialize(&conf, &mut bytes).unwrap();
            println!("bytes = {:?}", bytes);
            let conf2 = proxyconf::serialization::deserialize(&bytes[..]).unwrap();
            println!("conf2X = {:?}", conf2);
        }
    }
}
