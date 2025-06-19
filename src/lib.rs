use std::{env::current_dir, error::Error, path::PathBuf};

pub fn pwd() -> Result<PathBuf, Box<dyn Error>>
{
        let binding = current_dir()?;

        Ok(binding)
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

                assert_eq!(path.display().to_string(), "C:\\");
        }
}
