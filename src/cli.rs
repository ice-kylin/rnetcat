use std::ops::RangeInclusive;

use clap::Parser;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Use IPv4 only
    #[arg(short = '4', long = None)]
    pub ipv4: bool,

    /// Use IPv6 only
    #[arg(short = '6', long = None)]
    pub ipv6: bool,

    /// Bind and listen for incoming connections
    #[arg(short = 'l', long = "listen")]
    pub listen: bool,

    /// Accept multiple connections in listen mode
    #[arg(short = 'k', long = "keep-open")]
    pub keep_open: bool,

    /// Hostname or IP address to connect to
    pub hostname: Option<String>,

    /// Port number to connect to or listen on
    #[arg(value_parser = port_in_range)]
    pub port: Option<u16>,
}

/// This function parses the port from the command line arguments.
fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}
