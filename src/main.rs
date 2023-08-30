use clap::Parser;
use human_panic::setup_panic;

use nc::cli;

#[tokio::main]
async fn main() {
    setup_panic!();

    nc::run(cli::Cli::parse()).await;
}
