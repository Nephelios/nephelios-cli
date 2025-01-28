use clap::{Parser, Subcommand};

/// Represents the main CLI configuration and command structure.
///
/// This struct is the entry point for command-line argument parsing
/// and contains all available commands and their respective options.
#[derive(Parser)]
#[command(
    name = "nephelios-cli",
    author = "Your Name <your.email@example.com>",
    version = "1.0.0",
    about = "Nephelios CLI tool for managing application deployments",
    long_about = "A command-line interface for managing your Nephelios deployments. Supports creating new deployments, \
                  managing existing ones, and deploying applications from GitHub repositories."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new application deployment
    ///
    /// This command creates a new deployment of your application on the Nephelios platform.
    /// It requires the application name, type, and GitHub repository URL.
    Create {
        /// Name of the application (e.g., my-awesome-app)
        #[arg(long, help = "Name of the application to deploy")]
        name: String,

        /// Type of the application (supported: nodejs, python, rust)
        #[arg(
            long,
            help = "Type of application to deploy",
            long_help = "Specify the type of application you're deploying. Currently supported types:\n\
                        - nodejs: For Node.js applications\n\
                        - python: For Python applications\n\
                        - rust: For Rust applications"
        )]
        type_: String,

        /// GitHub repository URL containing your application code
        #[arg(
            long,
            help = "URL of the GitHub repository containing your application",
            long_help = "The full HTTPS URL of your GitHub repository. The repository must be public \
                        or you must have configured appropriate access credentials."
        )]
        github_url: String,
    },
}
