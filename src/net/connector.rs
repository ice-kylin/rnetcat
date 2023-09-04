use std::net::SocketAddr;
use std::process;

use log::{error, info};
use tokio::io;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio_util::sync::CancellationToken;

use crate::net::connectable::Connectable;
use crate::{cli, util};

struct Connector<'a> {
    _cli: &'a cli::Cli,
    socket_addr: SocketAddr,
    rd: Option<OwnedReadHalf>,
    wr: Option<OwnedWriteHalf>,
    tk: CancellationToken,
}

pub async fn connect_to_server(cli: &cli::Cli) {
    Connector::build(cli).connect().await.process().await;
}

impl Connectable for Connector<'_> {}

impl<'a> Connector<'a> {
    /// Create a new connector.
    fn build(cli: &'a cli::Cli) -> Self {
        Self {
            _cli: cli,
            socket_addr: Connector::get_socket_addr(cli),
            rd: None,
            wr: None,
            tk: CancellationToken::new(),
        }
    }

    /// Get the socket address.
    async fn connect(&mut self) -> &mut Connector<'a> {
        let (rd, wr) = match TcpStream::connect(self.socket_addr).await {
            Ok(socket) => socket,
            Err(e) => {
                error!("Connect to {}: {}. QUITTING", self.socket_addr, e);
                process::exit(exitcode::NOHOST);
            }
        }
        .into_split();
        self.rd = Some(rd);
        self.wr = Some(wr);

        info!("Connecting to {}.", self.socket_addr);
        self
    }

    /// Process the connection.
    async fn process(&mut self) {
        self.spawn_read_and_write().read().await;

        info!("Connection from {} closed.", self.socket_addr);
    }

    /// Spawn a task to read from stdin and write to the socket.
    fn spawn_read_and_write(&mut self) -> &mut Connector<'a> {
        let mut wr = self.wr.take().unwrap(); // safe
        util::spawn_cancellable_task(&self.tk, async move {
            io::copy(&mut io::stdin(), &mut wr).await.unwrap(); // os error
        });

        self
    }

    /// Read from the socket and write to stdout.
    async fn read(&mut self) {
        io::copy(
            self.rd.as_mut().unwrap(), // safe
            &mut io::stdout(),
        )
        .await
        .unwrap(); // os error

        self.tk.cancel();
    }
}
