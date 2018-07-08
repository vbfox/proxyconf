extern crate proxyconf;

#[macro_use]
extern crate clap;

mod args;
mod commands;
mod write_config;

fn on_unexpected() {
    args::get().print_help().unwrap();
    std::process::exit(1);
}

fn main() {
    let matches = args::get().get_matches();

    if let Some(_) = matches.subcommand_matches("show") {
        commands::main::show();
    } else if let Some(set_matches) = matches.subcommand_matches("set") {
        if let Some(_) = set_matches.subcommand_matches("no-proxy") {
            commands::main::set_no_proxy();
        } else if let Some(_) = set_matches.subcommand_matches("auto-detect") {
            commands::main::set_auto_detect();
        } else if let Some(script_matches) = set_matches.subcommand_matches("setup-script") {
            let url = script_matches.value_of("url").unwrap();
            commands::main::set_setup_script(url);
        } else if let Some(proxy_matches) = set_matches.subcommand_matches("proxy") {
            let server = proxy_matches.value_of("server").unwrap();
            let bypass_list = proxy_matches.value_of("bypass").unwrap_or("<local>");
            commands::main::set_server(server, bypass_list);
        } else {
            on_unexpected();
        }
    } else if let Some(winhttp_matches) = matches.subcommand_matches("winhttp") {
        if let Some(_) = winhttp_matches.subcommand_matches("show") {
            commands::winhttp::show();
        } else if let Some(set_matches) = winhttp_matches.subcommand_matches("set") {
            if let Some(_) = set_matches.subcommand_matches("no-proxy") {
                commands::winhttp::set_no_proxy();
            } else if let Some(proxy_matches) = set_matches.subcommand_matches("proxy") {
                let server = proxy_matches.value_of("server").unwrap();
                let bypass_list = proxy_matches.value_of("bypass").unwrap_or("<local>");
                commands::winhttp::set_server(server, bypass_list);
            } else {
                on_unexpected();
            }
        } else {
            on_unexpected();
        }
    } else {
        on_unexpected();
    }
}
