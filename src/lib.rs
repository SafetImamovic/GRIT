pub mod cli;
pub mod commands;
pub mod config;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{apps, pwd, secret, sysinfo};
use config::Config;

/// Main entrypoint.
///
/// Parses incomming commands and arguemnts via `clap`
/// and loads the secret commands from .secret.toml
pub fn run() -> Result<(), Box<dyn std::error::Error>>
{
        let cli = Cli::parse();
        let secrets = secret::load_secret_commands()?;

        match cli.command
        {
                Commands::Pwd { platform } =>
                {
                        let config = Config { platform };
                        let output = pwd::pwd(&config)?;
                        println!("{output}");
                }
                Commands::Secret { name, args } =>
                {
                        secret::run_secret_command(&secrets, name, args)?
                }
                Commands::Sysinfo => sysinfo::sysinfo()?,
                Commands::Apps => apps::list_installed_apps()?,
        }

        Ok(())
}
