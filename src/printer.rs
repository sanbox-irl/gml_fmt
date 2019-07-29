use super::lex_token::TokenType;
use super::lex_token::*;

pub struct Printer {
    pub builder: Vec<char>,
    indent: i32,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            builder: Vec::new(),
            indent: 0
        }
    }

    pub fn autoformat(&mut self, tokens: &Vec<Token>) -> &Vec<char> {
        let mut iter = tokens.iter().enumerate();

        while let Some((_, this_token)) = iter.next() {
            match this_token.token_type {
                TokenType::LeftParen => {

                    if let Some(this_last_token) = last_token {
                        match this_last_token {
                            TokenType::Identifier(token) => {}
                            _ => {
                                self.whitespace();
                            }
                        }
                    }

                    self.build('(');
                }
                TokenType::RightParen => self.builder.push(')'),
                TokenType::LeftBrace => {
                    self.build('{');

                    self.new_line();
                    self.indent += 1;
                    self.build_indent();
                }
                TokenType::RightBrace => {
                    self.new_line();

                    self.indent -= 1;
                    self.build_indent();

                    self.build('}');
                }
                TokenType::LeftBracket => {

                }
                TokenType::RightBracket => {}
                TokenType::Comma => {}
                TokenType::Dot => {}
                TokenType::Minus => {}
                TokenType::Plus => {}
                TokenType::Colon => {}
                TokenType::Semicolon => {}
                TokenType::Slash => {}
                TokenType::Backslash => {}
                TokenType::Star => {}
                TokenType::Mod => {}
                TokenType::Hashtag => {}
                TokenType::ListIndexer => {}
                TokenType::MapIndexer => {}
                TokenType::GridIndexer => {}
                TokenType::ArrayIndexer => {}
                TokenType::LogicalAnd => {}
                TokenType::LogicalOr => {}
                TokenType::LogicalXor => {}
                TokenType::BinaryAnd => {}
                TokenType::BinaryOr => {}
                TokenType::BinaryXor => {}
                TokenType::Bang => {}
                TokenType::Hook => {}
                TokenType::BangEqual => {}
                TokenType::Equal => {}
                TokenType::EqualEqual => {}
                TokenType::Greater => {}
                TokenType::GreaterEqual => {}
                TokenType::Less => {}
                TokenType::LessEqual => {}
                TokenType::Macro => {}
                TokenType::RegionBegin => {}
                TokenType::RegionEnd => {}
                TokenType::Var => {}
                TokenType::If => {}
                TokenType::Else => {}
                TokenType::Return => {}
                TokenType::For => {}
                TokenType::Repeat => {}
                TokenType::While => {}
                TokenType::Do => {}
                TokenType::Until => {}
                TokenType::Switch => {}
                TokenType::Case => {}
                TokenType::DefaultCase => {}
                TokenType::Break => {}
                TokenType::True => {}
                TokenType::False => {}
                TokenType::AndAlias => {}
                TokenType::OrAlias => {}
                TokenType::NotAlias => {}
                TokenType::ModAlias => {}
                TokenType::Div => {}
                TokenType::Identifier(associated_str) => {}
                TokenType::String(associated_str) => {}
                TokenType::Number(associated_str) => {}
                TokenType::Comment(associated_str) => {}
                TokenType::MultilineComment(associated_str) => {}
                TokenType::EOF => {
                    break;
                }
            }
        }

        &self.builder
    }

    fn build(&mut self, c: char) {
        self.builder.push(c);
    }

    fn new_line(&mut self) {
        self.builder.push('\n');
    }

    fn whitespace(&mut self) {
        self.builder.push(' ');
    }

    fn build_indent(&mut self) {
        for _ in 0..self.indent {
            self.builder.push('\t');
        }
    }

    fn check_last_is(&mut self, tok: TokenType) -> bool {
        if let Some(this_token) = self.builder.last() {
            match this_token {
                tok => return true,
                _ => return false
            };
        };

        false
    }
}
