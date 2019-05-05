extern crate proxyconf;

#[macro_use]
extern crate clap;

use crate::command_result::CommandResult;

mod args;
mod command_result;
mod commands;
mod write_config;

fn exit_with_command_result(result: CommandResult) {
    let code = result as i32;
    std::process::exit(code);
}

fn on_unexpected_command() {
    args::get().print_help().unwrap();
    exit_with_command_result(CommandResult::UnexpectedCommand)
}

fn main() {
    let matches = args::get().get_matches();

    if let Some(_) = matches.subcommand_matches("show") {
        exit_with_command_result(commands::main::show());
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
            on_unexpected_command();
        }
    } else if let Some(winhttp_matches) = matches.subcommand_matches("winhttp") {
        if let Some(_) = winhttp_matches.subcommand_matches("no-proxy") {
            exit_with_command_result(commands::winhttp::set_no_proxy());
        } else if let Some(proxy_matches) = winhttp_matches.subcommand_matches("proxy") {
            let server = proxy_matches.value_of("server").unwrap();
            let bypass_list = proxy_matches.value_of("bypass").unwrap_or("<local>");
            exit_with_command_result(commands::winhttp::set_server(server, bypass_list));
        } else {
            on_unexpected_command();
        }
    } else {
        args::get().print_help().unwrap();
    }
}
