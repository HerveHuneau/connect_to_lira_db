use std::{fmt, process::Command};

use Environment::*;
use clap::{Parser, ValueEnum};
use regex::Regex;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Environment
    #[arg(short, long)]
    environment: Environment,

    /// Database name
    #[arg(short, long)]
    db_name: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Environment {
    Local,
    Staging,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Config {
    db_host: String,
    db_name: String,
    username: String,
    password: String,
}

impl Config {
    fn new(db_host: &str, db_name: &str, username: &str, password: &str) -> Self {
        Self {
            db_host: db_host.to_string(),
            db_name: db_name.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

fn main() {
    let args = Args::parse();
    let config = get_config(args);
    connect_to_db(config);
}

fn get_config(args: Args) -> Config {
    match args.environment {
        Local => Config::new(LOCAL_HOST, "postgres", "postgres", &args.db_name),
        Staging => {
            let (username, password) = get_credentials(&args.environment);
            Config::new(STAGING_HOST, &username, &password, &args.db_name)
        }
        Production => {
            let (username, password) = get_credentials(&args.environment);
            Config::new(PROD_HOST, &username, &password, &args.db_name)
        }
    }
}

fn get_credentials(environment: &Environment) -> (String, String) {
    let credentials = fetch_credentials(environment);
    let credentials_output = match credentials.status.code() {
        Some(0) => String::from_utf8_lossy(&credentials.stdout).to_string(),
        Some(2) => {
            login();
            let credentials = fetch_credentials(environment);
            String::from_utf8_lossy(&credentials.stdout).to_string()
        }
        _ => {
            panic!("vault read didn't finish successfully");
        }
    };
    parse_credentials(credentials_output)
}

fn fetch_credentials(environment: &Environment) -> std::process::Output {
    let credentials = Command::new("vault")
        .arg("read")
        .arg(format!(
            "database/payments/creds/SWEngineerDomainPaymentsTeamCreditCardProcessing-Onyx-payments-{}-postgres",
            environment.to_string().to_lowercase()
        ))
        .output()
        .expect("vault read didn't finish successfully");
    credentials
}

fn login() {
    let _login = Command::new("vault")
        .arg("login")
        .arg("-address=https://vault.helloprima.com:8200/")
        .arg("-method=oidc")
        .arg("-path=okta")
        .status()
        .expect("vault login didn't finish successfully");
}

fn parse_credentials(output_str: String) -> (String, String) {
    let re_password = Regex::new(r"(?m)^password\s+(\S+)\s*$").unwrap();
    let re_username = Regex::new(r"(?m)^username\s+(\S+)\s*$").unwrap();
    let username = re_username
        .captures(&output_str)
        .and_then(|cap| cap.get(1).map(|m| m.as_str()))
        .expect("Could not retrieve username");
    let password = re_password
        .captures(&output_str)
        .and_then(|cap| cap.get(1).map(|m| m.as_str()))
        .expect("Could not retrieve password");
    (username.to_owned(), password.to_owned())
}

fn connect_to_db(config: Config) {
    Command::new("pgcli")
        .arg(format!(
            "postgres://{}:{}@{}/{}",
            config.username, config.password, config.db_host, config.db_name
        ))
        .status()
        .expect("pgcli didn't finish successfully");
}

const LOCAL_HOST: &str = "postgres:15432";
const STAGING_HOST: &str = "staging-payments-aurora-cluster-staging-cluster.cluster-cffiig2xe4a8.eu-west-1.rds.amazonaws.com";
const PROD_HOST: &str = "db-payments-production.helloprima.co.uk";
