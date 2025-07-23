use std::env;

pub struct Config
{
        pub shells: Vec<String>,
}

impl Default for Config
{
        fn default() -> Self
        {
                Self::new()
        }
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
                        //env::var("COMSPEC").unwrap_or_else(|_| "powershell".into())
                        "powershell".to_string()
                }
                else
                {
                        env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into())
                }
        }

        pub fn default_shell(&self) -> &str
        {
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
                        println!("{i}");
                }
        }
}
