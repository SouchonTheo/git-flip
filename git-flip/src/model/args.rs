use clap::Parser;

/// A CLI tool to switch between different Git/SSH configurations.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The account name to activate (e.g., 'perso' or 'work')
    pub account: String,
}
