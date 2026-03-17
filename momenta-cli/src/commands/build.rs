use crate::config::Config;
use std::process::Command;

pub fn run_build(
    server_only: bool,
    client_only: bool,
    no_wasm_opt: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    let (config, project_root) = Config::discover(&cwd)?;

    let build_client = !server_only;
    let build_server = !client_only;

    if build_client {
        println!(
            "building client WASM (optimization: {:?})",
            config.build.optimization
        );

        let status = Command::new("cargo")
            .args(["build", "--target", "wasm32-unknown-unknown", "--release"])
            .current_dir(&project_root)
            .status()?;

        if !status.success() {
            return Err("client WASM build failed".into());
        }

        // wasm-bindgen
        let wasm_name = config.project.name.replace('-', "_");
        let wasm_input = project_root
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("{wasm_name}.wasm"));
        let pkg_dir = project_root.join(&config.client.output_dir).join("pkg");
        std::fs::create_dir_all(&pkg_dir)?;

        let _ = Command::new("wasm-bindgen")
            .args([
                "--out-dir",
                &pkg_dir.to_string_lossy(),
                "--target",
                "web",
                &wasm_input.to_string_lossy(),
            ])
            .status();

        // wasm-opt (optional)
        if config.build.wasm_opt && !no_wasm_opt {
            let wasm_out = pkg_dir.join(format!("{wasm_name}_bg.wasm"));
            if wasm_out.exists() {
                let opt_flag = match config.build.optimization {
                    crate::config::Optimization::Size => "-Oz",
                    crate::config::Optimization::Speed => "-O3",
                    crate::config::Optimization::None => "-O0",
                };
                let _ = Command::new("wasm-opt")
                    .args([opt_flag, "-o"])
                    .arg(&wasm_out)
                    .arg(&wasm_out)
                    .status();
            }
        }

        println!("  client artifacts → {}", pkg_dir.display());
    }

    if build_server {
        println!("building server binary");

        let status = Command::new("cargo")
            .args([
                "build",
                "--features",
                "server",
                "--release",
                "--bin",
                &format!("{}-server", config.project.name),
            ])
            .current_dir(&project_root)
            .status()?;

        if !status.success() {
            return Err("server build failed".into());
        }

        println!(
            "  server binary → target/release/{}-server",
            config.project.name
        );
    }

    println!("build complete");
    Ok(())
}
