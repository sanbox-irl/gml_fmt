extern crate clap;

use clap::{App, Arg};
use std::{path::PathBuf, process};

fn main() {
    let matches = App::new("gml_fmt")
        .version("0.1.0")
        .author("Jonathan Spira <jjspira@gmail.com>")
        .about("Code Formatter for GML")
        .arg(Arg::with_name("file").short("f").long("file").help("Sets gml_fmt to format a file"))
        .arg(
            Arg::with_name("PATH")
                .help("Sets the path to the file or directory to use. Leave blank to use the current directory.")
                .index(1)
        )
        .get_matches();

    let input_path = if matches.is_present("PATH") {
        PathBuf::from(matches.value_of("PATH").unwrap())
    } else {
        std::env::current_dir().unwrap()
    };

    let do_file = matches.is_present("file");

    match gml_fmt::run_config(input_path, do_file) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
}
