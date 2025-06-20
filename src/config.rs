use clap::ValueEnum;
use std::fmt;

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
        pub should_clip: bool,
}

impl fmt::Display for Platform
{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
        {
                match self
                {
                        Platform::Windows => write!(f, "Windows"),
                        Platform::Unix => write!(f, "Linux/Unix"),
                }
        }
}
