mod lexer;
use lexer::error_tokens::Error;
use lexer::error_tokens::LexError;
use lexer::lex_token;
use lexer::scanner::Scanner;

pub fn run(source: &str) {
    match lex(source, &mut Vec::new()) {
        Ok(tokens) => {
            for this_token in tokens {
                println!("{}", this_token);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    };
}

pub fn lex<'a>(
    source: &'a str,
    vec: &'a mut Vec<lex_token::Token<'a>>,
) -> Result<&'a Vec<lex_token::Token<'a>>, Error<LexError>> {
    let mut scanner = Scanner::new(source);

    scanner.lex_input(vec)
}

#[cfg(test)]
mod test {
    use super::lex_token::Token;
    use super::lex_token::TokenType;

    #[test]
    fn lex_basic_symbols() {
        let basic_numbers = "// this is a comment
(( )){} // grouping stuff
!*+-/=<> >= <= == // operators";

        assert_eq!(
            super::lex(basic_numbers, &mut Vec::new()).expect("Did not succesfully lex..."),
            &vec![
                // line 0
                Token::new(TokenType::Comment("// this is a comment"), 0, 0),
                // line 1
                Token::new(TokenType::LeftParen, 1, 0),
                Token::new(TokenType::LeftParen, 1, 1),
                Token::new(TokenType::RightParen, 1, 3),
                Token::new(TokenType::RightParen, 1, 4),
                Token::new(TokenType::LeftBrace, 1, 5),
                Token::new(TokenType::RightBrace, 1, 6),
                Token::new(TokenType::Comment("// grouping stuff"), 1, 8),
                // line 2
                Token::new(TokenType::Bang, 2, 0),
                Token::new(TokenType::Star, 2, 1),
                Token::new(TokenType::Plus, 2, 2),
                Token::new(TokenType::Minus, 2, 3),
                Token::new(TokenType::Slash, 2, 4),
                Token::new(TokenType::Equal, 2, 5),
                Token::new(TokenType::Less, 2, 6),
                Token::new(TokenType::Greater, 2, 7),
                Token::new(TokenType::GreaterEqual, 2, 9),
                Token::new(TokenType::LessEqual, 2, 12),
                Token::new(TokenType::EqualEqual, 2, 15),
                Token::new(TokenType::Comment("// operators"), 2, 18),
                Token::new(TokenType::EOF, 2, 30)
            ]
        );
    }

    #[test]
    fn lex_strings() {
        let string_input = "\"This is a good string.\"
\"This is a bad string.
\"This is another good string!\"";

        assert_eq!(
            super::lex(string_input, &mut Vec::new()).expect("Did not succesfully lex..."),
            &vec![
                // line 0
                Token::new(TokenType::String("\"This is a good string.\""), 0, 0),
                Token::new(TokenType::String("\"This is a bad string."), 1, 0),
                Token::new(TokenType::String("\"This is another good string!\""), 2, 0),
                Token::new(TokenType::EOF, 2, 30)
            ]
        );
    }
}
