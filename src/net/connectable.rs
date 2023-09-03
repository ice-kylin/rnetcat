use std::process;

use log::error;

use crate::cli;
use crate::net::parse;

pub trait Connectable {
    fn get_socket_addr(cli: &cli::Cli) -> std::net::SocketAddr {
        match parse::parse_socket_addr(cli) {
            Ok(socket_addr) => socket_addr,
            Err(e) => {
                error!("{}. QUITTING", e);
                process::exit(exitcode::USAGE);
            }
        }
    }
}
