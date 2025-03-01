use std::process::Command;

use clap::Parser;
use regex::Regex;

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
        let credentials = Command::new("vault")
            .arg("read")
            .arg(format!(
                "database/payments/creds/SWEngineerDomainPaymentsTeamCreditCardProcessing-Onyx-payments-{}-postgres",
                args.environment
            ))
            .output()
            .expect("vault read didn't finish successfully");

        let output_str = String::from_utf8_lossy(&credentials.stdout);

        // Define regex patterns to capture username and password
        let re_password = Regex::new(r"(?m)^password\s+(\S+)\s*$").unwrap();
        let re_username = Regex::new(r"(?m)^username\s+(\S+)\s*$").unwrap();

        // Extract password
        let password = re_password
            .captures(&output_str)
            .and_then(|cap| cap.get(1).map(|m| m.as_str()));

        // Extract username
        let username = re_username
            .captures(&output_str)
            .and_then(|cap| cap.get(1).map(|m| m.as_str()));

        // Print the extracted username and password
        if let (Some(username), Some(password)) = (username, password) {
            let db_host = match args.environment.as_str() {
                "production" => "db-payments-production.helloprima.co.uk",
                "staging" => {
                    "staging-payments-aurora-cluster-staging-cluster.cluster-cffiig2xe4a8.eu-west-1.rds.amazonaws.com"
                }
                _ => panic!("Unknown environment"),
            };

            Command::new("pgcli")
                .arg(format!(
                    "postgres://{}:{}@{}/{}",
                    username, password, db_host, args.db_name
                ))
                .status()
                .expect("pgcli didn't finish successfully");
        } else {
            println!("Username or password not found");
        }
    }
}
