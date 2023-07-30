use kvs::{Cli, Commands};
use clap::Parser;
use std::io::{Write, stderr};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get(_) => write!(&mut stderr(), "unimplemented").unwrap(),
        Commands::Set(_) => write!(&mut stderr(), "unimplemented").unwrap(),
        Commands::Remove(_) => write!(&mut stderr(), "unimplemented").unwrap(),
    };
    std::process::exit(1)
}