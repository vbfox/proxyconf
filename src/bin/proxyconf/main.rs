extern crate proxyconf;

#[macro_use]
extern crate clap;

mod args;
mod commands;
mod write_config;

use proxyconf::ie;
use proxyconf::winhttp;

fn on_unexpected() {
    args::get().print_help().unwrap();
    std::process::exit(1);
}

fn main() {
    let matches = args::get().get_matches();

    if let Some(_matches) = matches.subcommand_matches("show") {
        let conf = ie::modern::registry::read().unwrap();
        println!("Internet explorer: ");
        write_config::ie_modern(&conf);

        println!("");
        println!("Internet explorer (legacy): ");
        let legacy_conf = ie::legacy::registry::read().unwrap();
        write_config::ie_legacy(&legacy_conf);

        println!("");
        println!("WinHTTP (System wide): ");
        let winhttp_conf = winhttp::registry::read().unwrap();
        write_config::winhttp(&winhttp_conf);
    } else if let Some(set_matches) = matches.subcommand_matches("set") {
        if let Some(_) = set_matches.subcommand_matches("no-proxy") {
            commands::set_no_proxy();
        } else if let Some(_) = set_matches.subcommand_matches("auto-detect") {
            commands::set_auto_detect();
        } else if let Some(script_matches) = set_matches.subcommand_matches("setup-script") {
            let url = script_matches.value_of("url").unwrap();
            commands::set_setup_script(url);
        } else if let Some(proxy_matches) = set_matches.subcommand_matches("proxy") {
            let server = proxy_matches.value_of("server").unwrap();
            let bypass_list = proxy_matches.value_of("bypass").unwrap_or("<local>");
            commands::set_server(server, bypass_list);
        } else {
            on_unexpected();
        }
    } else {
        on_unexpected();
    }
}
