use std::net::SocketAddr;
use std::process;

use log::error;
use tokio::net::TcpStream;

use crate::cli;
use crate::network::connectable::Connectable;

struct Connector<'a> {
    cli: &'a cli::Cli,
    socket_addr: SocketAddr,
    socket: Option<TcpStream>,
}

pub async fn connect_to_server(cli: &cli::Cli) {
    Connector::build(cli).await.connect().await;
}

impl Connectable for Connector<'_> {}

impl<'a> Connector<'a> {
    async fn build(cli: &'a cli::Cli) -> Connector<'a> {
        Self {
            cli,
            socket_addr: Connector::get_socket_addr(cli),
            socket: None,
        }
    }

    async fn connect(&mut self) -> &mut Connector<'a> {
        self.socket = Some(match TcpStream::connect(self.socket_addr).await {
            Ok(socket) => socket,
            Err(e) => {
                error!("Bind to {}: {}. QUITTING", self.socket_addr, e);
                process::exit(exitcode::NOHOST);
            }
        });

        self
    }
}
