mod cli;
mod core;
mod engine;

use clap::Parser;
use cli::args::CliArgs;
use cli::run_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    if let Some(command) = args.command {
        run_cli(command).await?;
    } else {
        // This is where we will eventually launch the TUI
        println!("TUI mode not yet implemented. Use --help for CLI usage.");
    }

    Ok(())
}
