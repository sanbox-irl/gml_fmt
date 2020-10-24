#![allow(clippy::bool_comparison)]

mod config;
mod expressions;
mod lang_config;
mod lex_token;
mod parser;
mod printer;
mod scanner;
mod statements;

use anyhow::Result as AnyResult;
use parser::Parser;
use printer::Printer;
use std::fs;

pub use config::{Config, PrintFlags};
pub use lang_config::LangConfig;

pub fn run_with_config(config: &Config, lang_config: &LangConfig) -> AnyResult<()> {
    let log = config.print_flags.contains(PrintFlags::LOGS);
    let overwrite = config.print_flags.contains(PrintFlags::OVERWRITE);

    for this_file in &config.files {
        let contents = fs::read_to_string(this_file)?;

        if contents.contains("// @gml_fmt ignore") {
            continue;
        }

        if log {
            println!("=========INPUT=========");
            println!("{}", contents);
        }

        let mut ast_log = if config.print_flags.contains(PrintFlags::LOG_AST) {
            Some(String::new())
        } else {
            None
        };

        match run(&contents, lang_config, ast_log.as_mut()) {
            Ok(output) => {
                if log {
                    println!("=========OUTPUT=========");
                    println!("{}", output);
                }

                if let Some(ast) = ast_log {
                    println!("==========AST===========");
                    println!("{}", ast);
                }

                if overwrite {
                    fs::write(this_file, output)?;
                }
            }
            Err(e) => {
                println!("Could not parse file {:?}", this_file);
                println!("{}", e);
            }
        }
    }

    Ok(())
}

pub fn run(source: &str, lang_config: &LangConfig, print_ast: Option<&mut String>) -> AnyResult<String> {
    let source_size = source.len();
    match Parser::new(source).build_ast() {
        Ok(ast) => {
            if let Some(give_ast) = print_ast {
                *give_ast = format!("{:#?}", ast);
            }

            let printer = Printer::new(source_size / 2, lang_config).autoformat(&ast);

            Ok(printer.get_output(source_size))
        }

        Err(e) => {
            anyhow::bail!("{}", e);
        }
    }
}

pub fn run_snippet(source: &str, lang_config: Option<LangConfig>) -> AnyResult<String> {
    let source_size = source.len();
    let ast = Parser::new(source).build_ast()?;
    let config = lang_config.unwrap_or_default();
    let printer = Printer::new(source_size / 2, &config).autoformat(&ast);

    Ok(printer.get_output(source_size))
}
