use std::path::PathBuf;

use clap::CommandFactory;

#[path = "src/cli.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(
        std::env::var_os("OUT_DIR"
        )
            .ok_or_else(
                || std::io::ErrorKind::NotFound
            )?
    );

    generate_man_page(&out_dir)
}

fn generate_man_page(out_dir: &PathBuf) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Default::default();

    clap_mangen::Man::new(
        cli::Cli::command()
    ).render(&mut buffer)?;

    std::fs::write(out_dir.join("nc.1"), buffer)?;

    Ok(())
}
