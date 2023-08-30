use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv6Addr};
use std::str::FromStr;

/// This struct represents a hostname and port.
pub struct HostAddr {
    pub hostname: Hostname,
    pub port: u16,
}

impl Display for HostAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hostname, self.port)
    }
}

/// This enum represents a hostname or IP address.
pub enum Hostname {
    Name(String),
    Addr(IpAddr),
}

impl Display for Hostname {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Hostname::Name(name) => write!(f, "{}", name),
            Hostname::Addr(addr) => write!(f, "{}", addr),
        }
    }
}

/// This function parses the hostname and port from the command line arguments.
///
/// If the hostname is not specified, and the program is listening, then
/// the hostname is set to `::` (IPv6 unspecified address).
///
/// If the hostname is not specified, and the program is connecting, then
/// an error is returned.
///
/// If the hostname is specified, then it is parsed as an IP address, and
/// if that fails, then it is parsed as a hostname.
///
/// If the port is not specified, then the port is set to 31337.
///
/// # Arguments
///
/// * `listen` - Whether the program is listening or connecting.
/// * `hostname` - The hostname to parse.
/// * `port` - The port to parse.
///
/// # Returns
///
/// * `Ok(HostAddr)` - The parsed hostname and port.
/// * `Err(String)` - An error message.
pub fn parse_host_addr(
    listen: bool,
    hostname: &Option<String>,
    port: Option<u16>,
) -> Result<HostAddr, String> {
    let hostname = parse_hostname(listen, hostname)?;
    let port = parse_port(port);
    Ok(HostAddr { hostname, port })
}


/// This function parses the hostname from the command line arguments.
///
/// If the hostname is not specified, and the program is listening, then
/// the hostname is set to `::` (IPv6 unspecified address).
///
/// If the hostname is not specified, and the program is connecting, then
/// an error is returned.
///
/// If the hostname is specified, then it is parsed as an IP address, and
/// if that fails, then it is parsed as a hostname.
///
/// # Arguments
///
/// * `listen` - Whether the program is listening or connecting.
/// * `hostname` - The hostname to parse.
///
/// # Returns
///
/// * `Ok(Hostname)` - The parsed hostname.
/// * `Err(String)` - An error message.
fn parse_hostname(listen: bool, hostname: &Option<String>) -> Result<Hostname, String> {
    match hostname {
        Some(hostname) => match IpAddr::from_str(&hostname) {
            Ok(hostname) => Ok(Hostname::Addr(hostname)),
            _ => Ok(Hostname::Name(hostname.to_owned()))
        }
        None => {
            if listen {
                Ok(Hostname::Addr(IpAddr::V6(Ipv6Addr::UNSPECIFIED)))
            } else {
                Err(String::from("No hostname specified"))
            }
        }
    }
}

/// This function parses the port from the command line arguments.
///
/// If the port is not specified, then the port is set to 31337.
///
/// # Arguments
///
/// * `port` - The port to parse.
///
/// # Returns
///
/// * `u16` - The parsed port.
fn parse_port(port: Option<u16>) -> u16 {
    match port {
        Some(port) => port,
        None => 31337,
    }
}
