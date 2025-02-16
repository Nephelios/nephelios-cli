mod commands;
mod types;
mod utils;

use crate::types::cli::{Cli, Commands};
use clap::Parser;

/// Main entry point for the Nephelios CLI application.
/// Parses command line arguments and executes the appropriate command.
///
/// # Returns
///
/// * `Ok(())` if the command executed successfully
/// * `Err(anyhow::Error)` if there was an error during execution
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create {
            name,
            type_,
            github_url,
        } => {
            commands::create::execute(name, type_, github_url).await?;
        },

        Commands::Remove {
            name,
        } => {
            commands::remove::execute(name).await?;
        },

        Commands::Stop {
            name,
        } => {
            commands::stop::execute(name).await?;
        },

        Commands::Start {
            name,
        } => {
            commands::start::execute(name).await?;
        }

    }

    Ok(())
}
