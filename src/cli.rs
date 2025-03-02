use clap::Parser;
use clap::ValueEnum;
use std::fmt;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Environment
    #[arg(short, long)]
    pub environment: Environment,

    /// Database name
    #[arg(short, long)]
    pub db_name: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Environment {
    Local,
    Staging,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
