use super::lex_token::*;
use fnv::FnvHashMap;
use once_cell::sync::Lazy;
use std::iter::Peekable;
use std::str::CharIndices;

static KEYWORD_MAP: Lazy<FnvHashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = FnvHashMap::with_capacity_and_hasher(25, Default::default());
    map.insert("var", TokenType::Var);
    map.insert("and", TokenType::AndAlias);
    map.insert("or", TokenType::OrAlias);
    map.insert("not", TokenType::NotAlias);
    map.insert("if", TokenType::If);
    map.insert("else", TokenType::Else);
    map.insert("function", TokenType::Function);
    map.insert("constructor", TokenType::Constructor);
    map.insert("new", TokenType::New);
    map.insert("delete", TokenType::Delete);
    map.insert("return", TokenType::Return);
    map.insert("for", TokenType::For);
    map.insert("repeat", TokenType::Repeat);
    map.insert("while", TokenType::While);
    map.insert("do", TokenType::Do);
    map.insert("until", TokenType::Until);
    map.insert("switch", TokenType::Switch);
    map.insert("case", TokenType::Case);
    map.insert("default", TokenType::DefaultCase);
    map.insert("mod", TokenType::ModAlias);
    map.insert("div", TokenType::Div);
    map.insert("break", TokenType::Break);
    map.insert("exit", TokenType::Exit);
    map.insert("enum", TokenType::Enum);
    map.insert("with", TokenType::With);
    map.insert("then", TokenType::Then);
    map.insert("globalvar", TokenType::GlobalVar);
    map
});

pub struct Scanner<'a> {
    input: &'a str,
    line_number: u32,
    column_number: u32,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Scanner<'a> {
        Scanner {
            input,
            line_number: 0,
            column_number: 0,
            iter: input.char_indices().peekable(),
        }
    }

    pub fn lex_input(&mut self) -> Option<Token<'a>> {
        while let Some((i, c)) = self.iter.next() {
            let found_token = match c {
                '(' => self.add_simple_token(TokenType::LeftParen),
                ')' => self.add_simple_token(TokenType::RightParen),
                '{' => self.add_simple_token(TokenType::LeftBrace),
                '}' => self.add_simple_token(TokenType::RightBrace),
                ',' => self.add_simple_token(TokenType::Comma),
                '~' => self.add_simple_token(TokenType::Tilde),
                '-' => {
                    if let Some((_, c)) = self.iter.peek() {
                        if let Some(token) = match c {
                            '=' => Some(TokenType::MinusEquals),
                            '-' => Some(TokenType::Decrementer),
                            _ => None,
                        } {
                            self.iter.next();
                            self.add_multiple_token(token, 2)
                        } else {
                            self.add_simple_token(TokenType::Minus)
                        }
                    } else {
                        self.add_simple_token(TokenType::Minus)
                    }
                }
                '+' => {
                    if let Some((_, c)) = self.iter.peek() {
                        if let Some(token) = match c {
                            '=' => Some(TokenType::PlusEquals),
                            '+' => Some(TokenType::Incrementer),
                            _ => None,
                        } {
                            self.iter.next();
                            self.add_multiple_token(token, 2)
                        } else {
                            self.add_simple_token(TokenType::Plus)
                        }
                    } else {
                        self.add_simple_token(TokenType::Plus)
                    }
                }

                ';' => self.add_simple_token(TokenType::Semicolon),
                '*' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::StarEquals, 2)
                    } else {
                        self.add_simple_token(TokenType::Star)
                    }
                }
                ':' => self.add_simple_token(TokenType::Colon),
                '%' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::ModEquals, 2)
                    } else {
                        self.add_simple_token(TokenType::Mod)
                    }
                }
                ']' => self.add_simple_token(TokenType::RightBracket),
                '?' => self.add_simple_token(TokenType::Hook),
                '\\' => self.add_simple_token(TokenType::Backslash),
                '!' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::BangEqual, 2)
                    } else {
                        self.add_simple_token(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::EqualEqual, 2)
                    } else {
                        self.add_simple_token(TokenType::Equal)
                    }
                }
                '<' => {
                    if let Some((_, c)) = self.iter.peek() {
                        match c {
                            '=' => {
                                self.iter.next();
                                self.add_multiple_token(TokenType::LessEqual, 2)
                            }
                            '>' => {
                                self.iter.next();
                                self.add_multiple_token(TokenType::LessThanGreaterThan, 2)
                            }
                            '<' => {
                                self.iter.next();
                                self.add_multiple_token(TokenType::BitLeft, 2)
                            }
                            _ => self.add_simple_token(TokenType::Less),
                        }
                    } else {
                        self.add_simple_token(TokenType::Less)
                    }
                }
                '>' => {
                    if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::GreaterEqual, 2)
                    } else if self.peek_and_check_consume('>') {
                        self.add_multiple_token(TokenType::BitRight, 2)
                    } else {
                        self.add_simple_token(TokenType::Greater)
                    }
                }

                '&' => {
                    if let Some((_, c)) = self.iter.peek() {
                        if let Some(token) = match c {
                            '&' => Some(TokenType::LogicalAnd),
                            '=' => Some(TokenType::BitAndEquals),
                            _ => None,
                        } {
                            self.iter.next();
                            self.add_multiple_token(token, 2)
                        } else {
                            self.add_simple_token(TokenType::BitAnd)
                        }
                    } else {
                        self.add_simple_token(TokenType::BitAnd)
                    }
                }

                '|' => {
                    if let Some((_, c)) = self.iter.peek() {
                        if let Some(token) = match c {
                            '|' => Some(TokenType::LogicalOr),
                            '=' => Some(TokenType::BitOrEquals),
                            _ => None,
                        } {
                            self.iter.next();
                            self.add_multiple_token(token, 2)
                        } else {
                            self.add_simple_token(TokenType::BitOr)
                        }
                    } else {
                        self.add_simple_token(TokenType::BitOr)
                    }
                }

                '^' => {
                    if let Some((_, c)) = self.iter.peek() {
                        if let Some(token) = match c {
                            '^' => Some(TokenType::LogicalXor),
                            '=' => Some(TokenType::BitXorEquals),
                            _ => None,
                        } {
                            self.iter.next();
                            self.add_multiple_token(token, 2)
                        } else {
                            self.add_simple_token(TokenType::BitXor)
                        }
                    } else {
                        self.add_simple_token(TokenType::BitXor)
                    }
                }

                // Indexing
                '[' => {
                    if let Some((_, next_char)) = self.iter.peek() {
                        match next_char {
                            '@' => {
                                self.iter.next();
                                self.add_multiple_token(TokenType::ArrayIndexer, 2)
                            }

                            '?' => {
                                self.iter.next();
                                self.add_multiple_token(TokenType::MapIndexer, 2)
                            }

                            '|' => {
                                self.iter.next();
                                self.add_multiple_token(TokenType::ListIndexer, 2)
                            }

                            '#' => {
                                self.iter.next();
                                self.add_multiple_token(TokenType::GridIndexer, 2)
                            }

                            _ => self.add_simple_token(TokenType::LeftBracket),
                        }
                    } else {
                        self.add_simple_token(TokenType::LeftBracket)
                    }
                }

                // Compiler Directives
                '#' => {
                    let start = i;

                    // Multiline macro stuff
                    let start_line = self.line_number;
                    let start_column = self.column_number;
                    let mut is_multiline = false;
                    let mut last_column_break = start;

                    // Get our first word, with the hashtag
                    while let Some((_, next_char)) = self.iter.peek() {
                        if (next_char.is_ascii_alphanumeric() || *next_char == '_') == false {
                            break;
                        };
                        self.iter.next();
                    }

                    let mut current = self.next_char_boundary();
                    let token_returned = match &self.input[start..current] {
                        "#macro" => {
                            while let Some((_, peek_char)) = self.iter.peek() {
                                match peek_char {
                                    '\n' => break,

                                    '\\' => {
                                        self.iter.next();
                                        if self.peek_and_check_consume('\n') {
                                            last_column_break = self.next_char_boundary();
                                            is_multiline = true;
                                            self.next_line();
                                        }
                                    }

                                    _ => {
                                        self.iter.next().unwrap();
                                    }
                                }
                            }
                            current = self.next_char_boundary();
                            Some(TokenType::Macro(&self.input[start..current]))
                        }
                        "#region" => {
                            while let Some((_, peek_char)) = self.iter.peek() {
                                match peek_char {
                                    '\n' => break,
                                    _ => {
                                        self.iter.next().unwrap();
                                    }
                                }
                            }
                            Some(TokenType::RegionBegin(&self.input[start..self.next_char_boundary()]))
                        }
                        "#endregion" => {
                            while let Some((_, peek_char)) = self.iter.peek() {
                                match peek_char {
                                    '\n' => break,
                                    _ => {
                                        self.iter.next().unwrap();
                                    }
                                }
                            }
                            Some(TokenType::RegionEnd(&self.input[start..self.next_char_boundary()]))
                        }
                        "#define" => Some(TokenType::Define),
                        _ => None,
                    };

                    current = self.next_char_boundary();
                    match token_returned {
                        Some(macro_directive) => {
                            if is_multiline {
                                self.column_number += (current - last_column_break) as u32;
                                Token::new(TokenType::Macro(&self.input[start..current]), start_line, start_column)
                            } else {
                                self.add_multiple_token(macro_directive, (current - start) as u32)
                            }
                        }

                        None => {
                            // we're adding a hashtag token, which doesn't really mean anything,
                            // but just want to keep the sizes right.
                            self.add_simple_token(TokenType::Hashtag)

                            // for a weird # floating in space
                            // if current - start - 1 != 0 {
                            //     self.add_multiple_token(
                            //         TokenType::Identifier(&self.input[start..current]),
                            //         (current - start - 1) as u32,
                            //     )
                            // }
                        }
                    }
                }

                // string literals
                '@' => {
                    let start = i;
                    let start_line = self.line_number;
                    let start_column = self.column_number;

                    if let Some((_, this_char)) = self.iter.peek() {
                        match this_char {
                            '\'' | '\"' => {
                                let (_, this_char) = self.iter.next().unwrap();
                                let (current, last_column_break) = self.scan_multiline_string(start, this_char);

                                self.column_number += (current - last_column_break) as u32;
                                Token::new(TokenType::String(&self.input[start..current]), start_line, start_column)
                            }

                            _ => {
                                let end_byte = self.next_char_boundary();
                                self.add_multiple_token(
                                    TokenType::UnidentifiedInput(&self.input[i..end_byte]),
                                    (end_byte - i) as u32,
                                )
                            }
                        }
                    } else {
                        let end_byte = self.next_char_boundary();
                        self.add_multiple_token(
                            TokenType::UnidentifiedInput(&self.input[i..end_byte]),
                            (end_byte - i) as u32,
                        )
                    }
                }
                '"' => {
                    let start = i;
                    let mut current = start;

                    while let Some((_, break_char)) = self.iter.peek() {
                        match *break_char {
                            '\n' => {
                                current = self.next_char_boundary();
                                break;
                            }

                            '\\' => {
                                self.iter.next();
                                if let Some((_, break_char)) = self.iter.peek() {
                                    if *break_char == '"' || *break_char == '\\' {
                                        self.iter.next();
                                        current = self.next_char_boundary();
                                    }
                                }
                            }

                            '"' => {
                                self.iter.next();
                                current = self.next_char_boundary();
                                break;
                            }
                            _ => {
                                self.iter.next();
                                current = self.next_char_boundary();
                            }
                        }
                    }

                    self.add_multiple_token(TokenType::String(&self.input[start..current]), (current - start) as u32)
                }

                '\'' => {
                    let start = i;
                    let mut current = start;

                    while let Some((_, break_char)) = self.iter.peek() {
                        match *break_char {
                            '\n' => {
                                current = self.next_char_boundary();
                                break;
                            }

                            '\\' => {
                                self.iter.next();
                                if let Some((_, break_char)) = self.iter.peek() {
                                    if *break_char == '\'' {
                                        self.iter.next();
                                        current = self.next_char_boundary();
                                    }
                                }
                            }

                            '\'' => {
                                self.iter.next();
                                current = self.next_char_boundary();
                                break;
                            }
                            _ => {
                                self.iter.next();
                                current = self.next_char_boundary();
                            }
                        }
                    }

                    self.add_multiple_token(TokenType::String(&self.input[start..current]), (current - start) as u32)
                }

                '.' => match self.iter.peek() {
                    Some((_, next_char)) if next_char.is_digit(10) => {
                        let start = i;
                        let mut current = start;

                        while let Some((_, number_char)) = self.iter.peek() {
                            if number_char.is_digit(10) {
                                self.iter.next();
                                current = self.next_char_boundary();
                            } else {
                                break;
                            }
                        }

                        self.add_multiple_token(
                            TokenType::NumberStartDot(&self.input[start..current]),
                            (current - start) as u32,
                        )
                    }
                    _ => self.add_simple_token(TokenType::Dot),
                },

                '0'..='9' => {
                    let start = i;

                    // Check for Hex
                    if c == '0' {
                        if let Some((_, number_char)) = self.iter.peek() {
                            if *number_char == 'x' {
                                self.iter.next();

                                while let Some((_, number_char)) = self.iter.peek() {
                                    if number_char.is_digit(16) {
                                        self.iter.next();
                                    } else {
                                        break;
                                    }
                                }

                                let current = self.next_char_boundary();

                                return Some(self.add_multiple_token(
                                    TokenType::Number(&self.input[start..current]),
                                    (current - start) as u32,
                                ));
                            }
                        }
                    }

                    let mut is_fractional = false;
                    while let Some((_, number_char)) = self.iter.peek() {
                        if number_char.is_digit(10) {
                            self.iter.next();
                        } else {
                            is_fractional = *number_char == '.';
                            break;
                        }
                    }
                    let mut current = self.next_char_boundary();

                    if is_fractional {
                        // eat the "."
                        self.iter.next();
                        let mut is_end_dot = true;
                        while let Some((_, number_char)) = self.iter.peek() {
                            if number_char.is_digit(10) {
                                is_end_dot = false;
                                self.iter.next();
                            } else {
                                current = self.next_char_boundary();
                                break;
                            }
                        }
                        if is_end_dot {
                            return Some(self.add_multiple_token(
                                TokenType::NumberEndDot(&self.input[start..current]),
                                (current - start) as u32,
                            ));
                        }
                    }

                    self.add_multiple_token(TokenType::Number(&self.input[start..current]), (current - start) as u32)
                }

                // Secondary Hex
                '$' => {
                    let start = i;
                    let mut current = self.next_char_boundary();

                    while let Some((_, hex_char)) = self.iter.peek() {
                        if hex_char.is_digit(16) {
                            self.iter.next();
                            current = self.next_char_boundary();
                        } else {
                            current = self.next_char_boundary();
                            break;
                        }
                    }

                    self.add_multiple_token(TokenType::Number(&self.input[start..current]), (current - start) as u32)
                }

                // Comments
                '/' => {
                    // Normal Comment
                    if self.peek_and_check_consume('/') {
                        let start = i;
                        while let Some((_, peek_char)) = self.iter.peek() {
                            if *peek_char == '\n' {
                                break;
                            }
                            self.iter.next();
                        }
                        let current = self.next_char_boundary();

                        self.add_multiple_token(
                            TokenType::Comment(&self.input[start..current]),
                            (current - start) as u32,
                        )
                    } else if self.peek_and_check_consume('*') {
                        // Multiline Comment
                        let start = i;
                        let start_line = self.line_number;
                        let start_column = self.column_number;

                        let mut last_column_break = start;
                        let mut current = start;

                        while let Some((_, comment_char)) = self.iter.next() {
                            match comment_char {
                                '*' => {
                                    if let Some((_, next_next_char)) = self.iter.peek() {
                                        if next_next_char == &'/' {
                                            self.iter.next();
                                            current = self.next_char_boundary();
                                            break;
                                        }
                                    }
                                }

                                '\n' => {
                                    last_column_break = self.next_char_boundary();
                                    self.next_line();
                                }

                                _ => {}
                            };
                        }

                        // TODO: Figure out what the bleeding hecc is going on here. Broke with function formatting.
                        if current > last_column_break { // TODO: Find good test for this edge case.
                            self.column_number += (current - last_column_break) as u32;
                        } else {
                            self.column_number = current as u32;
                        }
                        Token::new(
                            TokenType::MultilineComment(&self.input[start..current]),
                            start_line,
                            start_column,
                        )
                    } else if self.peek_and_check_consume('=') {
                        self.add_multiple_token(TokenType::SlashEquals, 2)
                    } else {
                        self.add_simple_token(TokenType::Slash)
                    }
                }

                // Identifiers and keywords
                'A'..='Z' | 'a'..='z' | '_' => {
                    let start = i;
                    while let Some((_, next_char)) = self.iter.peek() {
                        if (next_char.is_ascii_alphanumeric() || *next_char == '_') == false {
                            break;
                        };
                        self.iter.next();
                    }

                    let current = self.next_char_boundary();

                    match KEYWORD_MAP.get(&self.input[start..current]) {
                        Some(token) => {
                            let token = *token;
                            self.add_multiple_token(token, (current - start) as u32)
                        }
                        None => self.add_multiple_token(
                            TokenType::Identifier(&self.input[start..current]),
                            (current - start) as u32,
                        ),
                    }
                }

                // Whitespace we care about...
                ' ' | '\t' => {
                    self.column_number += 1;
                    continue;
                }

                // Newline
                '\n' => {
                    let mut tally = 0;
                    while let Some((_, c)) = self.iter.peek() {
                        match c {
                            ' ' => {
                                self.iter.next();
                                tally += 1;
                            }
                            '\t' => {
                                self.iter.next();
                                tally += 4;
                            }
                            _ => break,
                        };
                    }
                    let ret = self.add_multiple_token(TokenType::Newline(tally / 4), tally as u32);
                    self.next_line();
                    ret
                }

                // Whitespace we don't care about
                '\r' => continue,

                _ => self.return_unidentified_input(i),
            };

            return Some(found_token);
        }

        None
    }

    fn return_unidentified_input(&mut self, start: usize) -> Token<'a> {
        let end_byte = self.next_char_boundary();
        self.add_multiple_token(
            TokenType::UnidentifiedInput(&self.input[start..end_byte]),
            (end_byte - start) as u32,
        )
    }

    fn add_simple_token(&mut self, token_type: TokenType<'a>) -> Token<'a> {
        self.add_multiple_token(token_type, 1)
    }

    fn add_multiple_token(&mut self, token_type: TokenType<'a>, size: u32) -> Token<'a> {
        let ret = Token::new(token_type, self.line_number, self.column_number);
        self.column_number += size;
        ret
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

    fn next_line(&mut self) {
        self.line_number += 1;
        self.column_number = 0;
    }

    fn scan_multiline_string(&mut self, mut last_column_break: usize, break_char: char) -> (usize, usize) {
        while let Some((_, this_char)) = self.iter.next() {
            if this_char == break_char {
                break;
            } else if this_char == '\n' {
                last_column_break = self.next_char_boundary();
                self.next_line();
            }
        }
        (self.next_char_boundary(), last_column_break)
    }

    fn next_char_boundary(&mut self) -> usize {
        match self.iter.peek() {
            Some(_) => self.iter.peek().unwrap().0,
            None => self.input.len(),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.lex_input()
    }
}

#[cfg(test)]
mod scanner_test {
    use super::Scanner;
    use super::*;

    #[test]
    fn lex_symbols<'a>() {
        let input_string = "(){}[] // grouping stuff
! * + - / % & | ^ # ? // binary operators
= == <> > < >= <= // equality operators
.:;, // dots and commas
&& || ^^ // logical operators
+= -= *= /= ^= |= &= %= // set operators";

        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            vec,
            vec![
                // line 0
                Token::new(TokenType::LeftParen, 0, 0),
                Token::new(TokenType::RightParen, 0, 1),
                Token::new(TokenType::LeftBrace, 0, 2),
                Token::new(TokenType::RightBrace, 0, 3),
                Token::new(TokenType::LeftBracket, 0, 4),
                Token::new(TokenType::RightBracket, 0, 5),
                Token::new(TokenType::Comment("// grouping stuff"), 0, 7),
                Token::new(TokenType::Newline(0), 0, 24),
                // line 1
                Token::new(TokenType::Bang, 1, 0),
                Token::new(TokenType::Star, 1, 2),
                Token::new(TokenType::Plus, 1, 4),
                Token::new(TokenType::Minus, 1, 6),
                Token::new(TokenType::Slash, 1, 8),
                Token::new(TokenType::Mod, 1, 10),
                Token::new(TokenType::BitAnd, 1, 12),
                Token::new(TokenType::BitOr, 1, 14),
                Token::new(TokenType::BitXor, 1, 16),
                Token::new(TokenType::Hashtag, 1, 18),
                Token::new(TokenType::Hook, 1, 20),
                Token::new(TokenType::Comment("// binary operators"), 1, 22),
                Token::new(TokenType::Newline(0), 1, 41),
                // line 2
                Token::new(TokenType::Equal, 2, 0),
                Token::new(TokenType::EqualEqual, 2, 2),
                Token::new(TokenType::LessThanGreaterThan, 2, 5),
                Token::new(TokenType::Greater, 2, 8),
                Token::new(TokenType::Less, 2, 10),
                Token::new(TokenType::GreaterEqual, 2, 12),
                Token::new(TokenType::LessEqual, 2, 15),
                Token::new(TokenType::Comment("// equality operators"), 2, 18),
                Token::new(TokenType::Newline(0), 2, 39),
                // line 3
                Token::new(TokenType::Dot, 3, 0),
                Token::new(TokenType::Colon, 3, 1),
                Token::new(TokenType::Semicolon, 3, 2),
                Token::new(TokenType::Comma, 3, 3),
                Token::new(TokenType::Comment("// dots and commas"), 3, 5),
                Token::new(TokenType::Newline(0), 3, 23),
                // line 4
                Token::new(TokenType::LogicalAnd, 4, 0),
                Token::new(TokenType::LogicalOr, 4, 3),
                Token::new(TokenType::LogicalXor, 4, 6),
                Token::new(TokenType::Comment("// logical operators"), 4, 9),
                Token::new(TokenType::Newline(0), 4, 29),
                // line 5
                Token::new(TokenType::PlusEquals, 5, 0),
                Token::new(TokenType::MinusEquals, 5, 3),
                Token::new(TokenType::StarEquals, 5, 6),
                Token::new(TokenType::SlashEquals, 5, 9),
                Token::new(TokenType::BitXorEquals, 5, 12),
                Token::new(TokenType::BitOrEquals, 5, 15),
                Token::new(TokenType::BitAndEquals, 5, 18),
                Token::new(TokenType::ModEquals, 5, 21),
                Token::new(TokenType::Comment("// set operators"), 5, 24),
            ]
        );
    }

    #[test]
    fn lex_strings<'a>() {
        let input_string = "\"This is a good string.\"
\"This is a bad string.
\"\"
\"This is another good string!\"
@\"This is a
multi-linestring. The demon's plaything!\"";
        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
            &vec![
                Token::new(TokenType::String("\"This is a good string.\""), 0, 0),
                Token::new(TokenType::Newline(0), 0, 24),
                Token::new(TokenType::String("\"This is a bad string."), 1, 0),
                Token::new(TokenType::Newline(0), 1, 22),
                Token::new(TokenType::String("\"\""), 2, 0),
                Token::new(TokenType::Newline(0), 2, 2),
                Token::new(TokenType::String("\"This is another good string!\""), 3, 0),
                Token::new(TokenType::Newline(0), 3, 30),
                Token::new(
                    TokenType::String("@\"This is a\nmulti-linestring. The demon's plaything!\""),
                    4,
                    0
                ),
            ]
        );
    }

    #[test]
    fn lex_numbers<'a>() {
        let input_string = "314159
3.14159
314159.
.314159
4
9
0
.3";

        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
            &vec![
                Token::new(TokenType::Number("314159"), 0, 0),
                Token::new(TokenType::Newline(0), 0, 6),
                Token::new(TokenType::Number("3.14159"), 1, 0),
                Token::new(TokenType::Newline(0), 1, 7),
                Token::new(TokenType::NumberEndDot("314159."), 2, 0),
                Token::new(TokenType::Newline(0), 2, 7),
                Token::new(TokenType::NumberStartDot(".314159"), 3, 0),
                Token::new(TokenType::Newline(0), 3, 7),
                Token::new(TokenType::Number("4"), 4, 0),
                Token::new(TokenType::Newline(0), 4, 1),
                Token::new(TokenType::Number("9"), 5, 0),
                Token::new(TokenType::Newline(0), 5, 1),
                Token::new(TokenType::Number("0"), 6, 0),
                Token::new(TokenType::Newline(0), 6, 1),
                Token::new(TokenType::NumberStartDot(".3"), 7, 0),
            ]
        );
    }

    #[test]
    fn lex_hex<'a>() {
        let input_string = "0123456789
0x01234567
0x0A1B2C3D4E5F6
0xABCDEF
0x
$012345
$0A1B2C3D4E5F6
$ABCDEF
$";

        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
            &vec![
                Token::new(TokenType::Number("0123456789"), 0, 0),
                Token::new(TokenType::Newline(0), 0, 10),
                Token::new(TokenType::Number("0x01234567"), 1, 0),
                Token::new(TokenType::Newline(0), 1, 10),
                Token::new(TokenType::Number("0x0A1B2C3D4E5F6"), 2, 0),
                Token::new(TokenType::Newline(0), 2, 15),
                Token::new(TokenType::Number("0xABCDEF"), 3, 0),
                Token::new(TokenType::Newline(0), 3, 8),
                Token::new(TokenType::Number("0x"), 4, 0),
                Token::new(TokenType::Newline(0), 4, 2),
                Token::new(TokenType::Number("$012345"), 5, 0),
                Token::new(TokenType::Newline(0), 5, 7),
                Token::new(TokenType::Number("$0A1B2C3D4E5F6"), 6, 0),
                Token::new(TokenType::Newline(0), 6, 14),
                Token::new(TokenType::Number("$ABCDEF"), 7, 0),
                Token::new(TokenType::Newline(0), 7, 7),
                Token::new(TokenType::Number("$"), 8, 0),
            ]
        );
    }

    #[test]
    fn lex_basic_identifiers<'a>() {
        let input_string = "a
Z
AbCdE
_test
_test123
test_123
testCase";

        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
            &vec![
                Token::new(TokenType::Identifier("a"), 0, 0),
                Token::new(TokenType::Newline(0), 0, 1),
                Token::new(TokenType::Identifier("Z"), 1, 0),
                Token::new(TokenType::Newline(0), 1, 1),
                Token::new(TokenType::Identifier("AbCdE"), 2, 0),
                Token::new(TokenType::Newline(0), 2, 5),
                Token::new(TokenType::Identifier("_test"), 3, 0),
                Token::new(TokenType::Newline(0), 3, 5),
                Token::new(TokenType::Identifier("_test123"), 4, 0),
                Token::new(TokenType::Newline(0), 4, 8),
                Token::new(TokenType::Identifier("test_123"), 5, 0),
                Token::new(TokenType::Newline(0), 5, 8),
                Token::new(TokenType::Identifier("testCase"), 6, 0),
            ]
        )
    }

    #[test]
    fn lex_reserved_keywords<'a>() {
        let input_string = "var and or if else return for repeat while do until switch case default div break enum function constructor new";

        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
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
                Token::new(TokenType::Div, 0, 72),
                Token::new(TokenType::Break, 0, 76),
                Token::new(TokenType::Enum, 0, 82),
                Token::new(TokenType::Function, 0, 87),
                Token::new(TokenType::Constructor, 0, 96),
                Token::new(TokenType::New, 0, 108),
            ]
        )
    }

    #[test]
    fn lex_alias_words<'a>() {
        let input_string = "and not or mod";

        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
            &vec![
                Token::new(TokenType::AndAlias, 0, 0),
                Token::new(TokenType::NotAlias, 0, 4),
                Token::new(TokenType::OrAlias, 0, 8),
                Token::new(TokenType::ModAlias, 0, 11),
            ]
        )
    }

    #[test]
    fn lex_indexers<'a>() {
        let input_string = "[ [? [# [| [@ ]";

        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
            &vec![
                Token::new(TokenType::LeftBracket, 0, 0),
                Token::new(TokenType::MapIndexer, 0, 2),
                Token::new(TokenType::GridIndexer, 0, 5),
                Token::new(TokenType::ListIndexer, 0, 8),
                Token::new(TokenType::ArrayIndexer, 0, 11),
                Token::new(TokenType::RightBracket, 0, 14),
            ]
        )
    }

    #[test]
    fn lex_compiler_directives<'a>() {
        let input_string = "#region Region Name Long
#macro macroName 0
#endregion
#macro doing this \\
is bad";

        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
            &vec![
                Token::new(TokenType::RegionBegin("#region Region Name Long"), 0, 0),
                Token::new(TokenType::Newline(0), 0, 24),
                Token::new(TokenType::Macro("#macro macroName 0"), 1, 0),
                Token::new(TokenType::Newline(0), 1, 18),
                Token::new(TokenType::RegionEnd("#endregion"), 2, 0),
                Token::new(TokenType::Newline(0), 2, 10),
                Token::new(TokenType::Macro("#macro doing this \\\nis bad"), 3, 0),
            ]
        )
    }
    #[test]
    fn lex_comments<'a>() {
        let input_string = "// normal comment
var x = a; // end comment
/* one liner */
/* multi
liner comment
*/";
        let scanner = Scanner::new(input_string);
        let vec: Vec<Token<'a>> = scanner.collect();
        assert_eq!(
            &vec,
            &vec![
                // line 0
                Token::new(TokenType::Comment("// normal comment"), 0, 0),
                Token::new(TokenType::Newline(0), 0, 17),
                // line 1
                Token::new(TokenType::Var, 1, 0),
                Token::new(TokenType::Identifier("x"), 1, 4),
                Token::new(TokenType::Equal, 1, 6),
                Token::new(TokenType::Identifier("a"), 1, 8),
                Token::new(TokenType::Semicolon, 1, 9),
                Token::new(TokenType::Comment("// end comment"), 1, 11),
                Token::new(TokenType::Newline(0), 1, 25),
                // line 2
                Token::new(TokenType::MultilineComment("/* one liner */"), 2, 0),
                Token::new(TokenType::Newline(0), 2, 15),
                // line 3
                Token::new(TokenType::MultilineComment("/* multi\nliner comment\n*/"), 3, 0),
            ]
        )
    }
}
