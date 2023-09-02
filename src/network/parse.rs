use std::error::Error;
use std::fmt;
use std::net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

use crate::cli;

/// This enum represents a host parsing error.
///
/// # Variants
///
/// * `AddrParse(AddrParseError)` - An `AddrParseError` occurred.
/// * `NoHostName` - No hostname was specified.
#[derive(Debug)]
pub enum HostParseError {
    AddrParse(AddrParseError),
    NoHostName,
}

/// This function parses the socket address from the command line arguments.
///
/// If the hostname is not specified, and the program is listening, then
/// the hostname is set to `::` (IPv6 unspecified address).
///
/// If the hostname is not specified, and the program is connecting, then
/// an error is returned.
///
/// # Arguments
///
/// * `cli` - The command line arguments.
///
/// # Returns
///
/// * `Result<SocketAddr, HostParseError>` - The parsed socket address.
pub fn parse_socket_addr(cli: &cli::Cli) -> Result<SocketAddr, HostParseError> {
    match &cli.hostname {
        None => parse_unspecified_addr(cli),
        Some(_) => parse_specified_addr(cli),
    }
}

fn parse_specified_addr(cli: &cli::Cli) -> Result<SocketAddr, HostParseError> {
    let hostname = cli.hostname.as_ref().unwrap();
    let port = parse_port(cli.port);

    Ok(SocketAddr::from_str(&format!("{}:{}", hostname, port))?)
}

fn parse_unspecified_addr(cli: &cli::Cli) -> Result<SocketAddr, HostParseError> {
    if cli.listen {
        Ok(parse_listen_unspecified_addr(cli))
    } else {
        Err(HostParseError::NoHostName)
    }
}

fn parse_listen_unspecified_addr(cli: &cli::Cli) -> SocketAddr {
    let port = parse_port(cli.port);

    if cli.ipvs.ipv4 {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)
    } else {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), port)
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

impl Error for HostParseError {}

impl fmt::Display for HostParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HostParseError::AddrParse(e) => write!(f, "AddrParseError: {}", e),
            HostParseError::NoHostName => write!(f, "No hostname specified"),
        }
    }
}

impl From<AddrParseError> for HostParseError {
    fn from(error: AddrParseError) -> Self {
        HostParseError::AddrParse(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_none_port() {
        assert_eq!(parse_port(None), 31337);
    }

    #[test]
    fn test_parse_some_port() {
        assert_eq!(parse_port(Some(1234)), 1234);
    }
}
