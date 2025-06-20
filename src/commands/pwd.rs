use copypasta::{ClipboardContext, ClipboardProvider};

use crate::config::Config;
use std::{env, error::Error, path::Path};

/// `pwd` returns the `String` path.
pub fn pwd(config: &Config) -> Result<String, Box<dyn Error>>
{
        let path = env::current_dir()?;

        if config.platform != crate::config::Platform::Windows
        {
                let result = to_unix(&path);

                return result;
        }

        Ok(path.display().to_string())
}

pub fn append_cd(text: &mut String)
{
        text.insert_str(0, "cd ");
}

pub fn clip(text: &str)
{
        let mut ctx = ClipboardContext::new().expect("Failed to access clipboard");

        ctx.set_contents(text.to_owned())
           .expect("Failed to copy to clipboard");
}

/// `to_unix` converts a windows path to a unix path.
pub fn to_unix(path: &Path) -> Result<String, Box<dyn Error>>
{
        let mut path_str = path.display().to_string().replace("\\", "/");

        if let Some(drive) = path_str.get(0..1)
        {
                if path_str.get(1..2) == Some(":")
                {
                        path_str.replace_range(..2, &format!("/{}", drive.to_lowercase()));
                }
        }

        Ok(path_str)
}
