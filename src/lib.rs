pub mod config;
pub mod lex_token;
pub mod scanner;

use config::Config;
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

pub fn run_config_test_file_output(file_path: &str)-> Result<(), Box<dyn Error>> {
    run_config(PathBuf::from(file_path), true)
}

fn run(source: &str) {
    for this_token in lex(source, &mut Vec::new()) {
        println!("{}", this_token);
        println!();
    }
}

fn lex<'a>(
    source: &'a str,
    vec: &'a mut Vec<lex_token::Token<'a>>,
) -> &'a Vec<lex_token::Token<'a>> {
    let mut scanner = Scanner::new(source);

    scanner.lex_input(vec);
    vec
}

#[cfg(test)]
mod test {
    use super::lex_token::Token;
    use super::lex_token::TokenType;

    #[test]
    fn lex_basic_symbols() {
        let basic_numbers = "// this is a comment
(){}[] // grouping stuff
!*+-/=%<> >= <= == & | ^ # ? // operators
.:;, // dots and commas
&& || ^^ // logical operators";

        assert_eq!(
            super::lex(basic_numbers, &mut Vec::new()),
            &vec![
                // line 0
                Token::new(TokenType::Comment("// this is a comment"), 0, 0),
                // line 1
                Token::new(TokenType::LeftParen, 1, 0),
                Token::new(TokenType::RightParen, 1, 1),
                Token::new(TokenType::LeftBrace, 1, 2),
                Token::new(TokenType::RightBrace, 1, 3),
                Token::new(TokenType::LeftBracket, 1, 4),
                Token::new(TokenType::RightBracket, 1, 5),
                Token::new(TokenType::Comment("// grouping stuff"), 1, 7),
                // line 2
                Token::new(TokenType::Bang, 2, 0),
                Token::new(TokenType::Star, 2, 1),
                Token::new(TokenType::Plus, 2, 2),
                Token::new(TokenType::Minus, 2, 3),
                Token::new(TokenType::Slash, 2, 4),
                Token::new(TokenType::Equal, 2, 5),
                Token::new(TokenType::Mod, 2, 6),
                Token::new(TokenType::Less, 2, 7),
                Token::new(TokenType::Greater, 2, 8),
                Token::new(TokenType::GreaterEqual, 2, 10),
                Token::new(TokenType::LessEqual, 2, 13),
                Token::new(TokenType::EqualEqual, 2, 16),
                Token::new(TokenType::BinaryAnd, 2, 19),
                Token::new(TokenType::BinaryOr, 2, 21),
                Token::new(TokenType::BinaryXor, 2, 23),
                Token::new(TokenType::Hashtag, 2, 25),
                Token::new(TokenType::Hook, 2, 27),
                Token::new(TokenType::Comment("// operators"), 2, 29),
                // line 3
                Token::new(TokenType::Dot, 3, 0),
                Token::new(TokenType::Colon, 3, 1),
                Token::new(TokenType::Semicolon, 3, 2),
                Token::new(TokenType::Comma, 3, 3),
                Token::new(TokenType::Comment("// dots and commas"), 3, 5),
                // line 4
                Token::new(TokenType::LogicalAnd, 4, 0),
                Token::new(TokenType::LogicalOr, 4, 3),
                Token::new(TokenType::LogicalXor, 4, 6),
                Token::new(TokenType::Comment("// logical operators"), 4, 9),
                //EOF
                Token::new(TokenType::EOF, 4, 29)
            ]
        );
    }

    #[test]
    fn lex_strings() {
        let string_input = "\"This is a good string.\"
\"This is a bad string.
\"This is another good string!\"";

        assert_eq!(
            super::lex(string_input, &mut Vec::new()),
            &vec![
                // line 0
                Token::new(TokenType::String("\"This is a good string.\""), 0, 0),
                Token::new(TokenType::String("\"This is a bad string."), 1, 0),
                Token::new(TokenType::String("\"This is another good string!\""), 2, 0),
                Token::new(TokenType::EOF, 2, 30)
            ]
        );
    }

    #[test]
    fn lex_numbers() {
        let string_input = "314159
3.14159
314159.
.314159
4
9
0";

        assert_eq!(
            super::lex(string_input, &mut Vec::new()),
            &vec![
                Token::new(TokenType::Number("314159"), 0, 0),
                Token::new(TokenType::Number("3.14159"), 1, 0),
                Token::new(TokenType::Number("314159."), 2, 0),
                Token::new(TokenType::Number(".314159"), 3, 0),
                Token::new(TokenType::Number("4"), 4, 0),
                Token::new(TokenType::Number("9"), 5, 0),
                Token::new(TokenType::Number("0"), 6, 0),
                Token::new(TokenType::EOF, 6, 1),
            ]
        );
    }

    #[test]
    fn lex_hex() {
        let string_input = "0123456789
0x01234567
0x0A1B2C3D4E5F6
0xABCDEF
$012345
$0A1B2C3D4E5F6
$ABCDEF";

        assert_eq!(
            super::lex(string_input, &mut Vec::new()),
            &vec![
                Token::new(TokenType::Number("0123456789"), 0, 0),
                Token::new(TokenType::Number("0x01234567"), 1, 0),
                Token::new(TokenType::Number("0x0A1B2C3D4E5F6"), 2, 0),
                Token::new(TokenType::Number("0xABCDEF"), 3, 0),
                Token::new(TokenType::Number("$012345"), 4, 0),
                Token::new(TokenType::Number("$0A1B2C3D4E5F6"), 5, 0),
                Token::new(TokenType::Number("$ABCDEF"), 6, 0),
                Token::new(TokenType::EOF, 6, 7),
            ]
        );
    }

    #[test]
    fn lex_basic_identifiers() {
        let string_input = "a
Z
AbCdE
_test
_test123
test_123
testCase";

        assert_eq!(
            super::lex(string_input, &mut Vec::new()),
            &vec![
                Token::new(TokenType::Identifier("a"), 0, 0),
                Token::new(TokenType::Identifier("Z"), 1, 0),
                Token::new(TokenType::Identifier("AbCdE"), 2, 0),
                Token::new(TokenType::Identifier("_test"), 3, 0),
                Token::new(TokenType::Identifier("_test123"), 4, 0),
                Token::new(TokenType::Identifier("test_123"), 5, 0),
                Token::new(TokenType::Identifier("testCase"), 6, 0),
                Token::new(TokenType::EOF, 6, 8),
            ]
        )
    }

    #[test]
    fn lex_reserved_keywords() {
        let input_string =
            "var and or if else return for repeat while do until switch case default true false div";

        assert_eq!(
            super::lex(input_string, &mut Vec::new()),
            &vec![
                Token::new(TokenType::Var, 0, 0),
                Token::new(TokenType::AndAlias, 0, 4),
                Token::new(TokenType::OrAlias, 0, 8),
                Token::new(TokenType::If, 0, 11),
                Token::new(TokenType::Else, 0, 14),
                Token::new(TokenType::Return, 0, 19),
                Token::new(TokenType::For, 0, 26),
                Token::new(TokenType::Repeat, 0, 30),
                Token::new(TokenType::While, 0, 37),
                Token::new(TokenType::Do, 0, 43),
                Token::new(TokenType::Until, 0, 46),
                Token::new(TokenType::Switch, 0, 52),
                Token::new(TokenType::Case, 0, 59),
                Token::new(TokenType::DefaultCase, 0, 64),
                Token::new(TokenType::True, 0, 72),
                Token::new(TokenType::False, 0, 77),
                Token::new(TokenType::Div, 0, 83),
                Token::new(TokenType::EOF, 0, 86),
            ]
        )
    }

    #[test]
    fn lex_alias_words() {
        let input_string = "and not or mod";

        assert_eq!(
            super::lex(input_string, &mut Vec::new()),
            &vec![
                Token::new(TokenType::AndAlias, 0, 0),
                Token::new(TokenType::NotAlias, 0, 4),
                Token::new(TokenType::OrAlias, 0, 8),
                Token::new(TokenType::ModAlias, 0, 11),
                Token::new(TokenType::EOF, 0, 14)
            ]
        )
    }

    #[test]
    fn lex_indexers() {
        let input_string = "[ [? [# [| [@ ]";

        assert_eq!(
            super::lex(input_string, &mut Vec::new()),
            &vec![
                Token::new(TokenType::LeftBracket, 0, 0),
                Token::new(TokenType::MapIndexer, 0, 2),
                Token::new(TokenType::GridIndexer, 0, 5),
                Token::new(TokenType::ListIndexer, 0, 8),
                Token::new(TokenType::ArrayIndexer, 0, 11),
                Token::new(TokenType::RightBracket, 0, 14),
                Token::new(TokenType::EOF, 0, 15),
            ]
        )
    }

    #[test]
    fn lex_compiler_directives() {
        let input_string = "#region Region Name Long
#macro macroName 0
#endregion
#macro doing this \\
is bad";

        assert_eq!(
            super::lex(input_string, &mut Vec::new()),
            &vec![
                Token::new(TokenType::RegionBegin, 0, 0),
                Token::new(TokenType::Identifier("Region"), 0, 8),
                Token::new(TokenType::Identifier("Name"), 0, 15),
                Token::new(TokenType::Identifier("Long"), 0, 20),
                Token::new(TokenType::Macro, 1, 0),
                Token::new(TokenType::Identifier("macroName"), 1, 7),
                Token::new(TokenType::Number("0"), 1, 17),
                Token::new(TokenType::RegionEnd, 2, 0),
                Token::new(TokenType::Macro, 3, 0),
                Token::new(TokenType::Identifier("doing"), 3, 7),
                Token::new(TokenType::Identifier("this"), 3, 13),
                Token::new(TokenType::Backslash, 3, 18),
                Token::new(TokenType::Identifier("is"), 4, 0),
                Token::new(TokenType::Identifier("bad"), 4, 3),
                Token::new(TokenType::EOF, 4, 6),
            ]
        )
    }
    #[test]
    fn lex_comments() {
        let input_string = "// normal comment
var x = 20; // end comment
/* one liner */
/* multi
liner comment
*/";

        assert_eq!(
            super::lex(input_string, &mut Vec::new()),
            &vec![
                // line 0
                Token::new(TokenType::Comment("// normal comment"), 0, 0),
                Token::new(TokenType::Var, 1, 0),
                Token::new(TokenType::Identifier("x"), 1, 4),
                Token::new(TokenType::Equal, 1, 6),
                Token::new(TokenType::Number("20"), 1, 8),
                Token::new(TokenType::Semicolon, 1, 10),
                Token::new(TokenType::Comment("// end comment"), 1, 12),
                // line 1
                Token::new(TokenType::MultilineComment("/* one liner */"), 2, 0),
                Token::new(
                    TokenType::MultilineComment("/* multi\nliner comment\n*/"),
                    3,
                    0
                ),
                Token::new(TokenType::EOF, 5, 2),
            ]
        )
    }
}
