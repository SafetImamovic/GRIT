use std::error::Error;
use std::io::{self, Write};

use crossterm::terminal;

use crate::cli::Cli;
use crate::config::Config;

pub fn clear(config: &Config, cli: &Cli) -> Result<(), Box<dyn Error>>
{
        let lines = cli.args
                       .get(1)
                       .and_then(|arg| arg.parse::<u16>().ok())
                       .or_else(|| terminal::size().ok().map(|(_, height)| height))
                       .unwrap_or(config.clear_default_lines);

        let mut stdout = io::stdout();

        for _ in 0..lines
        {
                writeln!(stdout)?;
        }

        // Move cursor to top-left of the visible screen
        write!(stdout, "\x1B[H")?;
        stdout.flush()?;

        Ok(())
}
