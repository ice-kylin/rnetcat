use std::net::SocketAddr;
use std::process;

use log::{error, info};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;

use crate::net::connectable::Connectable;
use crate::{cli, util};

struct Connector<'a> {
    _cli: &'a cli::Cli,
    socket_addr: SocketAddr,
    rd: Option<OwnedReadHalf>,
    wr: Option<OwnedWriteHalf>,
    tx: Sender<Vec<u8>>,
    tk: CancellationToken,
}

pub async fn connect_to_server(cli: &cli::Cli) {
    Connector::build(cli).connect().await.listen().await;
}

impl Connectable for Connector<'_> {}

impl<'a> Connector<'a> {
    /// Create a new connector.
    fn build(cli: &'a cli::Cli) -> Connector<'a> {
        let (tx, _) = broadcast::channel(16);

        Self {
            _cli: cli,
            socket_addr: Connector::get_socket_addr(cli),
            rd: None,
            wr: None,
            tx,
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

    /// Listen for incoming messages.
    async fn listen(&mut self) {
        self.spawn_input().spawn_write().read().await;

        info!("Connection from {} closed.", self.socket_addr);
    }

    /// Spawn a task to read from stdin and write to the broadcast channel.
    fn spawn_input(&mut self) -> &mut Connector<'a> {
        let tx_clone = self.tx.clone();
        util::spawn_cancellable_task(&self.tk, async move {
            let mut buffer = vec![0; 8192];
            loop {
                let n = io::stdin().read(&mut buffer).await.unwrap(); // os error

                // EOF or broken pipe.
                if n == 0 || tx_clone.send(buffer[..n].to_owned()).is_err() {
                    break;
                }
            }
        });

        self
    }

    /// Spawn a task to write bytes from the broadcast channel to the socket.
    fn spawn_write(&mut self) -> &mut Connector<'a> {
        let mut rx = self.tx.subscribe();
        let mut wr = self.wr.take().unwrap(); // safe
        util::spawn_cancellable_task(&self.tk, async move {
            while let Ok(msg) = rx.recv().await {
                if wr.write_all(&msg).await.is_err() {
                    break;
                }
            }
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
