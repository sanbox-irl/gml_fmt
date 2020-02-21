use clap::{App, Arg};
use gml_fmt_lib::{Config, LangConfig, PrintFlags};
use std::{path::PathBuf, process};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("gml_fmt")
        .version(VERSION)
        .version_short("v")
        .author("Jonathan Spira <jjspira@gmail.com>")
        .about("Code Formatter for GML")
        .arg(Arg::with_name("file").short("f").help("Sets gml_fmt to format a file"))
        .arg(
            Arg::with_name("PATH")
                .help("Sets the path to the file or directory to use. Leave blank to use the current directory.")
                .index(1),
        )
        .arg(
            Arg::with_name("log")
                .short("l")
                .help("Prints out logging information along with formatting"),
        )
        .arg(
            Arg::with_name("log-ast")
                .short("s")
                .help("Prints out the AST to the console"),
        )
        .arg(
            Arg::with_name("no-overwrite")
                .short("n")
                .help("Do not overwrite the original file. Mostly used in conjungtion with -l to log output."),
        )
        .get_matches();

    // Get our path and make our lang_config file
    let our_path = std::env::current_dir().unwrap();
    let lang_config = LangConfig::new(&our_path);

    // Get Path
    let input_path = if matches.is_present("PATH") {
        PathBuf::from(matches.value_of("PATH").unwrap())
    } else {
        our_path
    };

    // Is it a file?
    let do_file = matches.is_present("file");

    // Do we print logs?
    let mut print_flags = PrintFlags::OVERWRITE;

    if matches.is_present("no-overwrite") {
        print_flags = PrintFlags::empty()
    }

    if matches.is_present("log") {
        print_flags.insert(PrintFlags::LOGS);
    }

    if matches.is_present("log") {
        print_flags.insert(PrintFlags::LOGS);
    }

    if matches.is_present("log-ast") {
        print_flags.insert(PrintFlags::LOG_AST);
    }

    let config = Config::new(input_path, print_flags, do_file).unwrap_or_else(|e| {
        eprintln!("File reading error: {}", e);
        process::exit(1);
    });

    match gml_fmt_lib::run_with_config(&config, &lang_config) {
        Ok(()) => {
            println!("Format complete.");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
}
