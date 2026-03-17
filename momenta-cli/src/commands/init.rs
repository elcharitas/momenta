use crate::config::{Backend, Config, ProjectConfig, ServerConfig};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn run_init(path: PathBuf, name: Option<String>, backend: String) -> Result<(), Box<dyn std::error::Error>> {
    let target = if path == Path::new(".") {
        std::env::current_dir()?
    } else {
        let abs = std::env::current_dir()?.join(&path);
        fs::create_dir_all(&abs)?;
        abs
    };

    let project_name = name.unwrap_or_else(|| {
        target
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "momenta-app".into())
    });

    let backend_enum: Backend = match backend.as_str() {
        "actix" => Backend::Actix,
        "hyper" => Backend::Hyper,
        _ => Backend::Axum,
    };

    // Guard against re-initialization
    let config_path = target.join("momenta.toml");
    if config_path.exists() {
        return Err(format!(
            "momenta.toml already exists in {}. Aborting.",
            target.display()
        )
        .into());
    }

    // Build the config with user choices
    let mut config = Config::default();
    config.project = ProjectConfig {
        name: project_name.clone(),
    };
    config.server = ServerConfig {
        backend: backend_enum,
        ..ServerConfig::default()
    };

    // Write momenta.toml
    fs::write(&config_path, config.to_toml()?)?;

    // Scaffold directories
    let src = target.join("src");
    fs::create_dir_all(&src)?;
    fs::create_dir_all(target.join("static"))?;

    // Cargo.toml
    write_if_missing(
        &target.join("Cargo.toml"),
        &cargo_toml(&project_name),
    )?;

    // src/main.rs — client entry point
    write_if_missing(&src.join("main.rs"), CLIENT_MAIN)?;

    // src/app.rs — shared app component
    write_if_missing(&src.join("app.rs"), APP_COMPONENT)?;

    // src/server.rs — Axum server entry (only for axum backend)
    write_if_missing(&src.join("server.rs"), SERVER_MAIN)?;

    // static/index.html — shell page
    write_if_missing(
        &target.join("static").join("index.html"),
        &index_html(&project_name),
    )?;

    println!("✔ Created Momenta project '{project_name}' in {}", target.display());
    println!();
    println!("  cd {}", path.display());
    println!("  momenta dev");
    println!();

    Ok(())
}

fn write_if_missing(path: &Path, content: &str) -> io::Result<()> {
    if !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}

fn cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[dependencies]
momenta = {{ version = "0.3", features = ["dom", "full-reactivity", "wasm"] }}
momenta-ssr = {{ version = "0.3", features = ["axum"], optional = true }}
axum = {{ version = "0.8", optional = true }}
tokio = {{ version = "1", features = ["full"], optional = true }}
wasm-bindgen = "0.2"

[features]
default = []
server = ["momenta-ssr/axum", "axum", "tokio"]

[[bin]]
name = "{name}"
path = "src/main.rs"

[[bin]]
name = "{name}-server"
path = "src/server.rs"
required-features = ["server"]
"#
    )
}

const CLIENT_MAIN: &str = r#"use momenta::prelude::*;

mod app;

fn main() {
    momenta::mount(app::App);
}
"#;

const APP_COMPONENT: &str = r#"use momenta::prelude::*;

#[component]
pub fn App() -> Node {
    let (count, set_count) = use_signal(0);

    rsx!(
        <main>
            <h1>Welcome to Momenta</h1>
            <button on:click={move |_| set_count.update(|n| *n += 1)}>
                {"Count: "}{count}
            </button>
        </main>
    )
}
"#;

const SERVER_MAIN: &str = r#"#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::{Router, routing::get};
    use momenta_ssr::render_to_string;

    mod app;

    let router = Router::new().route("/", get(|| async {
        let html = render_to_string(|| {
            use momenta::prelude::*;
            app::App(())
        });
        axum::response::Html(html)
    }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind");
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, router).await.expect("server error");
}

#[cfg(not(feature = "server"))]
fn main() {
    eprintln!("This binary requires the 'server' feature. Build with: cargo build --features server");
    std::process::exit(1);
}
"#;

fn index_html(name: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{name}</title>
</head>
<body data-momenta-root>
    <script type="module">
        import init from './pkg/{name}.js';
        init();
    </script>
</body>
</html>
"#
    )
}
