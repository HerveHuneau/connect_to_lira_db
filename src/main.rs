use anyhow::Result;
use clap::Parser;
use cli::Args;
use credentials::Credentials;
use database::{Config, connect_to_db};

mod cli;
mod credentials;
mod database;

fn main() -> Result<()> {
    let args = Args::parse();
    let credentials = Credentials::try_from(&args.environment)?;
    let config = Config::new(&args.environment, &args.db_name, credentials);
    connect_to_db(config)
}
