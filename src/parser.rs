use super::expressions::*;
use super::lex_token::TokenType;
use super::lex_token::*;
use super::statements::*;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::slice;

pub struct Parser<'a> {
    pub ast: Vec<Box<Statement<'a>>>,
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
        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::EOF => break,
                _ => {
                    let ret = self.declaration();
                    self.ast.push(ret);
                }
            }
        }
    }

    fn declaration(&mut self) -> Box<Statement<'a>> {
        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::Var => {
                    return self.series_var_declaration();
                }
                _ => break,
            }
        }

        self.statement()
    }

    fn series_var_declaration(&mut self) -> Box<Statement<'a>> {
        let mut var_decl = Vec::new();

        var_decl.push(self.var_declaration());

        while let Some(_) = self.iter.peek() {
            if self.check_next_consume(TokenType::Comma) {
                var_decl.push(self.var_declaration());
            } else {
                break;
            }
        }

        if self.check_next(TokenType::Semicolon) || self.check_next(TokenType::Newline) {
            self.iter.next();
        }

        Box::new(Statement::VariableDeclList { var_decl })
    }

    fn var_declaration(&mut self) -> Box<Statement<'a>> {
        self.check_next_consume(TokenType::Var);

        let var_expr = self.primary();

        let assignment = if self.check_next(TokenType::Equal) {
            self.iter.next();
            Some(self.expression())
        } else {
            None
        };

        Box::new(Statement::VariableDecl {
            var_expr,
            assignment,
        })
    }

    fn statement(&mut self) -> Box<Statement<'a>> {
        if let Some((_, token)) = self.iter.peek() {
            match token.token_type {
                TokenType::LeftBrace => return self.block(),
                _ => return self.expression_statement(),
            }
        };
        self.expression_statement()
    }

    fn block(&mut self) -> Box<Statement<'a>> {
              
    }                                               

    fn expression_statement(&mut self) -> Box<Statement<'a>> {
        let expr = self.expression();

        if self.check_next(TokenType::Semicolon) || self.check_next(TokenType::Newline) {
            self.iter.next();
        }
        Box::new(Statement::Expresssion { expression: expr })
    }

    fn expression(&mut self) -> Box<Expr<'a>> {
        self.assignment()
    }

    fn assignment(&mut self) -> Box<Expr<'a>> {
        let expr = self.equality();

        if self.check_next_consume(TokenType::Equal) {
            let assignment_expr = self.assignment();

            Box::new(Expr::Assign {
                left: expr,
                right: assignment_expr,
            })
        } else {
            expr
        }
    }

    fn equality(&mut self) -> Box<Expr<'a>> {
        let expr = self.comparison();

        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::EqualEqual | TokenType::BangEqual => {
                    let (i, _) = self.iter.next().unwrap();
                    let right = self.comparison();

                    return Box::new(Expr::Binary {
                        left: expr,
                        operator: self.tokens[i],
                        right,
                    });
                }
                _ => break,
            };
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr<'a>> {
        let expr = self.addition();

        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    let (i, _) = self.iter.next().unwrap();
                    let right = self.addition();

                    return Box::new(Expr::Binary {
                        left: expr,
                        operator: self.tokens[i],
                        right,
                    });
                }
                _ => break,
            };
        }

        expr
    }

    fn addition(&mut self) -> Box<Expr<'a>> {
        let expr = self.multiplication();

        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::Minus | TokenType::Plus => {
                    let (i, _) = self.iter.next().unwrap();
                    let right = self.multiplication();

                    return Box::new(Expr::Binary {
                        left: expr,
                        operator: self.tokens[i],
                        right,
                    });
                }
                _ => break,
            };
        }

        expr
    }

    fn multiplication(&mut self) -> Box<Expr<'a>> {
        let expr = self.unary();

        while let Some((_, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::Slash | TokenType::Star => {
                    let (i, _) = self.iter.next().unwrap();
                    let right = self.unary();

                    return Box::new(Expr::Binary {
                        left: expr,
                        operator: self.tokens[i],
                        right,
                    });
                }
                _ => break,
            };
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr<'a>> {
        if let Some((_, t)) = self.iter.peek() {
            if let TokenType::Bang | TokenType::Minus = t.token_type {
                let (i, _) = self.iter.next().unwrap();
                let right = self.unary();

                return Box::new(Expr::Unary {
                    operator: self.tokens[i],
                    right,
                });
            }
        };

        self.primary()
    }

    fn primary(&mut self) -> Box<Expr<'a>> {
        if let Some((_i, t)) = self.iter.peek() {
            match t.token_type {
                TokenType::False | TokenType::True => {
                    let (_, t) = self.iter.next().unwrap();
                    return Box::new(Expr::Literal { literal_token: *t });
                }
                TokenType::Number(_) | TokenType::String(_) => {
                    let (_, t) = self.iter.next().unwrap();
                    return Box::new(Expr::Literal { literal_token: *t });
                }
                TokenType::Identifier(_) => {
                    let (_, t) = self.iter.next().unwrap();
                    return Box::new(Expr::Identifier { name: *t });
                }
                TokenType::LeftParen => {
                    let (_, _) = self.iter.next().unwrap();
                    let expression = self.expression();

                    if let Some((_, t)) = self.iter.peek() {
                        if t.token_type == TokenType::RightParen {
                            self.iter.next();
                        }
                    }

                    return Box::new(Expr::Grouping { expression });
                }

                _ => {
                    let (_, t) = self.iter.next().unwrap();
                    return Box::new(Expr::UnidentifiedAsLiteral { literal_token: *t });
                }
            }
        }

        Box::new(Expr::UnexpectedEnd)
    }

    fn check_next(&mut self, token_type: TokenType) -> bool {
        if let Some((_i, t)) = self.iter.peek() {
            return t.token_type == token_type;
        }

        false
    }

    fn check_next_consume(&mut self, token_type: TokenType) -> bool {
        if self.check_next(token_type) {
            self.iter.next().unwrap();
            true
        } else {
            false
        }
    }
}

// #[cfg(test)]
// mod scanner_test {
//     use super::*;

//     #[test]
//     fn
// }
