pub mod config;
pub mod lex_token;
pub mod printer;
pub mod scanner;

use config::Config;
use printer::Printer;
use scanner::Scanner;
use std::{error::Error, fs, path::PathBuf};

pub fn run_config(input_path: PathBuf, do_file: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::new(input_path, do_file)?;

    for this_file in config.files {
        println!("========== LEX READOUT OF {:?} ==========", this_file);
        let contents = fs::read_to_string(this_file)?;

        run(&contents);
    }

    Ok(())
}

pub fn run_config_test_file_no_output(file_path: &str) -> Result<(), Box<dyn Error>> {
    let config = Config::new(PathBuf::from(file_path), true)?;

    for this_file in config.files {
        let contents = fs::read_to_string(this_file)?;

        lex(&contents, &mut Vec::new());
    }

    Ok(())
}

pub fn run_config_test_file_output(file_path: &str) -> Result<(), Box<dyn Error>> {
    run_config(PathBuf::from(file_path), true)
}

fn run(source: &str) {
    let mut empty_tokens = Vec::new();
    let filled_tokens = lex(source, &mut empty_tokens);

    let output = print(&filled_tokens);
    println!("{}", output);
}

fn lex<'a>(
    source: &'a str,
    vec: &'a mut Vec<lex_token::Token<'a>>,
) -> &'a Vec<lex_token::Token<'a>> {
    let mut scanner = Scanner::new(source);

    scanner.lex_input(vec);
    vec
}

// @performance we're cloning a string here. That's uggo!
// Probably let's switch to something more robust
fn print<'a>(vec: &'a Vec<lex_token::Token<'a>>) -> String {
    let mut printer = Printer::new();
    printer.autoformat(vec).to_string()
}