use crate::config::Platform;
use clap::{Parser, Subcommand};

/// `Cli` derives from `Parser` which uses the `parse()` method
/// to gather commands and arguemnts passed via CLI.
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
        pub command: Option<Commands>,

        /// Execute a secret command defined in ~/.config/.grit-secret.toml
        pub name: Option<String>,

        /// Pass additional arguments to the secret command
        pub args: Vec<String>,
}

/// There are 2 main categories of commands:
///     - Predefined commands
///     - Secret commands
///
/// Predefined commands are defined in code while
/// secret commands are defined in the ~/.config/.grit-secret.toml
#[derive(Subcommand, Debug)]
pub enum Commands
{
        #[command(about = "Print the present working directory")]
        Pwd
        {
                #[arg(short, long, value_enum, default_value = "windows")]
                platform: Platform,
        },

        #[command(about = "Detailed info about the system")]
        Sysinfo,

        #[command(about = "List all installed applications")]
        Apps,

        #[command(about = "List all hidden commands")]
        ListSecret,
}
