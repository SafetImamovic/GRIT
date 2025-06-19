use std::{env::current_dir, error::Error, fmt::Display, path::PathBuf};

pub fn run(config: &Config) -> Result<String, Box<dyn Error>>
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

#[derive(PartialEq)]
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

                let path = run(&Config { platform: (Platform::Unix) }).expect("Failed");

                assert_eq!(path, "/c/");
        }
}
