use crate::config::Config;
use std::process::Command;

pub fn run_check() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    let (config, project_root) = Config::discover(&cwd)?;

    println!("checking project '{}'", config.project.name);

    // 1. Validate config (already parsed successfully if we're here)
    println!("  ✔ momenta.toml valid");

    // 2. Check Rust compilation for client target
    print!("  checking client (wasm32) … ");
    let client = Command::new("cargo")
        .args(["check", "--target", "wasm32-unknown-unknown"])
        .current_dir(&project_root)
        .output()?;

    if client.status.success() {
        println!("ok");
    } else {
        println!("FAILED");
        let stderr = String::from_utf8_lossy(&client.stderr);
        eprintln!("{stderr}");
        return Err("client compilation check failed".into());
    }

    // 3. Check server compilation
    print!("  checking server … ");
    let server = Command::new("cargo")
        .args(["check", "--features", "server"])
        .current_dir(&project_root)
        .output()?;

    if server.status.success() {
        println!("ok");
    } else {
        println!("FAILED");
        let stderr = String::from_utf8_lossy(&server.stderr);
        eprintln!("{stderr}");
        return Err("server compilation check failed".into());
    }

    // 4. Check formatting
    let fmt = Command::new("cargo")
        .args(["fmt", "--check"])
        .current_dir(&project_root)
        .output()?;

    if fmt.status.success() {
        println!("  ✔ formatting ok");
    } else {
        println!("  ⚠ formatting issues found — run `momenta format`");
    }

    println!("all checks passed");
    Ok(())
}
