use std::collections::HashMap;
use std::fs;
use std::{env::current_dir, error::Error, fmt::Display, path::PathBuf};

use clap::ValueEnum;

use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(name = "grit")]
#[command(
          version = "0.1",
          about = r#"   

  .g8"""bgd `7MM"""Mq.  `7MMF'MMP""MM""YMM 
.dP'     `M   MM   `MM.   MM  P'   MM   `7 
dM'       `   MM   ,M9    MM       MM      
MM            MMmmdM9     MM       MM      
MM.    `7MMF' MM  YM.     MM       MM      
`Mb.     MM   MM   `Mb.   MM       MM      
  `"bmmmdPY .JMML. .JMM..JMML.   .JMML.    
                                           
  General      Rust    Interface  Tool

multi-purpose CLI utility written in Rust
(ASCII art generated @ https://www.patorjk.com/software/taag/
[font: Georgia11])
"#
)]
pub struct Cli
{
        #[command(subcommand)]
        command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands
{
        Pwd
        {
                #[arg(short, long, value_enum, default_value = "windows")]
                platform: Platform,
        },

        /// Run a secret command defined in the config file
        Secret
        {
                name: Option<String>,
                args: Vec<String>,
        },
}

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

pub fn run() -> Result<(), Box<dyn Error>>
{
        let cli = Cli::parse();

        match cli.command
        {
                Commands::Pwd { platform } =>
                {
                        let config = Config { platform };
                        let output = run_inner(&config)?;
                        println!("{output}");
                }

                Commands::Secret { name, args: _ } =>
                {
                        let secrets = load_secret_commands()?;

                        if let Some(cmd_name) = name
                        {
                                if let Some(secret) = secrets.get(&cmd_name)
                                {
                                        println!("Running secret command: {}", secret.description);

                                        // If you want, you can pass `args` into the command string with templating
                                        let command_to_run = &secret.command;

                                        let status =
                                        std::process::Command::new("powershell").arg("-c")
                                                                                .arg(command_to_run)
                                                                                .status()?;

                                        std::process::exit(status.code().unwrap_or(1));
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
                }
        }

        Ok(())
}

fn run_inner(config: &Config) -> Result<String, Box<dyn Error>>
{
        let path = pwd();

        if config.platform == Platform::Windows
        {
                return Ok(path?.display().to_string());
        }

        to_unix(path?)
}

pub fn pwd() -> Result<PathBuf, Box<dyn Error>>
{
        let binding = current_dir()?;

        Ok(binding)
}

pub fn to_unix(path: PathBuf) -> Result<String, Box<dyn Error>>
{
        let path_str: String = path.display().to_string();

        let mut path_str: String = path_str.replace("\\", "/");

        if let Some(c) = path_str.get(0..1)
        {
                if path_str.get(1..2) == Some(":")
                {
                        path_str.replace_range(..2, &format!("/{}", c.to_lowercase()));
                }
        }

        Ok(path_str)
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
#[clap(rename_all = "lower")]
pub enum Platform
{
        Windows,
        Unix,
}

pub struct Config
{
        pub platform: Platform,
}

impl Config
{
        pub fn build(platform: Platform) -> Config
        {
                Config { platform }
        }
}

impl Display for Platform
{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
                match self
                {
                        Platform::Windows => write!(f, "Windows"),
                        Platform::Unix => write!(f, "Linux/Unix"),
                }
        }
}

#[cfg(test)]
mod tests
{
        use super::*;
        use std::{env, path::Path};

        #[test]
        fn test_pwd_root_dir()
        {
                let root = if cfg!(windows)
                {
                        Path::new("C:\\")
                }
                else
                {
                        Path::new("/")
                };

                env::set_current_dir(root).expect("Failed to set current directory");

                let path = pwd().expect("Failed to get current directory");

                assert_eq!(path, root);
        }

        #[test]
        fn test_to_unix_conversion()
        {
                let root = if cfg!(windows)
                {
                        Path::new("C:\\")
                }
                else
                {
                        Path::new("/")
                };

                env::set_current_dir(root).expect("Failed to set current directory");

                let path = run_inner(&Config { platform: (Platform::Unix) }).expect("Failed");

                if cfg!(windows)
                {
                        assert_eq!(path, "/c/");
                }
                else
                {
                        assert_eq!(path, "/");
                }
        }
}
