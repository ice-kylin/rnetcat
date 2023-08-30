use clap::Parser;
use human_panic::setup_panic;

use rnc::cli;

#[tokio::main]
async fn main() {
    // Setup the human panic handler.
    setup_panic!();

    App::new().init_logger().run().await;
}

/// This struct represents the application.
struct App {
    cli: cli::Cli,
}

impl App {
    /// Create a new application.
    fn new() -> Self {
        let cli = cli::Cli::parse();
        Self { cli }
    }

    /// Initialize the logger.
    fn init_logger(&self) -> &Self {
        env_logger::Builder::new()
            .filter_level(self.cli.verbose.log_level_filter())
            .init();

        self
    }

    /// Run the application.
    async fn run(&self) {
        rnc::run(&self.cli).await;
    }
}
