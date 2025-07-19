use clap::CommandFactory;
use dirs_next::home_dir;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use crate::{cli::Cli, config::Config};

#[derive(Debug, Deserialize)]
pub struct SecretCommand
{
        pub description: String,
        pub command: String,
}

pub fn load_secret_commands() -> Result<HashMap<String, SecretCommand>, Box<dyn Error>>
{
        let home = home_dir().ok_or("Could not find home directory")?;

        let secret_path: PathBuf = home.join(".config/")
                                       .join(".grit/")
                                       .join(".grit-secret.toml");

        let toml_str = fs::read_to_string(&secret_path).map_err(|e| {
                               format!("Failed to read secret file at {secret_path:?}: {e}")
                       })?;

        let map: HashMap<String, SecretCommand> =
                toml::from_str(&toml_str).map_err(|e| format!("Failed to parse TOML: {e}"))?;

        Ok(map)
}

pub fn run_secret_command(config: &Config,
                          secrets: &HashMap<String, SecretCommand>,
                          name: Option<String>,
                          _args: Vec<String>)
                          -> Result<(), Box<dyn Error>>
{
        if let Some(secret_name) = name
        {
                if let Some(secret) = secrets.get(&secret_name)
                {
                        println!("Running secret command: {}", secret.description);
                        let status = std::process::Command::new(config.default_shell()).arg("-c")
                                                                 .arg(&secret.command)
                                                                 .args(_args)
                                                                 .status()?;
                        std::process::exit(status.code().unwrap_or(1));
                }
                else
                {
                        eprintln!("Unknown secret command: {secret_name}");

                        list_secrets()?;

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

pub fn list_secrets() -> Result<(), Box<dyn Error>>
{
        println!("Secret commands (from ~/.config/.grit/.grit-secret.toml):\n");

        match load_secret_commands()
        {
                Ok(secrets) =>
                {
                        if secrets.is_empty()
                        {
                                println!("There are no secret commands currently.");
                                return Ok(());
                        }

                        for (key, secret) in secrets
                        {
                                println!("  {key:<15} - {}", secret.description);
                        }
                }
                Err(_) =>
                {
                        println!("  (None found or failed to load ~/.config/.grit/.grit-secret.toml,)");
                }
        }

        Ok(())
}
