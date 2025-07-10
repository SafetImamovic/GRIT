use std::env;

pub struct Config
{
        shells: Vec<String>, // Made private to enforce controlled access
}

impl Config
{
        pub fn new() -> Self
        {
                let shell = Self::get_default_shell();
                Self { shells: vec![shell] }
        }

        fn get_default_shell() -> String
        {
                if cfg!(windows)
                {
                        env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".into())
                }
                else
                {
                        env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into())
                }
        }

        pub fn default_shell(&self) -> &str
        {
                // Prefer returning data over printing in library code
                self.shells.first().expect("Always has at least one shell")
        }

        pub fn all_shells(&self) -> &[String]
        {
                &self.shells
        }

        pub fn list_shells(&self)
        {
                for i in self.all_shells()
                {
                        println!("{}", i);
                }
        }
}
