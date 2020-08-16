use clap::{App, Arg, SubCommand};

trait CommonCommands<'a, 'b> {
    fn no_proxy(self) -> App<'a, 'b>;
    fn auto_detect(self) -> App<'a, 'b>;
    fn setup_script(self) -> App<'a, 'b>;
    fn manual_proxy(self) -> App<'a, 'b>;
}

impl<'a, 'b> CommonCommands<'a, 'b> for App<'a, 'b> {
    fn no_proxy(self) -> App<'a, 'b> {
        self.subcommand(
            SubCommand::with_name("no-proxy")
                .about("Disable proxy")
                .aliases(&["disabled"]),
        )
    }

    fn auto_detect(self) -> App<'a, 'b> {
        self.subcommand(
            SubCommand::with_name("auto-detect")
                .about("Automatically detect settings")
                .aliases(&["auto"]),
        )
    }

    fn setup_script(self) -> App<'a, 'b> {
        self.subcommand(
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
    }

    fn manual_proxy(self) -> App<'a, 'b> {
        self.subcommand(
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
}

pub fn get<'a, 'b>() -> App<'a, 'b> {
    App::new("ProxyConf")
        .version(crate_version!())
        .author("Julien Roncaglia <julien@roncaglia.fr>")
        .about("Windows proxy configuration from the command line")
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the current user proxy configuration")
                .no_proxy()
                .auto_detect()
                .setup_script()
                .manual_proxy(),
        )
        .subcommand(SubCommand::with_name("show").about("Show the current proxy configuration"))
        .subcommand(
            SubCommand::with_name("winhttp")
                .about("Set the system-wide WinHTTP configuration")
                .no_proxy()
                .manual_proxy(),
        )
}
