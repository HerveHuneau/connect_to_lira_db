use std::process::Command;

use anyhow::{Context, Result};

use crate::{cli::Environment, credentials::Credentials};

pub struct Config {
    pub db_host: String,
    pub db_name: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new(env: &Environment, db_name: &str, credentials: Credentials) -> Self {
        let db_host = match env {
            Environment::Local => LOCAL_HOST,
            Environment::Staging => STAGING_HOST,
            Environment::Production => PROD_HOST,
        }
        .to_string();
        Self {
            db_host,
            db_name: db_name.to_string(),
            username: credentials.0,
            password: credentials.1,
        }
    }
}

pub fn connect_to_db(config: Config) -> Result<()> {
    Command::new("pgcli")
        .arg(format!(
            "postgres://{}:{}@{}/{}",
            config.username, config.password, config.db_host, config.db_name
        ))
        .status()
        .context("Failed to connect to the database")
        .map(|_| ())
}

const LOCAL_HOST: &str = "postgres:15432";
const STAGING_HOST: &str = "staging-payments-aurora-cluster-staging-cluster.cluster-cffiig2xe4a8.eu-west-1.rds.amazonaws.com";
const PROD_HOST: &str = "db-payments-production.helloprima.co.uk";
