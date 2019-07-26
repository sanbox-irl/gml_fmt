use super::lex_token::TokenType;
use super::lex_token::*;

pub struct Printer {
    pub output_string: String,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            output_string: String::new(),
        }
    }

    pub fn autoformat(&mut self, tokens: &Vec<Token>) -> &str {
        let mut iter = tokens.iter().enumerate();
        while let Some((_, this_token)) = iter.next() {
            match this_token.token_type {
                TokenType::LeftParen => {}
                TokenType::RightParen => {}
                TokenType::LeftBrace => {}
                TokenType::RightBrace => {}
                TokenType::LeftBracket => {}
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

        &self.output_string
    }
}
