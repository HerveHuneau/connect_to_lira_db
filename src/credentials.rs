use std::process::{Command, Output};

use anyhow::{Context, Result};
use regex::Regex;

use crate::cli::{Args, Environment};

pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl TryFrom<&Args> for Credentials {
    type Error = anyhow::Error;

    fn try_from(value: &Args) -> std::result::Result<Self, Self::Error> {
        match value.environment {
            Environment::Local => Ok(get_local_credentials()),
            Environment::Staging | Environment::Production => {
                get_remote_credentials(&value.environment)
            }
        }
    }
}

fn get_local_credentials() -> Credentials {
    Credentials::new("postgres".to_string(), "postgres".to_string())
}

fn get_remote_credentials(environment: &Environment) -> Result<Credentials> {
    let credentials = fetch_credentials(environment)?;
    let credentials_output = match credentials.status.code() {
        Some(0) => String::from_utf8_lossy(&credentials.stdout).to_string(),
        Some(2) => {
            login()?;
            let credentials = fetch_credentials(environment)?;
            String::from_utf8_lossy(&credentials.stdout).to_string()
        }
        _ => Err(anyhow::anyhow!(
            "Failed to fetch credentials: {}",
            String::from_utf8_lossy(&credentials.stderr)
        ))?,
    };
    parse_credentials(credentials_output)
}

fn fetch_credentials(environment: &Environment) -> Result<Output> {
    Command::new("vault")
        .arg("read")
        .arg(format!(
            "database/payments/creds/SWEngineerDomainPaymentsTeamCreditCardProcessing-Onyx-payments-{}-postgres",
            environment.to_string().to_lowercase()
        ))
        .output()
        .context("vault read didn't finish successfully")
}

fn login() -> Result<()> {
    Command::new("vault")
        .arg("login")
        .arg("-address=https://vault.helloprima.com:8200/")
        .arg("-method=oidc")
        .arg("-path=okta")
        .status()
        .context("vault login didn't finish successfully")
        .and_then(|status| match status.code() {
            Some(0) => Ok(()),
            _ => Err(anyhow::anyhow!("Failed to login to vault")),
        })
}

fn parse_credentials(output_str: String) -> Result<Credentials> {
    let re_password = Regex::new(r"(?m)^password\s+(\S+)\s*$").unwrap();
    let re_username = Regex::new(r"(?m)^username\s+(\S+)\s*$").unwrap();
    let username = re_username
        .captures(&output_str)
        .and_then(|cap| cap.get(1).map(|m| m.as_str()))
        .context("Could not parse username")?;
    let password = re_password
        .captures(&output_str)
        .and_then(|cap| cap.get(1).map(|m| m.as_str()))
        .context("Could not parse password")?;
    Ok(Credentials::new(username.to_owned(), password.to_owned()))
}
