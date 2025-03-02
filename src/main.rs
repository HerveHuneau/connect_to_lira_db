use anyhow::Result;
use clap::Parser;
use cli::Args;
use credentials::Credentials;
use database::Config;

mod cli;
mod credentials;
mod database;

fn main() -> Result<()> {
    let args = Args::parse();
    let credentials = Credentials::try_from(&args)?;
    Config::new(args, credentials).connect()
}
