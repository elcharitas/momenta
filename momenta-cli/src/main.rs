mod cli;
mod commands;
mod config;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Init {
            path,
            name,
            backend,
        } => commands::run_init(path, name, backend),
        Command::Dev { port, open } => commands::run_dev(port, open),
        Command::Build {
            server_only,
            client_only,
            no_wasm_opt,
        } => commands::run_build(server_only, client_only, no_wasm_opt),
        Command::Format { check } => commands::run_format(check),
        Command::Check => commands::run_check(),
        Command::Start { port } => commands::run_start(port),
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
