use clap::Parser;
use human_panic::setup_panic;

use rnc::cli;

/// This struct represents the application.
struct App {
    cli: cli::Cli,
}

#[tokio::main]
async fn main() {
    // Setup the human panic handler.
    setup_panic!();

    App::new().reorder().init_logger().run().await;
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

    /// Reorder the hostname and port.
    fn reorder(&mut self) -> &mut Self {
        if self.cli.listen && self.cli.hostname.is_some() && self.cli.port.is_none() {
            if let Ok(port) = self.cli.hostname.as_ref().unwrap().parse::<u16>() {
                self.cli.hostname = None;
                self.cli.port = Some(port);
            }
        }

        self
    }

    /// Run the application.
    async fn run(&self) {
        rnc::run(&self.cli).await;
    }
}
