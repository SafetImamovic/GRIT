use grit::commands::pwd::{Config, Platform, pwd};
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

        let path = pwd(&Config { platform: (Platform::Windows),
                                 should_clip: false }).expect("Failed to get current directory");

        if cfg!(windows)
        {
                assert_eq!(path, "\"C:\\\"");
        }
        else
        {
                assert_eq!(path, "\"/c/\"");
        }
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

        let path = pwd(&Config { platform: (Platform::Unix),
                                 should_clip: false }).expect("Failed");

        if cfg!(windows)
        {
                assert_eq!(path, "\"/c/\"");
        }
        else
        {
                assert_eq!(path, "\"/\"");
        }
}
