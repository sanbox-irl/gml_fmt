extern crate clap;

use clap::{App, Arg};
use gml_fmt::config::config::Config;
use std::{path::PathBuf, process};

fn main() {
    let matches = App::new("gml_fmt")
        .version("0.1.0")
        .author("Jonathan Spira <jjspira@gmail.com>")
        .about("Code Formatter for GML")
        .arg(Arg::with_name("file").short("f").long("file"))
        .arg(
            Arg::with_name("PATH")
                .help("Sets the path to the file or directory to use. Defaults to directory.")
                .index(1)
                .required(true),
        )
        .get_matches();

    let input_arg = matches.value_of("PATH").unwrap();
    let input_path = PathBuf::from(input_arg);
    let do_file = matches.is_present("file");

    let config = Config::new(input_path, do_file).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    gml_fmt::run_config(config).unwrap_or_else(|err| {
        eprintln!("Error in Application {}", err);
        process::exit(65);
    });
}
