use std::error::Error;
use std::fmt;
use std::net::{AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};

use crate::cli;

/// This enum represents a host parsing error.
///
/// # Variants
///
/// * `AddrParse(AddrParseError)`: An `AddrParseError` occurred.
/// * `NoHostName`: No hostname was specified.
/// * `IpVersionMismatch`: The IP version of the hostname and the IP version of the IP version
///  specified in the command line arguments do not match.
#[derive(Debug)]
pub enum HostParseError {
    AddrParse(AddrParseError),
    NoHostName,
    IpVersionMismatch,
}

/// This function parses the socket address from the command line arguments.
///
/// # Arguments
///
/// * `cli`: The command line arguments.
pub fn parse_socket_addr(cli: &cli::Cli) -> Result<SocketAddr, HostParseError> {
    match &cli.hostname {
        None => parse_unspecified_socket_addr(cli),
        Some(_) => parse_specified_socket_addr(cli),
    }
}

fn parse_specified_socket_addr(cli: &cli::Cli) -> Result<SocketAddr, HostParseError> {
    (
        cli.hostname.as_ref().unwrap().as_ref(),
        parse_port(cli.port),
    )
        .to_socket_addrs()
        .unwrap()
        .filter(|x| !(cli.ipvs.ipv4 && x.is_ipv6() || cli.ipvs.ipv6 && x.is_ipv4()))
        .nth(0)
        .ok_or(HostParseError::IpVersionMismatch)
}

fn parse_unspecified_socket_addr(cli: &cli::Cli) -> Result<SocketAddr, HostParseError> {
    if cli.listen {
        Ok(SocketAddr::new(
            if cli.ipvs.ipv4 {
                IpAddr::V4(Ipv4Addr::UNSPECIFIED)
            } else {
                IpAddr::V6(Ipv6Addr::UNSPECIFIED)
            },
            parse_port(cli.port),
        ))
    } else {
        Err(HostParseError::NoHostName)
    }
}

/// This function parses the port from the command line arguments.
///
/// If the port is not specified, then the port is set to 31337.
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
            HostParseError::IpVersionMismatch => write!(f, "IP version mismatch"),
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
    use clap::Parser;

    use super::*;

    fn parse_cli(args: &[&str]) -> cli::Cli {
        let mut result = cli::Cli::parse_from(args);
        result.reorder();

        result
    }

    #[test]
    fn test_parse_socket_addr_1() -> Result<(), HostParseError> {
        let socket_addr = parse_socket_addr(&parse_cli(&["rnc", "-l", "127.0.0.1", "1234"]))?;

        assert_eq!(socket_addr.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(socket_addr.port(), 1234);

        Ok(())
    }

    #[test]
    fn test_parse_socket_addr_2() -> Result<(), HostParseError> {
        let socket_addr = parse_socket_addr(&parse_cli(&["rnc", "-l", "127.0.0.1"]))?;

        assert_eq!(socket_addr.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(socket_addr.port(), 31337);

        Ok(())
    }

    #[test]
    fn test_parse_socket_addr_3() -> Result<(), HostParseError> {
        let socket_addr = parse_socket_addr(&parse_cli(&["rnc", "-l", "1234"]))?;

        assert_eq!(socket_addr.ip(), IpAddr::V6(Ipv6Addr::UNSPECIFIED));
        assert_eq!(socket_addr.port(), 1234);

        Ok(())
    }

    #[test]
    fn test_parse_socket_addr_4() -> Result<(), HostParseError> {
        let socket_addr = parse_socket_addr(&parse_cli(&["rnc", "-l"]))?;

        assert_eq!(socket_addr.ip(), IpAddr::V6(Ipv6Addr::UNSPECIFIED));
        assert_eq!(socket_addr.port(), 31337);

        Ok(())
    }

    #[test]
    fn test_parse_socket_addr_5() -> Result<(), HostParseError> {
        let socket_addr = parse_socket_addr(&parse_cli(&["rnc", "127.0.0.1", "1234"]))?;

        assert_eq!(socket_addr.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(socket_addr.port(), 1234);

        Ok(())
    }

    #[test]
    fn test_parse_socket_addr_6() -> Result<(), HostParseError> {
        let socket_addr = parse_socket_addr(&parse_cli(&["rnc", "127.0.0.1"]))?;

        assert_eq!(socket_addr.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(socket_addr.port(), 31337);

        Ok(())
    }

    #[test]
    fn test_parse_socket_addr_7() -> Result<(), HostParseError> {
        let socket_addr = parse_socket_addr(&parse_cli(&["rnc", "localhost"]))?;

        assert_eq!(socket_addr.ip(), IpAddr::V6(Ipv6Addr::LOCALHOST));
        assert_eq!(socket_addr.port(), 31337);

        Ok(())
    }

    #[test]
    fn test_parse_socket_addr_8() -> Result<(), HostParseError> {
        let socket_addr = parse_socket_addr(&parse_cli(&["rnc","-4","localhost"]))?;

        assert_eq!(socket_addr.ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(socket_addr.port(), 31337);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "IpVersionMismatch")]
    fn test_parse_socket_addr_9()  {
        parse_socket_addr(&parse_cli(&["rnc","-6","127.0.0.1"])).unwrap();
    }

    #[test]
    #[should_panic(expected = "NoHostName")]
    fn test_parse_socket_addr_10()  {
        parse_socket_addr(&parse_cli(&["rnc"])).unwrap();
    }
}
