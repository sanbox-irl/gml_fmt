pub mod config;
pub mod expressions;
pub mod lex_token;
pub mod parser;
pub mod printer;
pub mod scanner;
pub mod statements;

use config::{Config, PrintFlags};
use parser::Parser;
use printer::Printer;
use scanner::Scanner;
use std::{error::Error, fs};

pub fn run_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let log = config.print_flags.contains(PrintFlags::LOGS);
    let overwrite = config.print_flags.contains(PrintFlags::OVERWRITE);

    for this_file in &config.files {
        let contents = fs::read_to_string(this_file)?;

        if log {
            println!("=========INPUT=========");
            println!("{}", contents);
        }

        let res = run(&contents, log);
        if log {
            println!("=========OUTPUT=========");
            println!("{}", res.0);
            println!("==========AST===========");
            println!("{}", res.1.unwrap());
        }

        if overwrite {
            fs::write(this_file, res.0)?;
        }
    }

    Ok(())
}

fn run(source: &str, print_ast: bool) -> (String, Option<String>) {
    let mut tok = Vec::new();
    let mut scanner = Scanner::new(source, &mut tok);

    let our_tokens = scanner.lex_input();
    let mut parser = Parser::new(our_tokens);
    parser.build_ast();

    let mut printer = Printer::new();
    printer.autoformat(&parser.ast[..]);
    (
        Printer::get_output(&printer.output),
        if print_ast {
            Some(format!("{:#?}", parser.ast))
        } else {
            None
        },
    )
}
