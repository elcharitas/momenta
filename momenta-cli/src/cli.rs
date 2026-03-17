use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "momenta",
    about = "The Momenta framework CLI",
    version,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new Momenta project
    Init {
        /// Directory to initialize (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Project name (defaults to directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Backend framework
        #[arg(short, long, default_value = "axum")]
        backend: String,
    },

    /// Start the development server with file watching
    Dev {
        /// Override the port from momenta.toml
        #[arg(short, long)]
        port: Option<u16>,

        /// Open browser on startup
        #[arg(long)]
        open: bool,
    },

    /// Build the project for production
    Build {
        /// Build only the server target
        #[arg(long)]
        server_only: bool,

        /// Build only the client WASM target
        #[arg(long)]
        client_only: bool,

        /// Skip wasm-opt
        #[arg(long)]
        no_wasm_opt: bool,
    },

    /// Format Rust and RSX source files
    Format {
        /// Check formatting without writing changes
        #[arg(long)]
        check: bool,
    },

    /// Validate project configuration and compilation
    Check,

    /// Run the production build
    Start {
        /// Override the port from momenta.toml
        #[arg(short, long)]
        port: Option<u16>,
    },
}
