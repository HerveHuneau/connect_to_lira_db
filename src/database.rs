use std::process::Command;

use anyhow::{Context, Result};

use crate::{
    cli::{Args, Environment::*},
    credentials::Credentials,
};

pub struct Config {
    pub db_host: String,
    pub db_name: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new(args: Args, credentials: Credentials) -> Self {
        let db_host = match args.environment {
            Local => LOCAL_HOST,
            Staging => STAGING_HOST,
            Production => PROD_HOST,
        };
        Self {
            db_host: db_host.to_string(),
            db_name: args.db_name,
            username: credentials.username,
            password: credentials.password,
        }
    }

    pub fn connect(&self) -> Result<()> {
        Command::new("pgcli")
            .arg(format!(
                "postgres://{}:{}@{}/{}",
                self.username, self.password, self.db_host, self.db_name
            ))
            .env("PGOPTIONS", format!("--search_path={}", self.db_name))
            .status()
            .context("Failed to connect to the database")
            .map(|_| ())
    }
}

const LOCAL_HOST: &str = "postgres:15432";
const STAGING_HOST: &str = "staging-payments-aurora-cluster-staging-cluster.cluster-cffiig2xe4a8.eu-west-1.rds.amazonaws.com";
const PROD_HOST: &str = "db-payments-production.helloprima.co.uk";
