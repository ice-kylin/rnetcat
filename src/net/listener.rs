use std::net::SocketAddr;
use std::process;

use log::{error, info};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

use crate::cli;
use crate::net::connectable::Connectable;
use crate::util;

struct Listener<'a> {
    cli: &'a cli::Cli,
    socket_addr: SocketAddr,
    listener: Option<TcpListener>,
    tx: broadcast::Sender<Vec<u8>>,
    tk: CancellationToken,
}

struct ListenerConnection {
    client_addr: SocketAddr,
    rd: Option<OwnedReadHalf>,
    wr: Option<OwnedWriteHalf>,
    tx: broadcast::Sender<Vec<u8>>,
    tk: CancellationToken,
}

pub async fn start_listener(cli: &cli::Cli) {
    Listener::build(cli)
        .bind()
        .await
        .spawn_input()
        .process()
        .await;
}

impl Connectable for Listener<'_> {}

impl<'a> Listener<'a> {
    fn build(cli: &'a cli::Cli) -> Self {
        let (tx, _) = broadcast::channel(16);
        let socket_addr = Listener::get_socket_addr(&cli);

        Self {
            cli,
            socket_addr,
            listener: None,
            tx,
            tk: CancellationToken::new(),
        }
    }

    async fn bind(&'a mut self) -> &mut Listener<'a> {
        self.listener = Some(match TcpListener::bind(self.socket_addr).await {
            Ok(listener) => listener,
            Err(e) => {
                error!("Bind to {}: {}. QUITTING", self.socket_addr, e);
                process::exit(exitcode::NOHOST);
            }
        });

        info!("Listening on {}.", self.socket_addr);

        self
    }

    async fn process(&mut self) {
        if self.cli.keep_open {
            loop {
                let mut connection = ListenerConnection::accept(self).await;
                connection.spawn_write();

                tokio::spawn(async move {
                    connection.read().await;
                });
            }
        } else {
            ListenerConnection::accept(self)
                .await
                .spawn_write()
                .read()
                .await;
        }

        info!("Listener on {} closed.", self.socket_addr);
    }

    fn spawn_input(&mut self) -> &mut Self {
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
}

impl ListenerConnection {
    async fn accept(listener: &Listener<'_>) -> Self {
        let (socket, client_addr) = listener
            .listener
            .as_ref()
            .unwrap() // safe
            .accept()
            .await
            .unwrap(); // os error
        let (rd, wr) = socket.into_split();

        info!("Accepted connection from {}.", client_addr);

        Self {
            client_addr,
            rd: Some(rd),
            wr: Some(wr),
            tx: listener.tx.clone(),
            tk: listener.tk.clone(),
        }
    }

    /// Spawn a task to write bytes from the broadcast channel to the socket.
    fn spawn_write(&mut self) -> &mut Self {
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

        info!("Connection from {} closed.", self.client_addr);
    }
}

#[cfg(test)]
mod tests {}
