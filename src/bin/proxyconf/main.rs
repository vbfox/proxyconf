extern crate proxyconf;

#[macro_use]
extern crate clap;

mod args;
mod commands;
mod write_config;

fn on_unexpected() {
    args::get().print_help();
    std::process::exit(1);
}

fn main() {
    let matches = args::get().get_matches();

    if let Some(_matches) = matches.subcommand_matches("show") {
        let conf = proxyconf::registry::read().unwrap();
        print!("Proxy configuration: ");
        write_config::to_stdout(&conf);
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
