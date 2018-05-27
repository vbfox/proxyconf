extern crate proxyconf;

#[macro_use]
extern crate clap;

mod args;
mod commands;
mod write_config;

fn main() {
    let matches = args::get().get_matches();

    if let Some(_matches) = matches.subcommand_matches("show") {
        let conf = proxyconf::registry::read().unwrap();
        print!("Proxy configuration: ");
        write_config::to_stdout(&conf);
    } else if let Some(set_matches) = matches.subcommand_matches("set") {
        if let Some(server) = set_matches.value_of("server") {
            let overrides = set_matches.value_of("bypass").unwrap_or("<local>");
            commands::set_server(server, overrides);
        } else if let Some(setupscript) = set_matches.value_of("setupscript") {
            commands::set_setupscript(setupscript);
        } else if set_matches.is_present("autodetect") {
            commands::set_autodetect();
        } else {
            println!("ERROR: No configuration specified.");
        }
    } else {
        println!("Nope");
    }
}
