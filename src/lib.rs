pub mod cli;
pub mod commands;
pub mod config;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{
        apps, pwd,
        secret::{self, run_secret_command},
        sysinfo,
};
use config::Config;

/// Main entrypoint.
///
/// Parses incomming commands and arguemnts via `clap`
/// and loads the secret commands from .secret.toml
pub fn run() -> Result<(), Box<dyn std::error::Error>>
{
        let cli = Cli::parse();

        let secrets = secret::load_secret_commands()?;

        if cli.list_secrets
        {
                secret::show_help_with_secrets();

                std::process::exit(0);
        }

        match &cli.command
        {
                Some(Commands::Pwd { platform }) =>
                {
                        let config = Config { platform: *platform };
                        let output = pwd::pwd(&config)?;
                        println!("{output}");
                }

                Some(Commands::Sysinfo) => sysinfo::sysinfo()?,

                Some(Commands::Apps) => apps::list_installed_apps()?,

                None => run_secret_command(&secrets, cli.name, cli.args)?,
        }

        Ok(())
}
