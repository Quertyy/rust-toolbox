use clap::{command, Args, Parser, Subcommand};
use std::io::{stderr, Write};

/// Cli struct is used to parse the command line arguments
/// It's using the clap crate to do this (https:///docs.rs/clap/latest/clap/)
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// The subcommand to run
    #[command(subcommand)]
    pub command: Commands,
}

/// Commands enum is used to parse the subcommands
/// Commands are get, set, and rm
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Get subcommand
    #[command(name = "get")]
    Get(Get),
    /// Set subcommand
    #[command(name = "set")]
    /// Set struct is defined below
    Set(Set),
    /// Remove subcommand
    #[command(name = "rm")]
    Remove(Remove),
}

/// Get struct is used to parse the get subcommand
/// It has one field, key, which is the key to get
/// Example: kvs get key
#[derive(Args, Debug)]
pub struct Get {
    /// The key to get
    pub key: String,
}

/// Set struct is used to parse the set subcommand
/// It has two fields, key and value, which are the key and value to set
/// Example: kvs set key value
#[derive(Args, Debug)]
pub struct Set {
    /// The key to set
    pub key: String,
    /// The value to set
    pub value: String,
}

/// Remove struct is used to parse the rm subcommand
/// It has one field, key, which is the key to remove
/// Example: kvs rm key
#[derive(Args, Debug)]
pub struct Remove {
    /// The key to remove
    pub key: String,
}



fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get(_) => write!(&mut stderr(), "unimplemented").unwrap(),
        Commands::Set(_) => write!(&mut stderr(), "unimplemented").unwrap(),
        Commands::Remove(_) => write!(&mut stderr(), "unimplemented").unwrap(),
    };
    std::process::exit(1)
}
