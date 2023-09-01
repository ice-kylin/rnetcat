use std::process;

use log::info;

use network::{connector, listener};

pub mod cli;
pub mod network;
pub mod util;

/// Run the application.
///
/// # Arguments
///
/// * `cli` - The command line arguments.
pub async fn run(cli: &cli::Cli) {
    print_info();
    process(cli).await;
    process::exit(exitcode::OK);
}

/// Print the application information.
fn print_info() {
    info!(
        "{} ({}) {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_HOMEPAGE"),
        env!("CARGO_PKG_VERSION")
    );
}

/// Process the application.
///
/// # Arguments
///
/// * `cli` - The command line arguments.
async fn process(cli: &cli::Cli) {
    if cli.listen {
        listener::start_listener(&cli).await;
    } else {
        connector::connect_to_server().await;
    }
}
