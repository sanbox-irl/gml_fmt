pub mod config;
pub mod expressions;
pub mod lex_token;
pub mod parser;
pub mod printer;
pub mod scanner;
pub mod statements;

use config::Config;
use parser::Parser;
use printer::Printer;
use scanner::Scanner;
use std::{error::Error, fs, path::PathBuf};

pub fn run_config(input_path: PathBuf, do_file: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::new(input_path, do_file)?;

    for this_file in config.files {
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
    let mut tok = Vec::new();
    let mut scanner = Scanner::new(source, &mut tok);

    let our_tokens = scanner.lex_input();
    let mut parser = Parser::new(our_tokens);
    parser.build_ast();

    let mut printer = Printer::new();
    printer.autoformat(&parser.ast[..]);

    if do_print {
        println!("=========INPUT=========");
        println!("{}", source);
        println!("=========OUTPUT=========");
        println!("{}", Printer::get_output(&printer.output));
        println!("==========AST==========");
        println!("{:#?}", parser.ast);
    }
}
