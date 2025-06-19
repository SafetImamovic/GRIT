use grit::*;
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

        let path = grit::pwd().expect("Failed to get current directory");

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

        let path = grit::run_inner(&Config { platform: (Platform::Unix) }).expect("Failed");

        if cfg!(windows)
        {
                assert_eq!(path, "/c/");
        }
        else
        {
                assert_eq!(path, "/");
        }
}
