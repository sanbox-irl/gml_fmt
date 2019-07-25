use std::env;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(this_arg) => this_arg,
            None => return Err("No filename given."),
        };

        Ok(Config { filename })
    }
}