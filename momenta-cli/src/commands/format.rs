use crate::config::Config;
use std::process::Command;

pub fn run_format(check: bool) -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    let (_config, project_root) = Config::discover(&cwd)?;

    let mut args = vec!["fmt"];
    if check {
        args.push("--check");
    }

    let status = Command::new("cargo")
        .args(&args)
        .current_dir(&project_root)
        .status()?;

    if !status.success() {
        if check {
            return Err("formatting check failed — run `momenta format` to fix".into());
        }
        return Err("formatting failed".into());
    }

    // TODO: RSX-specific formatting pass

    if check {
        println!("formatting ok");
    } else {
        println!("formatted");
    }
    Ok(())
}
