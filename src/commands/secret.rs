use serde::Deserialize;
use std::{
        collections::HashMap,
        error::Error,
        fs,
        process::{self, Command},
};

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
        if let Some(cmd_name) = name
        {
                if let Some(secret) = secrets.get(&cmd_name)
                {
                        println!("Running secret command: {}", secret.description);
                        let status = Command::new("powershell").arg("-c")
                                                               .arg(&secret.command)
                                                               .status()?;
                        process::exit(status.code().unwrap_or(1));
                }
                else
                {
                        eprintln!("Unknown secret command: {}", cmd_name);
                }
        }
        else
        {
                println!("Available secret commands:");
                for (key, secret) in secrets.iter()
                {
                        println!("  {} - {}", key, secret.description);
                }
        }

        Ok(())
}
