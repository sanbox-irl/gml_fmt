use super::lex_token::*;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    input: &'a str,
    line_number: u32,
    column_number: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Scanner<'a> {
        Scanner {
            input,
            line_number: 0,
            column_number: 0,
        }
    }

    pub fn lex_input(&mut self, mut tokens: &mut Vec<Token<'a>>) {
        let mut iter = self.input.chars().enumerate().peekable();

        while let Some((i, c)) = iter.next() {
            match c {
                // Single Char
                '(' => self.add_simple_token(TokenType::LeftParen, &mut tokens),
                ')' => self.add_simple_token(TokenType::RightParen, &mut tokens),
                '{' => self.add_simple_token(TokenType::LeftBrace, &mut tokens),
                '}' => self.add_simple_token(TokenType::RightBrace, &mut tokens),
                ',' => self.add_simple_token(TokenType::Comma, &mut tokens),
                '-' => self.add_simple_token(TokenType::Minus, &mut tokens),
                '+' => self.add_simple_token(TokenType::Plus, &mut tokens),
                ';' => self.add_simple_token(TokenType::Semicolon, &mut tokens),
                '*' => self.add_simple_token(TokenType::Star, &mut tokens),
                ':' => self.add_simple_token(TokenType::Colon, &mut tokens),
                '%' => self.add_simple_token(TokenType::Mod, &mut tokens),
                ']' => self.add_simple_token(TokenType::RightBracket, &mut tokens),
                '?' => self.add_simple_token(TokenType::Hook, &mut tokens),

                // Branching multichar symbols
                '!' => {
                    if self.peek_and_check_consume(&mut iter, '=') {
                        self.add_multiple_token(TokenType::BangEqual, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::Bang, &mut tokens)
                    }
                }
                '=' => {
                    if self.peek_and_check_consume(&mut iter, '=') {
                        self.add_multiple_token(TokenType::EqualEqual, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::Equal, &mut tokens)
                    }
                }
                '<' => {
                    if self.peek_and_check_consume(&mut iter, '=') {
                        self.add_multiple_token(TokenType::LessEqual, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::Less, &mut tokens)
                    }
                }
                '>' => {
                    if self.peek_and_check_consume(&mut iter, '=') {
                        self.add_multiple_token(TokenType::GreaterEqual, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::Greater, &mut tokens)
                    }
                }
                '&' => {
                    if self.peek_and_check_consume(&mut iter, '&') {
                        self.add_multiple_token(TokenType::LogicalAnd, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::BinaryAnd, &mut tokens);
                    }
                }

                '|' => {
                    if self.peek_and_check_consume(&mut iter, '|') {
                        self.add_multiple_token(TokenType::LogicalOr, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::BinaryOr, &mut tokens);
                    }
                }

                '^' => {
                    if self.peek_and_check_consume(&mut iter, '^') {
                        self.add_multiple_token(TokenType::LogicalXor, &mut tokens, 2);
                    } else {
                        self.add_simple_token(TokenType::BinaryXor, &mut tokens);
                    }
                }

                '[' => match iter.peek() {
                    Some((_i, next_char)) if *next_char == '@' => {
                        self.add_multiple_token(TokenType::ArrayIndexer, &mut tokens, 2);
                        iter.next();
                    }

                    Some((_i, next_char)) if *next_char == '?' => {
                        self.add_multiple_token(TokenType::MapIndexer, &mut tokens, 2);
                        iter.next();
                    }

                    Some((_i, next_char)) if *next_char == '|' => {
                        self.add_multiple_token(TokenType::ListIndexer, &mut tokens, 2);
                        iter.next();
                    }

                    Some((_i, next_char)) if *next_char == '#' => {
                        self.add_multiple_token(TokenType::GridIndexer, &mut tokens, 2);
                        iter.next();
                    }

                    _ => self.add_simple_token(TokenType::LeftBracket, &mut tokens),
                },

                // Compiler Directives
                '#' => {
                    let start = i;
                    let mut current = start;

                    if let None = self.peek_and_check_while(&mut iter, |i, this_char| {
                        current = i;
                        this_char.is_ascii_alphanumeric() || this_char == '_'
                    }) {
                        current += 1;
                    };

                    let token_returned = self.check_for_macro_directive(start, current);

                    match token_returned {
                        Some(macro_directive) => self.add_multiple_token(
                            macro_directive,
                            &mut tokens,
                            (current - start) as u32,
                        ),

                        None => {
                            // we're adding a hashtag token, which doesn't really mean anything,
                            // but just want to keep the sizes right.
                            self.add_simple_token(TokenType::Hashtag, &mut tokens);

                            // for a weird # floating in space
                            if current - start - 1 != 0 {
                                self.add_multiple_token(
                                    TokenType::Identifier(&self.input[start..current]),
                                    &mut tokens,
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

                    if let Some((i, break_char)) =
                        self.peek_and_check_while(&mut iter, |i, string_char| {
                            current = i;
                            string_char != '\n' && string_char != '"'
                        })
                    {
                        // eat the quote
                        if break_char == '"' {
                            iter.next();
                            current = i + 1;
                        }
                    }

                    self.add_multiple_token(
                        TokenType::String(&self.input[start..current]),
                        &mut tokens,
                        (current - start) as u32,
                    );
                }

                // Number literals
                '.' => {
                    match iter.peek() {
                        Some((_, next_char)) if next_char.is_digit(10) => {
                            let start = i;
                            let mut current = start;

                            // eat the "."
                            iter.next();

                            while let Some((i, number_char)) = iter.peek() {
                                if number_char.is_digit(10) {
                                    current = *i + 1;
                                    iter.next();
                                } else {
                                    break;
                                }
                            }

                            self.add_multiple_token(
                                TokenType::Number(&self.input[start..current]),
                                &mut tokens,
                                (current - start) as u32,
                            );
                        }
                        _ => self.add_simple_token(TokenType::Dot, &mut tokens),
                    }
                }

                '0'..='9' => {
                    let start = i;
                    let mut current = start + 1;

                    // Check for Hex
                    if c == '0' {
                        if let Some((_, number_char)) = iter.peek() {
                            if *number_char == 'x' {
                                iter.next();

                                while let Some((i, number_char)) = iter.peek() {
                                    if number_char.is_digit(16) {
                                        current = *i + 1;
                                        iter.next();
                                    } else {
                                        break;
                                    }
                                }

                                self.add_multiple_token(
                                    TokenType::Number(&self.input[start..current]),
                                    &mut tokens,
                                    (current - start) as u32,
                                );
                                continue;
                            }
                        }
                    }

                    let mut is_fractional = false;

                    while let Some((i, number_char)) = iter.peek() {
                        if number_char.is_digit(10) {
                            current = *i + 1;
                            iter.next();
                        } else {
                            is_fractional = *number_char == '.';
                            break;
                        }
                    }

                    if is_fractional {
                        // eat the "."
                        current = iter.next().unwrap().0 + 1;

                        while let Some((i, number_char)) = iter.peek() {
                            if number_char.is_digit(10) {
                                current = *i + 1;
                                iter.next();
                            } else {
                                break;
                            }
                        }
                    }

                    self.add_multiple_token(
                        TokenType::Number(&self.input[start..current]),
                        &mut tokens,
                        (current - start) as u32,
                    )
                }

                // Secondary Hex
                '$' => {
                    let start = i;
                    let mut current = start;

                    if let None = self.peek_and_check_while(&mut iter, |i, hex_char| {
                        current = i;
                        hex_char.is_digit(16)
                    }) {
                        current += 1;
                    }

                    self.add_multiple_token(
                        TokenType::Number(&self.input[start..current]),
                        &mut tokens,
                        (current - start) as u32,
                    );
                }

                // Comments
                '/' => {
                    if self.peek_and_check_consume(&mut iter, '/') {
                        let start = i;
                        let mut current = start;

                        if let None = self.peek_and_check_while(&mut iter, |i, this_char| {
                            current = i;
                            this_char != '\n'
                        }) {
                            current += 1;
                        }

                        self.add_multiple_token(
                            TokenType::Comment(&self.input[start..current]),
                            &mut tokens,
                            (current - start) as u32,
                        );
                    } else {
                        self.add_simple_token(TokenType::Slash, &mut tokens);
                    }
                }
                ' ' | '\t' => self.column_number += 1,

                '\n' => self.next_line(),
                '\r' => continue,

                'A'..='Z' | 'a'..='z' | '_' => {
                    let start = i;
                    let mut current = start + 1;

                    if let None = self.peek_and_check_while(&mut iter, |i, this_char| {
                        current = i;
                        this_char.is_ascii_alphanumeric() || this_char == '_'
                    }) {
                        current += 1;
                    };

                    let keyword_token_type: Option<TokenType> =
                        self.check_for_keyword(start, current);

                    match keyword_token_type {
                        Some(token_type) => self.add_multiple_token(
                            token_type,
                            &mut tokens,
                            (current - start) as u32,
                        ),
                        None => self.add_multiple_token(
                            TokenType::Identifier(&self.input[start..current]),
                            &mut tokens,
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

        self.add_simple_token(TokenType::EOF, tokens);
    }

    fn add_simple_token(&mut self, token_type: TokenType<'a>, tokens: &mut Vec<Token<'a>>) {
        self.add_multiple_token(token_type, tokens, 1);
    }

    fn add_multiple_token(
        &mut self,
        token_type: TokenType<'a>,
        tokens: &mut Vec<Token<'a>>,
        size: u32,
    ) {
        tokens.push(Token::new(token_type, self.line_number, self.column_number));
        self.column_number += size;
    }

    fn peek_and_check_consume(
        &self,
        iter: &mut Peekable<Enumerate<Chars>>,
        char_to_check: char,
    ) -> bool {
        if let Some((_i, next_char)) = iter.peek() {
            let ret = next_char == &char_to_check;
            if ret {
                iter.next();
            }
            ret
        } else {
            false
        }
    }

    fn peek_and_check_while<F>(
        &self,
        iter: &mut Peekable<Enumerate<Chars>>,
        mut f: F,
    ) -> Option<(usize, char)>
    where
        F: FnMut(usize, char) -> bool,
    {
        while let Some((i, next_char)) = iter.peek() {
            if f(*i, *next_char) == false {
                return Some((*i, *next_char));
            };
            iter.next();
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
