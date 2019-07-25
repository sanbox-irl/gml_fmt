use gml_fmt::config::config::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments...{}", err);
        process::exit(1);
    });

    gml_fmt::run_config(config).unwrap_or_else(|err| {
        eprintln!("Application error...{}", err);
    });
}
