use std::{
        env::{self},
        process,
};

use grit::{run, Config, Platform};

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

        let path = run(&config).unwrap_or_else(|err| {
                                       eprintln!("{err}");
                                       process::exit(1)
                               });

        println!("{}", path);
}
