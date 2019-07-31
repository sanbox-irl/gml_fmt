use super::lex_token::*;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    pub tokens: &'a mut Vec<Token<'a>>,
    input: Chars<'a>,
    line_number: u32,
    column_number: u32,
    iter: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str, tokens: &'a mut Vec<Token<'a>>) -> Scanner<'a> {
        Scanner {
            input: input.chars(),
            line_number: 0,
            column_number: 0,
            iter: input.chars().enumerate().peekable(),
            tokens,
        }
    }

    pub fn lex_input(&mut self) -> &mut std::vec::Vec<Token<'a>> {
        while let Some((i, c)) = self.iter.next() {
            match c {
                // Single Char
                '(' => self.add_simple_token(TokenType::LeftParen),
                ')' => self.add_simple_token(TokenType::RightParen),
                '{' => self.add_simple_token(TokenType::LeftBrace),
                '}' => self.add_simple_token(TokenType::RightBrace),
                ',' => self.add_simple_token(TokenType::Comma),
                '-' => {
                    if self.peek_and_check_consume('-') {
                        self.add_simple_token(TokenType::Decrementer);
                    } else {
                        self.add_simple_token(TokenType::Minus);
                    }
                }
                '+' => {
                    if self.peek_and_check_consume('+') {
                        self.add_simple_token(TokenType::Incrementer);
                    } else {
                        self.add_simple_token(TokenType::Plus);
                    }
                }
                ';' => self.add_simple_token(TokenType::Semicolon),
                '*' => self.add_simple_token(TokenType::Star),
                ':' => self.add_simple_token(TokenType::Colon),
                '%' => self.add_simple_token(TokenType::Mod),
                ']' => self.add_simple_token(TokenType::RightBracket),
                '?' => self.add_simple_token(TokenType::Hook),
                '\\' => self.add_simple_token(TokenType::Backslash),

                // Branching multichar symbols
                '!' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::BangEqual, 2);
                    } else {
                        self.add_simple_token(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::EqualEqual, 2);
                    } else {
                        self.add_simple_token(TokenType::Equal)
                    }
                }
                '<' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::LessEqual, 2);
                    } else if self.peek_and_check_consume('<') {
                        self.add_multiple_token(TokenType::BitLeft, 2);
                    } else {
                        self.add_simple_token(TokenType::Less)
                    }
                }
                '>' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::GreaterEqual, 2);
                    } else if self.peek_and_check_consume('>') {
                        self.add_multiple_token(TokenType::BitRight, 2);
                    } else {
                        self.add_simple_token(TokenType::Greater)
                    }
                }
                '&' => {
                    if self.peek_and_check_consume('&') {
                        self.add_multiple_token(TokenType::LogicalAnd, 2);
                    } else {
                        self.add_simple_token(TokenType::BitAnd);
                    }
                }

                '|' => {
                    if self.peek_and_check_consume('|') {
                        self.add_multiple_token(TokenType::LogicalOr, 2);
                    } else {
                        self.add_simple_token(TokenType::BitOr);
                    }
                }

                '^' => {
                    if self.peek_and_check_consume('^') {
                        self.add_multiple_token(TokenType::LogicalXor, 2);
                    } else {
                        self.add_simple_token(TokenType::BitXor);
                    }
                }

                '[' => match self.iter.peek() {
                    Some((_i, next_char)) if *next_char == '@' => {
                        self.add_multiple_token(TokenType::ArrayIndexer, 2);
                        self.iter.next();
                    }

                    Some((_i, next_char)) if *next_char == '?' => {
                        self.add_multiple_token(TokenType::MapIndexer, 2);
                        self.iter.next();
                    }

                    Some((_i, next_char)) if *next_char == '|' => {
                        self.add_multiple_token(TokenType::ListIndexer, 2);
                        self.iter.next();
                    }

                    Some((_i, next_char)) if *next_char == '#' => {
                        self.add_multiple_token(TokenType::GridIndexer, 2);
                        self.iter.next();
                    }

                    _ => self.add_simple_token(TokenType::LeftBracket),
                },

                // Compiler Directives
                '#' => {
                    let start = i;
                    let mut current = start;

                    if let None = self.peek_and_check_while(|i, this_char| {
                        current = i;
                        this_char.is_ascii_alphanumeric() || this_char == '_'
                    }) {
                        current += 1;
                    };

                    let token_returned = self.check_for_macro_directive(start, current);

                    match token_returned {
                        Some(macro_directive) => {
                            self.add_multiple_token(macro_directive, (current - start) as u32)
                        }

                        None => {
                            // we're adding a hashtag token, which doesn't really mean anything,
                            // but just want to keep the sizes right.
                            self.add_simple_token(TokenType::Hashtag);

                            // for a weird # floating in space
                            if current - start - 1 != 0 {
                                self.add_multiple_token(
                                    TokenType::Identifier(&self.input[start..current]),
                                    (current - start - 1) as u32,
                                );
                            }
                        }
                    }
                }

                // string literals
                '"' => {
                    let start = i;
                    let mut current = start;

                    if let Some((i, break_char)) = self.peek_and_check_while(|i, string_char| {
                        current = i;
                        string_char != '\n' && string_char != '"'
                    }) {
                        // eat the quote
                        if break_char == '"' {
                            self.iter.next();
                            current = i + 1;
                        }
                    }

                    self.add_multiple_token(
                        TokenType::String(&self.input[start..current]),
                        (current - start) as u32,
                    );
                }

                // Number literals
                '.' => {
                    match self.iter.peek() {
                        Some((_, next_char)) if next_char.is_digit(10) => {
                            let start = i;
                            let mut current = start;

                            // eat the "."
                            self.iter.next();

                            while let Some((i, number_char)) = self.iter.peek() {
                                if number_char.is_digit(10) {
                                    current = *i + 1;
                                    self.iter.next();
                                } else {
                                    break;
                                }
                            }

                            self.add_multiple_token(
                                TokenType::Number(&self.input[start..current]),
                                (current - start) as u32,
                            );
                        }
                        _ => self.add_simple_token(TokenType::Dot),
                    }
                }

                '0'..='9' => {
                    let start = i;
                    let mut current = start + 1;

                    // Check for Hex
                    if c == '0' {
                        if let Some((_, number_char)) = self.iter.peek() {
                            if *number_char == 'x' {
                                self.iter.next();

                                while let Some((i, number_char)) = self.iter.peek() {
                                    if number_char.is_digit(16) {
                                        current = *i + 1;
                                        self.iter.next();
                                    } else {
                                        break;
                                    }
                                }

                                self.add_multiple_token(
                                    TokenType::Number(&self.input[start..current]),
                                    (current - start) as u32,
                                );
                                continue;
                            }
                        }
                    }

                    let mut is_fractional = false;

                    while let Some((i, number_char)) = self.iter.peek() {
                        if number_char.is_digit(10) {
                            current = *i + 1;
                            self.iter.next();
                        } else {
                            is_fractional = *number_char == '.';
                            break;
                        }
                    }

                    if is_fractional {
                        // eat the "."
                        current = self.iter.next().unwrap().0 + 1;

                        while let Some((i, number_char)) = self.iter.peek() {
                            if number_char.is_digit(10) {
                                current = *i + 1;
                                self.iter.next();
                            } else {
                                break;
                            }
                        }
                    }

                    self.add_multiple_token(
                        TokenType::Number(&self.input[start..current]),
                        (current - start) as u32,
                    )
                }

                // Secondary Hex
                '$' => {
                    let start = i;
                    let mut current = start;

                    if let None = self.peek_and_check_while(|i, hex_char| {
                        current = i;
                        hex_char.is_digit(16)
                    }) {
                        current += 1;
                    }

                    self.add_multiple_token(
                        TokenType::Number(&self.input[start..current]),
                        (current - start) as u32,
                    );
                }

                // Comments
                '/' => {
                    // Normal Comment
                    if self.peek_and_check_consume('/') {
                        let start = i;
                        let mut current = start;

                        if let None = self.peek_and_check_while(|i, this_char| {
                            current = i;
                            this_char != '\n'
                        }) {
                            current += 1;
                        }

                        self.add_multiple_token(
                            TokenType::Comment(&self.input[start..current]),
                            (current - start) as u32,
                        );
                    } else if self.peek_and_check_consume('*') {
                        // Multiline Comment
                        let start = i;
                        let start_line = self.line_number;
                        let start_column = self.column_number;

                        let mut last_column_break = start;
                        let mut current = start;

                        while let Some((i, comment_char)) = self.iter.peek() {
                            current = *i;

                            match comment_char {
                                &'*' => {
                                    current = self.iter.next().unwrap().0 + 1;
                                    if let Some((_, next_next_char)) = self.iter.peek() {
                                        if next_next_char == &'/' {
                                            current = self.iter.next().unwrap().0 + 1;
                                            break;
                                        }
                                    }
                                }

                                &'\n' => {
                                    self.next_line();
                                    last_column_break = current + 1;
                                }

                                _ => {}
                            };

                            self.iter.next();
                        }

                        self.tokens.push(Token::new(
                            TokenType::MultilineComment(&self.input[start..current]),
                            start_line,
                            start_column,
                        ));
                        self.column_number += (current - last_column_break) as u32;
                    } else {
                        self.add_simple_token(TokenType::Slash);
                    }
                }
                ' ' | '\t' => self.column_number += 1,

                '\n' => {
                    self.add_simple_token(TokenType::Newline);
                    self.next_line();
                }
                '\r' => continue,

                'A'..='Z' | 'a'..='z' | '_' => {
                    let start = i;
                    let mut current = start + 1;

                    if let None = self.peek_and_check_while(|i, this_char| {
                        current = i;
                        this_char.is_ascii_alphanumeric() || this_char == '_'
                    }) {
                        current += 1;
                    };

                    let keyword_token_type: Option<TokenType> =
                        self.check_for_keyword(start, current);

                    match keyword_token_type {
                        Some(token_type) => {
                            self.add_multiple_token(token_type, (current - start) as u32)
                        }
                        None => self.add_multiple_token(
                            TokenType::Identifier(&self.input[start..current]),
                            (current - start) as u32,
                        ),
                    }
                }

                _ => {
                    println!("Unexpected character {}", c);
                    self.column_number += 1;
                }
            };
        }

        self.add_simple_token(TokenType::EOF);
        self.tokens
    }

    fn add_simple_token(&mut self, token_type: TokenType<'a>) {
        self.add_multiple_token(token_type, 1);
    }

    fn add_multiple_token(&mut self, token_type: TokenType<'a>, size: u32) {
        self.tokens
            .push(Token::new(token_type, self.line_number, self.column_number));
        self.column_number += size;
    }

    fn peek_and_check_consume(&mut self, char_to_check: char) -> bool {
        if let Some((_i, next_char)) = self.iter.peek() {
            let ret = next_char == &char_to_check;
            if ret {
                self.iter.next();
            }
            ret
        } else {
            false
        }
    }

    fn peek_and_check_while<F>(&mut self, mut f: F) -> Option<(usize, char)>
    where
        F: FnMut(usize, char) -> bool,
    {
        while let Some((i, next_char)) = self.iter.peek() {
            if f(*i, *next_char) == false {
                return Some((*i, *next_char));
            };
            self.iter.next();
        }
        None
    }

    fn next_line(&mut self) {
        self.line_number += 1;
        self.column_number = 0;
    }

    fn check_for_keyword(&self, start: usize, current: usize) -> Option<TokenType<'a>> {
        match &self.input[start..current] {
            "var" => Some(TokenType::Var),
            "and" => Some(TokenType::AndAlias),
            "or" => Some(TokenType::OrAlias),
            "not" => Some(TokenType::NotAlias),
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "return" => Some(TokenType::Return),
            "for" => Some(TokenType::For),
            "repeat" => Some(TokenType::Repeat),
            "while" => Some(TokenType::While),
            "do" => Some(TokenType::Do),
            "until" => Some(TokenType::Until),
            "switch" => Some(TokenType::Switch),
            "case" => Some(TokenType::Case),
            "default" => Some(TokenType::DefaultCase),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "mod" => Some(TokenType::ModAlias),
            "div" => Some(TokenType::Div),
            "break" => Some(TokenType::Break),
            "exit" => Some(TokenType::Exit),
            "enum" => Some(TokenType::Enum),
            _ => None,
        }
    }

    fn check_for_macro_directive(&self, start: usize, current: usize) -> Option<TokenType<'a>> {
        match &self.input[start..current] {
            "#macro" => Some(TokenType::Macro),
            "#region" => Some(TokenType::RegionBegin),
            "#endregion" => Some(TokenType::RegionEnd),
            _ => None,
        }
    }
}

#[cfg(test)]
mod scanner_test {
    use super::Scanner;
    use super::*;

    #[test]
    fn lex_basic_symbols() {
        let input_string = "// this is a comment
(){}[] // grouping stuff
!*+-/=%<> >= <= == & | ^ # ? // operators
.:;, // dots and commas
&& || ^^ // logical operators";

        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
            &vec![
                // line 0
                Token::new(TokenType::Comment("// this is a comment"), 0, 0),
                Token::new(TokenType::Newline, 0, 20),
                // line 1
                Token::new(TokenType::LeftParen, 1, 0),
                Token::new(TokenType::RightParen, 1, 1),
                Token::new(TokenType::LeftBrace, 1, 2),
                Token::new(TokenType::RightBrace, 1, 3),
                Token::new(TokenType::LeftBracket, 1, 4),
                Token::new(TokenType::RightBracket, 1, 5),
                Token::new(TokenType::Comment("// grouping stuff"), 1, 7),
                Token::new(TokenType::Newline, 1, 24),
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
                Token::new(TokenType::BitAnd, 2, 19),
                Token::new(TokenType::BitOr, 2, 21),
                Token::new(TokenType::BitXor, 2, 23),
                Token::new(TokenType::Hashtag, 2, 25),
                Token::new(TokenType::Hook, 2, 27),
                Token::new(TokenType::Comment("// operators"), 2, 29),
                Token::new(TokenType::Newline, 2, 41),
                // line 3
                Token::new(TokenType::Dot, 3, 0),
                Token::new(TokenType::Colon, 3, 1),
                Token::new(TokenType::Semicolon, 3, 2),
                Token::new(TokenType::Comma, 3, 3),
                Token::new(TokenType::Comment("// dots and commas"), 3, 5),
                Token::new(TokenType::Newline, 3, 23),
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
        let input_string = "\"This is a good string.\"
\"This is a bad string.
\"This is another good string!\"";
        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
            &vec![
                Token::new(TokenType::String("\"This is a good string.\""), 0, 0),
                Token::new(TokenType::Newline, 0, 24),
                Token::new(TokenType::String("\"This is a bad string."), 1, 0),
                Token::new(TokenType::Newline, 1, 22),
                Token::new(TokenType::String("\"This is another good string!\""), 2, 0),
                Token::new(TokenType::EOF, 2, 30)
            ]
        );
    }

    #[test]
    fn lex_numbers() {
        let input_string = "314159
3.14159
314159.
.314159
4
9
0";

        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
            &vec![
                Token::new(TokenType::Number("314159"), 0, 0),
                Token::new(TokenType::Newline, 0, 6),
                Token::new(TokenType::Number("3.14159"), 1, 0),
                Token::new(TokenType::Newline, 1, 7),
                Token::new(TokenType::Number("314159."), 2, 0),
                Token::new(TokenType::Newline, 2, 7),
                Token::new(TokenType::Number(".314159"), 3, 0),
                Token::new(TokenType::Newline, 3, 7),
                Token::new(TokenType::Number("4"), 4, 0),
                Token::new(TokenType::Newline, 4, 1),
                Token::new(TokenType::Number("9"), 5, 0),
                Token::new(TokenType::Newline, 5, 1),
                Token::new(TokenType::Number("0"), 6, 0),
                Token::new(TokenType::EOF, 6, 1),
            ]
        );
    }

    #[test]
    fn lex_hex() {
        let input_string = "0123456789
0x01234567
0x0A1B2C3D4E5F6
0xABCDEF
$012345
$0A1B2C3D4E5F6
$ABCDEF";

        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
            &vec![
                Token::new(TokenType::Number("0123456789"), 0, 0),
                Token::new(TokenType::Newline, 0, 10),
                Token::new(TokenType::Number("0x01234567"), 1, 0),
                Token::new(TokenType::Newline, 1, 10),
                Token::new(TokenType::Number("0x0A1B2C3D4E5F6"), 2, 0),
                Token::new(TokenType::Newline, 2, 15),
                Token::new(TokenType::Number("0xABCDEF"), 3, 0),
                Token::new(TokenType::Newline, 3, 8),
                Token::new(TokenType::Number("$012345"), 4, 0),
                Token::new(TokenType::Newline, 4, 7),
                Token::new(TokenType::Number("$0A1B2C3D4E5F6"), 5, 0),
                Token::new(TokenType::Newline, 5, 14),
                Token::new(TokenType::Number("$ABCDEF"), 6, 0),
                Token::new(TokenType::EOF, 6, 7),
            ]
        );
    }

    #[test]
    fn lex_basic_identifiers() {
        let input_string = "a
Z
AbCdE
_test
_test123
test_123
testCase";

        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
            &vec![
                Token::new(TokenType::Identifier("a"), 0, 0),
                Token::new(TokenType::Newline, 0, 1),
                Token::new(TokenType::Identifier("Z"), 1, 0),
                Token::new(TokenType::Newline, 1, 1),
                Token::new(TokenType::Identifier("AbCdE"), 2, 0),
                Token::new(TokenType::Newline, 2, 5),
                Token::new(TokenType::Identifier("_test"), 3, 0),
                Token::new(TokenType::Newline, 3, 5),
                Token::new(TokenType::Identifier("_test123"), 4, 0),
                Token::new(TokenType::Newline, 4, 8),
                Token::new(TokenType::Identifier("test_123"), 5, 0),
                Token::new(TokenType::Newline, 5, 8),
                Token::new(TokenType::Identifier("testCase"), 6, 0),
                Token::new(TokenType::EOF, 6, 8),
            ]
        )
    }

    #[test]
    fn lex_reserved_keywords() {
        let input_string =
            "var and or if else return for repeat while do until switch case default true false div break enum";

        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
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
                Token::new(TokenType::Break, 0, 87),
                Token::new(TokenType::Enum, 0, 93),
                Token::new(TokenType::EOF, 0, 97),
            ]
        )
    }

    #[test]
    fn lex_alias_words() {
        let input_string = "and not or mod";

        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
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

        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
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

        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
            &vec![
                Token::new(TokenType::RegionBegin, 0, 0),
                Token::new(TokenType::Identifier("Region"), 0, 8),
                Token::new(TokenType::Identifier("Name"), 0, 15),
                Token::new(TokenType::Identifier("Long"), 0, 20),
                Token::new(TokenType::Newline, 0, 24),
                Token::new(TokenType::Macro, 1, 0),
                Token::new(TokenType::Identifier("macroName"), 1, 7),
                Token::new(TokenType::Number("0"), 1, 17),
                Token::new(TokenType::Newline, 1, 18),
                Token::new(TokenType::RegionEnd, 2, 0),
                Token::new(TokenType::Newline, 2, 10),
                Token::new(TokenType::Macro, 3, 0),
                Token::new(TokenType::Identifier("doing"), 3, 7),
                Token::new(TokenType::Identifier("this"), 3, 13),
                Token::new(TokenType::Backslash, 3, 18),
                Token::new(TokenType::Newline, 3, 19),
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
        let vec = &mut Vec::new();
        let mut scanner = Scanner::new(input_string, vec);

        assert_eq!(
            scanner.lex_input(),
            &vec![
                // line 0
                Token::new(TokenType::Comment("// normal comment"), 0, 0),
                Token::new(TokenType::Newline, 0, 17),
                // line 1
                Token::new(TokenType::Var, 1, 0),
                Token::new(TokenType::Identifier("x"), 1, 4),
                Token::new(TokenType::Equal, 1, 6),
                Token::new(TokenType::Number("20"), 1, 8),
                Token::new(TokenType::Semicolon, 1, 10),
                Token::new(TokenType::Comment("// end comment"), 1, 12),
                Token::new(TokenType::Newline, 1, 26),
                // line 2©
                Token::new(TokenType::MultilineComment("/* one liner */"), 2, 0),
                Token::new(TokenType::Newline, 2, 15),
                // line 3
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
