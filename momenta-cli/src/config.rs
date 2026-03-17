use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Root configuration read from `momenta.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub project: ProjectConfig,
    pub server: ServerConfig,
    pub client: ClientConfig,
    pub build: BuildConfig,
    pub dev: DevConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProjectConfig {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    /// Backend framework: "axum", "actix", or "hyper".
    pub backend: Backend,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Axum,
    Actix,
    Hyper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ClientConfig {
    /// Crate entry file for the client WASM target.
    pub entry: PathBuf,
    /// Directory where built client artifacts are placed.
    pub output_dir: PathBuf,
    /// Whether to emit hydration markers in SSR output.
    pub hydration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildConfig {
    /// Optimization strategy: "size" or "speed".
    pub optimization: Optimization,
    /// Run wasm-opt on the output.
    pub wasm_opt: bool,
    /// Extra rustflags passed to client builds.
    pub rustflags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Optimization {
    Size,
    Speed,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DevConfig {
    /// Directories to watch for changes.
    pub watch: Vec<PathBuf>,
    /// Automatically reload the browser on rebuild.
    pub auto_reload: bool,
    /// Open a browser on startup.
    pub open_browser: bool,
}

// ── Defaults ────────────────────────────────────────────────────────

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig::default(),
            server: ServerConfig::default(),
            client: ClientConfig::default(),
            build: BuildConfig::default(),
            dev: DevConfig::default(),
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: String::from("momenta-app"),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            backend: Backend::Axum,
            host: String::from("127.0.0.1"),
            port: 3000,
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::Axum
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            entry: PathBuf::from("src/main.rs"),
            output_dir: PathBuf::from("dist"),
            hydration: true,
        }
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            optimization: Optimization::Size,
            wasm_opt: true,
            rustflags: Vec::new(),
        }
    }
}

impl Default for Optimization {
    fn default() -> Self {
        Self::Size
    }
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            watch: vec![PathBuf::from("src")],
            auto_reload: true,
            open_browser: false,
        }
    }
}

// ── Loading ─────────────────────────────────────────────────────────

pub const CONFIG_FILE: &str = "momenta.toml";

impl Config {
    /// Search for `momenta.toml` starting at `start` and walking up to the
    /// filesystem root. Returns the parsed config and the directory it was
    /// found in.
    pub fn discover(start: &Path) -> Result<(Self, PathBuf), ConfigError> {
        let mut dir = start.to_path_buf();
        loop {
            let candidate = dir.join(CONFIG_FILE);
            if candidate.is_file() {
                let raw = std::fs::read_to_string(&candidate)
                    .map_err(|e| ConfigError::Io(candidate.clone(), e))?;
                let cfg: Config =
                    toml::from_str(&raw).map_err(|e| ConfigError::Parse(candidate.clone(), e))?;
                return Ok((cfg, dir));
            }
            if !dir.pop() {
                break;
            }
        }
        Err(ConfigError::NotFound)
    }

    /// Serialize the config back to TOML.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    NotFound,
    Io(PathBuf, std::io::Error),
    Parse(PathBuf, toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "no momenta.toml found in current or parent directories"),
            Self::Io(path, e) => write!(f, "failed to read {}: {e}", path.display()),
            Self::Parse(path, e) => write!(f, "failed to parse {}: {e}", path.display()),
        }
    }
}

impl std::error::Error for ConfigError {}
