use std::net::SocketAddr;
use std::process;

use log::{error, info};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

use crate::cli;
use crate::{cancellable_task, util};

use super::parse;

pub async fn start_listener(cli: &cli::Cli) {
    let socket_addr = match parse::parse_socket_addr(cli) {
        Ok(socket_addr) => socket_addr,
        Err(e) => {
            error!("{}. QUITTING", e);
            process::exit(exitcode::USAGE);
        }
    };

    match tcp_listener_from(&socket_addr).await {
        Ok(listener) => {
            listening_on(&listener, &socket_addr, cli).await;
        }
        Err(e) => {
            error!("Bind to {}: {}. QUITTING", socket_addr, e);
            process::exit(exitcode::NOHOST);
        }
    };
}

async fn tcp_listener_from(socket_addr: &SocketAddr) -> io::Result<TcpListener> {
    TcpListener::bind(socket_addr).await
}

async fn listening_on(listener: &TcpListener, socket_addr: &SocketAddr, cli: &cli::Cli) {
    let (tx, _) = broadcast::channel(16);
    let tk = CancellationToken::new();

    let tx_clone = tx.clone();
    util::spawn_cancellable_task(&tk, async move {
        let mut buffer = vec![0; 1024];
        loop {
            let n = io::stdin().read(&mut buffer).await.unwrap(); // os error

            if n == 0 || tx_clone.send(buffer[..n].to_owned()).is_err() {
                break;
            }
        }
    });

    info!("Listening on {}.", socket_addr);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let (mut rd, mut wr) = socket.into_split();

        info!("Accepted connection from {}.", addr);

        let mut rx = tx.subscribe();
        util::spawn_cancellable_task(&tk, async move {
            while let Ok(msg) = rx.recv().await {
                if wr.write_all(&msg).await.is_err() {
                    break;
                }
            }
        });

        let task = cancellable_task!(tk, async move {
            io::copy(&mut rd, &mut io::stdout()).await.unwrap();

            info!("Connection from {} closed.", addr);
        });

        if cli.keep_open {
            tokio::spawn(task);
        } else {
            task.await;
            tk.cancel();

            break;
        }
    }

    info!("Listener on {} closed.", socket_addr);
}
