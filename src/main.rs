use std::{
        env::{self},
        path::PathBuf,
        process,
};

use grit::{Config, Platform};

fn main()
{
        let args: Vec<String> = env::args().collect();

        let config: Config = if args.contains(&"unix".to_string())
        {
                Config::build(Platform::Unix)
        }
        else
        {
                Config::build(Platform::Windows)
        };

        println!("{}", config.platform);

        let path: PathBuf = grit::pwd().unwrap_or_else(|err| {
                                               eprintln!("{err}");
                                               process::exit(1);
                                       });

        println!("{}", path.display());
}
