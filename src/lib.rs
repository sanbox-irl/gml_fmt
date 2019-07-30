pub mod config;
pub mod expressions;
pub mod lex_token;
pub mod parser;
pub mod scanner;

use config::Config;
use parser::Parser;
use scanner::Scanner;
use std::{error::Error, fs, path::PathBuf};

pub fn run_config(input_path: PathBuf, do_file: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::new(input_path, do_file)?;

    for this_file in config.files {
        println!("========== LEX READOUT OF {:?} ==========", this_file);
        let contents = fs::read_to_string(this_file)?;

        run(&contents, true);
    }

    Ok(())
}

pub fn run_config_test_file_no_output(file_path: &str) -> Result<(), Box<dyn Error>> {
    let config = Config::new(PathBuf::from(file_path), true)?;

    for this_file in config.files {
        let contents = fs::read_to_string(this_file)?;

        run(&contents, false);
    }

    Ok(())
}

pub fn run_config_test_file_output(file_path: &str) -> Result<(), Box<dyn Error>> {
    run_config(PathBuf::from(file_path), true)
}

fn run(source: &str, do_print: bool) {
    let mut our_tokens = Vec::new();
    let mut scanner = Scanner::new(source, &mut our_tokens);
    scanner.lex_input();

    let filled_tokens = scanner.tokens;

    let mut parser = Parser::new(filled_tokens);
    // parser.build_ast(vec).into_iter().collect()

    // println!("{:?}", );
}

