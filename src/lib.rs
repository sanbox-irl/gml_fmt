mod lexer;
use lexer::scanner::Scanner;
use lexer::error_tokens::LexError;
use lexer::lex_token;
use lexer::error_tokens::Error;

pub fn run(source: &str) {
    match lex(source) {
        Ok(tokens) => {
            for this_token in tokens {
                println!("{:?}", this_token);
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    };
}

pub fn lex(source: &str) -> Result<Vec<lex_token::Token>, Error<LexError>> {
    let scanner = Scanner::new(&source[..]);

    scanner.lex_input()
}

#[cfg(test)]
mod test {
    #[test]
    fn basic_numbers() {
        let basic_numbers = "\
                    x = 5;
                    x = 5.;
                    x = .5;
                    x = -5;
                    x = -5.5;
                    x = -.5;";

        assert_eq!(super::lex(basic_numbers), Ok(vec![]));
    }
}
