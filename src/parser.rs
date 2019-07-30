use super::expressions::*;
use super::lex_token::TokenType;
use super::lex_token::*;
use super::statements::*;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::slice;

/*
    TODO:
    - Figure out handling Newlines and comments...
    - Dot Operator
    - Indexing
        -- Standard Indexing
        -- @Indexing
        -- Map Indexing
        -- List Indexing
        -- Grid Indexing
    - Control
        -- Switch Statements
            - switch
            - case
            - default case
        -- Ternary Operators
        -- Break
    - Excess:
        -- Binary Operators & | ^
        -- LogicalAliasing ^^ And Not Or
        -- Mod Div
        -- %
*/



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

        if self.check_next(TokenType::Semicolon) {
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
                TokenType::If => {
                    self.iter.next();
                    return self.if_statement();
                }
                TokenType::Return => {
                    self.iter.next();
                    return self.return_statement();
                }
                TokenType::While => {
                    self.iter.next();
                    return self.while_statement();
                }
                TokenType::Repeat => {
                    self.iter.next();
                    return self.repeat_statement();
                }
                TokenType::For => {
                    self.iter.next();
                    return self.for_statement();
                }
                TokenType::LeftBrace => {
                    self.iter.next();
                    return self.block();
                }
                _ => return self.expression_statement(),
            }
        };
        self.expression_statement()
    }

    fn block(&mut self) -> Box<Statement<'a>> {
        let mut statements = Vec::new();

        while let Some((_i, token)) = self.iter.peek() {
            if token.token_type == TokenType::RightBrace {
                self.iter.next();
                break;
            } else {
                statements.push(self.declaration());
            }
        }

        Box::new(Statement::Block { statements })
    }

    fn if_statement(&mut self) -> Box<Statement<'a>> {
        self.check_next_consume(TokenType::LeftParen);

        let condition = self.expression();

        self.check_next_consume(TokenType::RightParen);

        let then_branch = self.statement();
        let else_branch = if self.check_next(TokenType::Else) {
            Some(self.statement())
        } else {
            None
        };

        Box::new(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Box<Statement<'a>> {
        self.check_next_consume(TokenType::LeftParen);

        let condition = self.expression();

        self.check_next_consume(TokenType::RightParen);

        let body = self.statement();

        Box::new(Statement::While { condition, body })
    }

    fn repeat_statement(&mut self) -> Box<Statement<'a>> {
        self.check_next_consume(TokenType::LeftParen);

        let condition = self.expression();

        self.check_next_consume(TokenType::RightParen);

        let body = self.statement();

        Box::new(Statement::Repeat { condition, body })
    }

    fn for_statement(&mut self) -> Box<Statement<'a>> {
        self.check_next_consume(TokenType::LeftParen);

        let initializer = if self.check_next_consume(TokenType::Semicolon) {
            None
        } else if self.check_next(TokenType::Var) {
            Some(self.series_var_declaration())
        } else {
            Some(self.expression_statement())
        };

        let condition = if self.check_next_consume(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression())
        };

        self.check_next_consume(TokenType::Semicolon);

        let increment = if self.check_next(TokenType::RightParen) {
            None
        } else {
            Some(self.expression())
        };

        self.check_next_consume(TokenType::RightParen);

        let body = self.statement();

        Box::new(Statement::For {
            initializer,
            condition,
            increment,
            body,
        })
    }

    fn return_statement(&mut self) -> Box<Statement<'a>> {
        let expression = if self.check_next(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression())
        };

        self.check_next_consume(TokenType::Semicolon);

        Box::new(Statement::Return { expression })
    }

    fn expression_statement(&mut self) -> Box<Statement<'a>> {
        let expr = self.expression();

        if self.check_next(TokenType::Semicolon) {
            self.iter.next();
        }
        Box::new(Statement::Expresssion { expression: expr })
    }

    fn expression(&mut self) -> Box<Expr<'a>> {
        self.assignment()
    }

    fn assignment(&mut self) -> Box<Expr<'a>> {
        let expr = self.or();

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

    // parse our Logical Operands here
    fn or(&mut self) -> Box<Expr<'a>> {
        let left = self.and();

        if self.check_next(TokenType::LogicalOr) || self.check_next(TokenType::OrAlias) {
            let (_, token) = self.iter.next().unwrap();

            let right = self.equality();

            Box::new(Expr::Logical {
                left,
                operator: *token,
                right,
            })
        } else {
            left
        }
    }

    fn and(&mut self) -> Box<Expr<'a>> {
        let left = self.equality();

        if self.check_next(TokenType::LogicalAnd) || self.check_next(TokenType::AndAlias) {
            let (_, token) = self.iter.next().unwrap();

            let right = self.equality();

            Box::new(Expr::Logical {
                left,
                operator: *token,
                right,
            })
        } else {
            left
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

        self.call()
    }

    fn call(&mut self) -> Box<Expr<'a>> {
        let expression = self.primary();

        loop {
            if self.check_next_consume(TokenType::LeftParen) {
                let arguments = self.finish_call();
                return Box::new(Expr::Call {
                    procedure_name: expression,
                    arguments,
                });
            } else {
                break;
            }
        }

        expression
    }

    fn finish_call(&mut self) -> Vec<Box<Expr<'a>>> {
        let mut arguments = Vec::new();

        if self.check_next(TokenType::RightParen) == false {
            loop {
                arguments.push(self.expression());
                if self.check_next_consume(TokenType::Comma) == false {
                    break;
                }
            }
        };

        self.check_next_consume(TokenType::RightParen);

        arguments
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

                TokenType::Newline => {
                    let (_, t) = self.iter.next().unwrap();
                    return Box::new(Expr::Newline { token: *t });
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

    #[allow(dead_code)]
    fn eat_all_tokens(&mut self, token_type: TokenType) {
        while let Some((_, token)) = self.iter.peek() {
            if token.token_type == token_type {
                self.iter.next();
            } else {
                break;
            }
        }
    }
}
