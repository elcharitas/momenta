use crate::config::Config;
use std::process::Command;

pub fn run_dev(port: Option<u16>, open: bool) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    let (mut config, project_root) = Config::discover(&cwd)?;

    if let Some(p) = port {
        config.server.port = p;
    }
    if open {
        config.dev.open_browser = true;
    }

    println!(
        "starting dev server · backend={:?} · http://{}:{}",
        config.server.backend, config.server.host, config.server.port
    );
    println!(
        "  project: {} ({})",
        config.project.name,
        project_root.display()
    );
    println!("  watching: {:?}", config.dev.watch);
    println!("  hydration: {}", config.client.hydration);
    println!();

    // Phase 1: build client WASM
    let client_status = Command::new("cargo")
        .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
        .current_dir(&project_root)
        .status()?;

    if !client_status.success() {
        return Err("client WASM build failed".into());
    }

    // Phase 2: run wasm-bindgen
    let wasm_name = config.project.name.replace('-', "_");
    let wasm_input = project_root
        .join("target/wasm32-unknown-unknown/release")
        .join(format!("{wasm_name}.wasm"));
    let pkg_dir = project_root.join(&config.client.output_dir).join("pkg");
    std::fs::create_dir_all(&pkg_dir)?;

    let bindgen_status = Command::new("wasm-bindgen")
        .args([
            "--out-dir",
            &pkg_dir.to_string_lossy(),
            "--target",
            "web",
            &wasm_input.to_string_lossy(),
        ])
        .status();

    match bindgen_status {
        Ok(s) if s.success() => {}
        _ => {
            eprintln!("warning: wasm-bindgen not found or failed — skipping JS binding generation");
        }
    }

    // Phase 3: start server binary (feature = "server")
    let mut server = Command::new("cargo")
        .args([
            "run",
            "--features",
            "server",
            "--bin",
            &format!("{}-server", config.project.name),
        ])
        .current_dir(&project_root)
        .spawn()?;

    // TODO: file watcher + rebuild loop
    // TODO: open browser if config.dev.open_browser

    server.wait()?;
    Ok(())
}
