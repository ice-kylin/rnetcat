use std::net::SocketAddr;

use tokio::{io, net};

use crate::{cli, parse};
use crate::parse::HostAddr;

///
pub async fn start_listener(host_addr:HostAddr, cli: &cli::Cli) {
    let listener = match host_addr.hostname {
        parse::Hostname::Name(ref host) => net::TcpListener::bind(
            format!("{}:{}", host, host_addr.port)
        ).await,
        parse::Hostname::Addr(addr) => net::TcpListener::bind(
            SocketAddr::new(addr, host_addr.port)
        ).await
    };

    match listener {
        Ok(listener) => {
            println!("Listening on {}.", host_addr);

            loop {
                let (socket, addr) = listener.accept().await.unwrap();
                println!("Accepted connection from {}.", addr);

                if cli.keep_open {
                    tokio::spawn(async move {
                        process_incoming_tcp(socket, &mut io::stdout()).await;
                    });
                } else {
                    process_incoming_tcp(socket, &mut io::stdout()).await;
                }
            }
        }
        Err(e) => {
            eprintln!("bind to {}: {}. QUITTING", host_addr, e);
            std::process::exit(exitcode::NOHOST);
        }
    };
}


///
pub async fn process_incoming_tcp<W>(mut socket: net::TcpStream, writer: &mut W)
    where W: io::AsyncWrite + Unpin + ?Sized,
{
    io::copy(&mut socket, writer)
        .await
        .unwrap();
}
