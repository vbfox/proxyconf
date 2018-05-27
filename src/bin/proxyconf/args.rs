use clap::{App, Arg, SubCommand};

fn config_args<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("autodetect")
            .short("a")
            .long("auto-detect")
            .help("Automatically detect settings")
            .conflicts_with("setupscript")
            .conflicts_with("server"),
    ).arg(
            Arg::with_name("setupscript")
                .short("S")
                .long("setup-script")
                .value_name("SCRIPT_URL")
                .help("Setup script address")
                .takes_value(true)
                .conflicts_with("autodetect")
                .conflicts_with("server"),
        )
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("proxy-server")
                .alias("server")
                .value_name("ADDRESS:PORT")
                .help("Use a manual proxy with the specified address and port")
                .takes_value(true)
                .conflicts_with("autodetect")
                .conflicts_with("setupscript"),
        )
        .arg(
            Arg::with_name("bypass")
                .short("b")
                .long("bypass-list")
                .alias("bypass")
                .value_name("BYPASS-LIST")
                .help("List of addresses that don't use the proxy (Separated by semicolons)")
                .takes_value(true)
                .conflicts_with("autodetect")
                .conflicts_with("setupscript"),
        )
}

pub fn get<'a, 'b>() -> App<'a, 'b> {
    return App::new("ProxyConf")
        .version(crate_version!())
        .author("Julien Roncaglia <julien@roncaglia.fr>")
        .about("Windows proxy configuration from the command line")
        .subcommand(config_args(
            SubCommand::with_name("set").about("Set the proxy configuration"),
        ))
        .subcommand(SubCommand::with_name("show").about("Show the current proxy configuration"));
}
