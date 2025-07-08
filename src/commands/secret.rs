use clap::CommandFactory;
use dirs_next::home_dir;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use crate::{cli::Cli, commands::secret};

#[derive(Debug, Deserialize)]
pub struct SecretCommand
{
        pub description: String,
        pub command: String,
}

pub fn load_secret_commands() -> Result<HashMap<String, SecretCommand>, Box<dyn Error>>
{
        // Resolve the user's home directory
        let home = home_dir().ok_or("Could not find home directory")?;

        // Construct the path to ~/.config/.grit-secret.toml
        let secret_path: PathBuf = home.join(".config").join(".grit-secret.toml");

        // Read and parse the TOML file
        let toml_str = fs::read_to_string(&secret_path).map_err(|e| {
                               format!("Failed to read secret file at {:?}: {}", secret_path, e)
                       })?;

        let map: HashMap<String, SecretCommand> =
                toml::from_str(&toml_str).map_err(|e| format!("Failed to parse TOML: {}", e))?;

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
                        let status = std::process::Command::new("bash").arg("-c")
                                                                       .arg(&secret.command)
                                                                       .args(_args)
                                                                       .status()?;
                        std::process::exit(status.code().unwrap_or(1));
                }
                else
                {
                        eprintln!("Unknown secret command: {}", secret_name);

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
        println!("Secret commands (from ~/.config/.grit-secret.toml):\n");

        match load_secret_commands()
        {
                Ok(secrets) =>
                {
                        if secrets.len() == 0
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
                        println!("  (None found or failed to load ~/.config/.grit-secret.toml,)");
                }
        }

        Ok(())
}
