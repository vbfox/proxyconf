use crate::command_result::CommandResult;
use clap::ArgMatches;

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

fn arg<'a>(matches: &'a ArgMatches, name: &str) -> Option<&'a str> {
    matches.get_one::<String>(name).map(String::as_str)
}

fn main() {
    let matches = args::get().get_matches();

    match matches.subcommand() {
        Some(("show", _)) => exit_with_command_result(commands::main::show()),
        Some(("set", set_matches)) => match set_matches.subcommand() {
            Some(("no-proxy", _)) => commands::main::set_no_proxy(),
            Some(("auto-detect", _)) => commands::main::set_auto_detect(),
            Some(("setup-script", script_matches)) => {
                let url = arg(script_matches, "url").unwrap();
                commands::main::set_setup_script(url);
            }
            Some(("proxy", proxy_matches)) => {
                let server = arg(proxy_matches, "server").unwrap();
                let bypass_list = arg(proxy_matches, "bypass").unwrap_or("<local>");
                commands::main::set_server(server, bypass_list);
            }
            _ => on_unexpected_command(),
        },
        Some(("winhttp", winhttp_matches)) => match winhttp_matches.subcommand() {
            Some(("no-proxy", _)) => exit_with_command_result(commands::winhttp::set_no_proxy()),
            Some(("proxy", proxy_matches)) => {
                let server = arg(proxy_matches, "server").unwrap();
                let bypass_list = arg(proxy_matches, "bypass").unwrap_or("<local>");
                exit_with_command_result(commands::winhttp::set_server(server, bypass_list));
            }
            _ => on_unexpected_command(),
        },
        _ => args::get().print_help().unwrap(),
    }
}
