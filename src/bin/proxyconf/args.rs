use clap::{crate_version, Arg, Command};

trait CommonCommands {
    fn no_proxy(self) -> Command;
    fn auto_detect(self) -> Command;
    fn setup_script(self) -> Command;
    fn manual_proxy(self) -> Command;
}

impl CommonCommands for Command {
    fn no_proxy(self) -> Command {
        self.subcommand(
            Command::new("no-proxy")
                .about("Disable proxy")
                .aliases(["disabled"]),
        )
    }

    fn auto_detect(self) -> Command {
        self.subcommand(
            Command::new("auto-detect")
                .about("Automatically detect settings")
                .aliases(["auto"]),
        )
    }

    fn setup_script(self) -> Command {
        self.subcommand(
            Command::new("setup-script")
                .about("Use a Proxy auto-config setup script (PAC)")
                .arg(
                    Arg::new("url")
                        .index(1)
                        .value_name("SCRIPT_URL")
                        .help("URL of the auto-config setup script (PAC) file")
                        .required(true),
                ),
        )
    }

    fn manual_proxy(self) -> Command {
        self.subcommand(
            Command::new("proxy")
                .about("Use a manual proxy")
                .arg(
                    Arg::new("server")
                        .index(1)
                        .value_name("ADDRESS:PORT")
                        .help("Use a manual proxy with the specified address and port")
                        .required(true),
                )
                .arg(
                    Arg::new("bypass")
                        .index(2)
                        .value_name("BYPASS_LIST")
                        .help(
                            "List of addresses that don't use the proxy (Separated by semicolons)",
                        )
                        .required(false),
                ),
        )
    }
}

pub fn get() -> Command {
    Command::new("ProxyConf")
        .version(crate_version!())
        .author("Julien Roncaglia <julien@roncaglia.fr>")
        .about("Windows proxy configuration from the command line")
        .subcommand(
            Command::new("set")
                .about("Set the current user proxy configuration")
                .no_proxy()
                .auto_detect()
                .setup_script()
                .manual_proxy(),
        )
        .subcommand(Command::new("show").about("Show the current proxy configuration"))
        .subcommand(
            Command::new("winhttp")
                .about("Set the system-wide WinHTTP configuration")
                .no_proxy()
                .manual_proxy(),
        )
}
