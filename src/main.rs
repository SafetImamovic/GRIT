use std::{path::PathBuf, process};

fn main()
{
        let path: PathBuf = grit::pwd().unwrap_or_else(|err| {
                                               eprintln!("{err}");
                                               process::exit(1);
                                       });

        println!("{}", path.display());
}
