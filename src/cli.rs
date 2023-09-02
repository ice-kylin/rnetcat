use std::ops::RangeInclusive;

use clap::{Args, Parser};
use clap_verbosity_flag::WarnLevel;

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[command(flatten)]
    pub ipvs: Ipvs,

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

    /// Set verbosity level
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity<WarnLevel>,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct Ipvs {
    /// Use IPv4 only
    #[arg(short = '4', long = None)]
    pub ipv4: bool,

    /// Use IPv6 only
    #[arg(short = '6', long = None)]
    pub ipv6: bool,
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

impl Cli {
    /// Reorder the hostname and port.
    #[allow(dead_code)] // TODO: Check this attribute.
    pub fn reorder(&mut self) {
        if self.listen && self.hostname.is_some() && self.port.is_none() {
            if let Ok(port) = self.hostname.as_ref().unwrap().parse::<u16>() {
                self.hostname = None;
                self.port = Some(port);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_in_range() -> Result<(), String> {
        let port = port_in_range("1")?;

        assert_eq!(port, 1);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "port not in range 1-65535")]
    fn test_port_not_in_range() {
        port_in_range("65536").unwrap();
    }

    #[test]
    #[should_panic(expected = "`a` isn't a port number")]
    fn test_port_not_a_number() {
        port_in_range("a").unwrap();
    }
}
