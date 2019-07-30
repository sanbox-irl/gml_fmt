use super::expressions::*;
use super::lex_token::TokenType;
use super::lex_token::*;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::slice;

pub struct Parser<'a> {
    pub ast: Vec<Box<Expr<'a>>>,
    iter: Peekable<Enumerate<slice::Iter<'a, Token<'a>>>>,
    tokens: &'a Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            ast: Vec::new(),
            iter: tokens.iter().enumerate().peekable(),
            tokens,
        }
    }

    pub fn build_ast(&mut self) {
        loop {
            match self.expression() {
                Some(this_box) => self.ast.push(this_box),
                None => break,
            }
        }
    }

    fn expression(&mut self) -> Option<Box<Expr<'a>>> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Box<Expr<'a>>> {
        let expr = self.comparison();

        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::EqualEqual | TokenType::BangEqual => {
                    let (i, _) = self.iter.next().unwrap();
                    let right = self.comparison();

                    return Some(Box::new(Expr::Binary {
                        left: expr,
                        operator: self.tokens[i],
                        right,
                    }));
                }
                _ => break,
            };
        }

        expr
    }

    fn comparison(&mut self) -> Option<Box<Expr<'a>>> {
        let expr = self.addition();

        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    let (i, _) = self.iter.next().unwrap();
                    let right = self.addition();

                    return Some(Box::new(Expr::Binary {
                        left: expr,
                        operator: self.tokens[i],
                        right,
                    }));
                }
                _ => break,
            };
        }

        expr
    }

    fn addition(&mut self) -> Option<Box<Expr<'a>>> {
        let expr = self.multiplication();

        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::Minus | TokenType::Plus => {
                    let (i, _) = self.iter.next().unwrap();
                    let right = self.multiplication();

                    return Some(Box::new(Expr::Binary {
                        left: expr,
                        operator: self.tokens[i],
                        right,
                    }));
                }
                _ => break,
            };
        }

        expr
    }

    fn multiplication(&mut self) -> Option<Box<Expr<'a>>> {
        let expr = self.unary();

        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::Slash | TokenType::Star => {
                    let (i, _) = self.iter.next().unwrap();
                    let right = self.unary();

                    return Some(Box::new(Expr::Binary {
                        left: expr,
                        operator: self.tokens[i],
                        right,
                    }));
                }
                _ => break,
            };
        }

        expr
    }

    fn unary(&mut self) -> Option<Box<Expr<'a>>> {
        if let Some((_, t)) = self.iter.peek() {
            if let TokenType::Bang | TokenType::Minus = t.token_type {
                let (i, _) = self.iter.next().unwrap();
                let right = self.unary();

                return Some(Box::new(Expr::Unary {
                    operator: self.tokens[i],
                    right,
                }));
            }
        };

        self.primary()
    }

    fn primary(&mut self) -> Option<Box<Expr<'a>>> {
        if let Some((i, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::False | TokenType::True | TokenType::RightParen => {
                    return Some(Box::new(Expr::Literal { literal_token: **t }));
                }
                TokenType::Number(_) | TokenType::String(_) => {
                    return Some(Box::new(Expr::Literal { literal_token: **t }));
                }
                TokenType::LeftParen => {
                    let expression = self.expression();
                    return Some(Box::new(Expr::Grouping { expression }));
                }

                _ => {}
            }
        }

        None
    }
}

// #[cfg(test)]
// mod scanner_test {
//     use super::*;

//     #[test]
//     fn
// }
