extern crate clap;

use clap::{App, Arg};
use gml_fmt::config::Config;
use gml_fmt::config::PrintFlags;
use std::{path::PathBuf, process};

fn main() {
    let matches = App::new("gml_fmt")
        .version("0.1.0")
        .author("Jonathan Spira <jjspira@gmail.com>")
        .about("Code Formatter for GML")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("Sets gml_fmt to format a file")
        )
        .arg(
            Arg::with_name("PATH")
                .help("Sets the path to the file or directory to use. Leave blank to use the current directory.")
                .index(1)
        )
        .arg(
            Arg::with_name("logs")
                .short("l")
                .long("logs")
                .help("Prints out logging information along with formatting")
        )
        .get_matches();

    // Get Path
    let input_path = if matches.is_present("PATH") {
        PathBuf::from(matches.value_of("PATH").unwrap())
    } else {
        std::env::current_dir().unwrap()
    };

    // Is it a file?
    let do_file = matches.is_present("file");

    // Do we print logs?
    let mut print_flags = PrintFlags::OVERWRITE;
    if matches.is_present("logs") {
        print_flags |= PrintFlags::LOGS;
    }

    let config = Config::new(input_path, print_flags, do_file).unwrap_or_else(|e| {
        eprintln!("File reading error: {}", e);
        process::exit(1);
    });

    match gml_fmt::run_config(&config) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
}
