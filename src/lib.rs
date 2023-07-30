use std::collections::HashMap;
use clap::{Subcommand, Parser, Args, command};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "get")]
    Get(Get),
    #[command(name = "set")]
    Set(Set),
    #[command(name = "rm")]
    Remove(Remove),
}

#[derive(Args, Debug)]
pub struct Get {
    key: String
}

#[derive(Args, Debug)]
pub struct Set {
    key: String,
    value: String
}

#[derive(Args, Debug)]
pub struct Remove {
    key: String
}

pub struct KvStore {
    pub items: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.items.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.items.remove(&key);
    }
}