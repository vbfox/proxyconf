use clap::{App, Arg, SubCommand};

fn args_for_set<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name("no-proxy")
            .about("Disable proxy")
            .aliases(&["disabled"]),
    ).subcommand(
            SubCommand::with_name("auto-detect")
                .about("Automatically detect settings")
                .aliases(&["auto"]),
        )
        .subcommand(
            SubCommand::with_name("setup-script")
                .about("Use a Proxy auto-config setup script (PAC)")
                .arg(
                    Arg::with_name("url")
                        .index(1)
                        .value_name("SCRIPT_URL")
                        .help("URL of the auto-config setup script (PAC) file")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("proxy")
                .about("Use a manual proxy")
                .arg(
                    Arg::with_name("server")
                        .index(1)
                        .value_name("ADDRESS:PORT")
                        .help("Use a manual proxy with the specified address and port")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("bypass")
                        .index(2)
                        .value_name("BYPASS_LIST")
                        .help(
                            "List of addresses that don't use the proxy (Separated by semicolons)",
                        )
                        .takes_value(true)
                        .required(false),
                ),
        )
}

pub fn get<'a, 'b>() -> App<'a, 'b> {
    return App::new("ProxyConf")
        .version(crate_version!())
        .author("Julien Roncaglia <julien@roncaglia.fr>")
        .about("Windows proxy configuration from the command line")
        .subcommand(args_for_set(
            SubCommand::with_name("set").about("Set the proxy configuration"),
        ))
        .subcommand(SubCommand::with_name("show").about("Show the current proxy configuration"));
}
