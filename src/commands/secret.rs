use clap::CommandFactory;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs};

use crate::cli::Cli;

#[derive(Debug, Deserialize)]
pub struct SecretCommand
{
        pub description: String,
        pub command: String,
}

pub fn load_secret_commands() -> Result<HashMap<String, SecretCommand>, Box<dyn Error>>
{
        let toml_str = fs::read_to_string(".secret.toml")?;

        let map: HashMap<String, SecretCommand> = toml::from_str(&toml_str)?;

        Ok(map)
}

pub fn run_secret_command(secrets: &HashMap<String, SecretCommand>,
                          name: Option<String>,
                          _args: Vec<String>)
                          -> Result<(), Box<dyn Error>>
{
        if let Some(secret_name) = name
        {
                if let Some(secret) = secrets.get(&secret_name)
                {
                        println!("Running secret command: {}", secret.description);
                        let status = std::process::Command::new("powershell").arg("-c")
                                                                             .arg(&secret.command)
                                                                             .args(_args)
                                                                             .status()?;
                        std::process::exit(status.code().unwrap_or(1));
                }
                else
                {
                        eprintln!("Unknown secret command: {}", secret_name);

                        show_help_with_secrets();

                        std::process::exit(1);
                }
        }
        else
        {
                Cli::command().print_help()?;

                println!();

                std::process::exit(0);
        }
}

pub fn show_help_with_secrets()
{
        let mut cmd = Cli::command();

        cmd.print_help().unwrap();

        println!("\n\nSecret commands (from .secret.toml):");

        match load_secret_commands()
        {
                Ok(secrets) =>
                {
                        for (key, secret) in secrets
                        {
                                println!("  {key:<15} - {}", secret.description);
                        }
                }
                Err(_) =>
                {
                        println!("  (None found or failed to load .secret.toml)");
                }
        }
}
