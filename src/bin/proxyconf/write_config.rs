use proxyconf;
use std::io;
use std::io::Write;

fn to_writer(writer: &mut Write, config: &proxyconf::ProxyConfig) {
    if config.automatically_detect_settings {
        writeln!(writer, "Automatically detect settings").unwrap();;
    } else if config.use_setup_script && config.setup_script_address.len() > 0 {
        writeln!(
            writer,
            "Setup script address: {}",
            config.setup_script_address
        ).unwrap();;
    } else if config.use_manual_proxy && config.manual_proxy_address.len() > 0 {
        writeln!(writer, "Manual proxy: {}", config.manual_proxy_address).unwrap();;
        writeln!(writer, "Bypass list: {}", config.manual_proxy_bypass_list).unwrap();;
    } else {
        writeln!(writer, "No proxy configured").unwrap();
    }
}

pub fn to_stdout(config: &proxyconf::ProxyConfig) {
    let mut stdout = io::stdout();
    to_writer(&mut stdout, &config);
}
