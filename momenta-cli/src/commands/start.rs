use crate::config::Config;
use std::process::Command;

pub fn run_start(port: Option<u16>) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    let (mut config, project_root) = Config::discover(&cwd)?;

    if let Some(p) = port {
        config.server.port = p;
    }

    let server_bin = project_root
        .join("target/release")
        .join(format!("{}-server", config.project.name));

    if !server_bin.exists() {
        return Err(format!(
            "server binary not found at {}. Run `momenta build` first.",
            server_bin.display()
        )
        .into());
    }

    println!(
        "starting {} · http://{}:{}",
        config.project.name, config.server.host, config.server.port
    );

    let status = Command::new(&server_bin)
        .env("HOST", &config.server.host)
        .env("PORT", config.server.port.to_string())
        .current_dir(&project_root)
        .status()?;

    if !status.success() {
        return Err("server exited with an error".into());
    }

    Ok(())
}
