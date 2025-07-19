pub mod cli;
pub mod commands;
pub mod config;
pub mod shell;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{
        apps, pwd,
        secret::{self, run_secret_command},
        sysinfo,
};

use crate::commands::oxide;

/// Main entrypoint.
///
/// Parses incomming commands and arguemnts via `clap`
/// and loads the secret commands from ~/.config/.grit-secret.toml
pub fn run() -> Result<(), Box<dyn std::error::Error>>
{
        let cfg: config::Config = config::Config::new();

        let cli = Cli::parse();

        let secrets = secret::load_secret_commands()?;

        match &cli.command
        {
                Some(Commands::Pwd { platform,
                                     clip,
                                     append, }) =>
                {
                        let config = pwd::Config { platform: *platform,
                                                   should_clip: *clip };

                        let mut output: String = pwd::pwd(&config)?;

                        if *append
                        {
                                pwd::append_cd(&mut output);
                        }

                        if *clip
                        {
                                pwd::clip(&output);
                        }

                        println!("{output}");
                }

                Some(Commands::Sysinfo) => sysinfo::sysinfo()?,

                Some(Commands::Apps) => apps::list_installed_apps()?,

                Some(Commands::ListSecret) => secret::list_secrets()?,

                Some(Commands::Shells) => cfg.list_shells(),

                Some(Commands::Oxide) => oxide::render_oxide(),

                None => run_secret_command(&cfg, &secrets, cli.name, cli.args)?,
        }

        Ok(())
}
