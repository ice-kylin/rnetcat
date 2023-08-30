pub mod network;
pub mod cli;
mod parse;

pub async fn run(cli: cli::Cli) {
    let host_addr = match parse::parse_host_addr(
        cli.listen,
        &cli.hostname,
        cli.port,
    ) {
        Ok(host_addr) => host_addr,
        Err(e) => {
            eprintln!("{}. QUITTING", e);
            std::process::exit(exitcode::USAGE);
        }
    };

    if cli.listen {
        network::listener::start_listener(host_addr, &cli).await;
    } else {
        network::connector::connect_to_server().await;
    }
}
