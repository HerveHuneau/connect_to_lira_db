use std::process::Command;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Environment
    #[arg(short, long)]
    environment: String,

    /// Database name
    #[arg(short, long)]
    db_name: String,
}

fn main() {
    let args = Args::parse();

    if args.environment == "local" {
        Command::new("pgcli")
            .arg(format!(
                "postgres://postgres:postgres@postgres:15432/{}",
                args.db_name
            ))
            .status()
            .expect("pgcli didn't finish successfully");
    } else {
        todo!()
    }
}
