mod model;
mod utils;

use clap::Parser;
use std::io::{Error, ErrorKind};
use std::{env, fs, path::PathBuf, process::Command};

use model::{Args, Config};
use utils::expand_tilde;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments.
    let args = Args::parse();

    // Construct the path to the configuration file.
    let home = env::var("HOME")?;
    let config_path = PathBuf::from(home).join(".config/git-flip/config.toml");

    // Read and parse the configuration file.
    let config_str =
        fs::read_to_string(&config_path).expect("Could not read the configuration file");
    let config: Config = toml::from_str(&config_str).expect("Error parsing the configuration file");

    // Retrieve the account configuration.
    let account_config = config.accounts.get(&args.account).ok_or_else(|| {
        Error::new(
            ErrorKind::NotFound,
            format!("Account '{}' not found in the configuration.", args.account),
        )
    })?;

    println!("Switching to '{}' account...", args.account);

    // 1. Remove all existing SSH keys.
    let status = Command::new("ssh-add").arg("-D").status()?;
    if !status.success() {
        eprintln!("Failed to remove SSH keys.");
    }

    // 2. Add the new SSH key (using the Apple keychain option as an example).
    let key_path = expand_tilde(&account_config.ssh_key);
    let status = Command::new("ssh-add")
        .arg("--apple-use-keychain")
        .arg(&key_path)
        .status()?;
    if !status.success() {
        eprintln!("Failed to add the SSH key at '{}'.", key_path);
        std::process::exit(1);
    }

    // 3. Set Git user.name and user.email.
    let status = Command::new("git")
        .args(&["config", "user.name", &account_config.git_username])
        .status()?;
    if !status.success() {
        eprintln!("Failed to set Git username.");
        std::process::exit(1);
    }

    let status = Command::new("git")
        .args(&["config", "user.email", &account_config.git_email])
        .status()?;
    if !status.success() {
        eprintln!("Failed to set Git email.");
        std::process::exit(1);
    }

    println!("Switching is done for the account: {}", args.account);
    Ok(())
}
