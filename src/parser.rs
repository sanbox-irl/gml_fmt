use super::expressions::*;
use super::lex_token::TokenType;
use super::lex_token::*;
use super::statements::*;
use std::iter::Peekable;
use std::slice;

pub struct Parser<'a> {
    pub ast: Vec<StmtBox<'a>>,
    iter: Peekable<slice::Iter<'a, Token<'a>>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            ast: Vec::new(),
            iter: tokens.iter().peekable(),
        }
    }

    pub fn build_ast(&mut self) {
        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::EOF => {
                    self.ast.push(StatementWrapper::new(Statement::EOF, false));
                    break;
                }
                _ => {
                    let ret = self.statement();
                    self.ast.push(ret);
                }
            }
        }
    }

    // @jack support compiler directive...#region, #macro #endregion
    fn statement(&mut self) -> StmtBox<'a> {
        if let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::Comment(_) => {
                    let comment = self.consume_next();
                    return StatementWrapper::new(Statement::Comment { comment: *comment }, false);
                }
                TokenType::MultilineComment(_) => {
                    let multiline_comment = self.consume_next();
                    return StatementWrapper::new(
                        Statement::MultilineComment {
                            multiline_comment: *multiline_comment,
                        },
                        false,
                    );
                }
                TokenType::RegionBegin => {
                    self.consume_next();
                    return StatementWrapper::new(
                        Statement::RegionBegin {
                            multi_word_name: self.get_remaining_tokens_on_line(),
                        },
                        false,
                    );
                }
                TokenType::RegionEnd => {
                    self.consume_next();
                    return StatementWrapper::new(
                        Statement::RegionEnd {
                            multi_word_name: self.get_remaining_tokens_on_line(),
                        },
                        false,
                    );
                }
                TokenType::Macro => {
                    self.consume_next();
                    return self.macro_statement();
                }
                TokenType::Define => {
                    self.consume_next();
                    return self.define_statement();
                }
                TokenType::Var => {
                    return self.series_var_declaration();
                }
                TokenType::Enum => {
                    self.consume_next();
                    return self.enum_declaration();
                }
                TokenType::If => {
                    self.consume_next();
                    return self.if_statement();
                }
                TokenType::Return => {
                    self.consume_next();
                    return self.return_statement();
                }
                TokenType::Break => {
                    self.consume_next();
                    return self.break_statement();
                }
                TokenType::Exit => {
                    self.consume_next();
                    return self.exit_statment();
                }
                TokenType::Do => {
                    self.consume_next();
                    return self.do_until_statement();
                }
                TokenType::While => {
                    self.consume_next();
                    return self.while_statement();
                }
                TokenType::Switch => {
                    self.consume_next();
                    return self.switch_statement();
                }
                TokenType::Repeat => {
                    self.consume_next();
                    return self.repeat_statement();
                }
                TokenType::For => {
                    self.consume_next();
                    return self.for_statement();
                }
                TokenType::LeftBrace => {
                    self.consume_next();
                    return self.block();
                }
                _ => return self.expression_statement(),
            }
        };
        self.expression_statement()
    }

    fn get_remaining_tokens_on_line(&mut self) -> Vec<Token<'a>> {
        let mut multi_word_name = vec![];

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::Newline => break,
                TokenType::EOF => break,
                _ => {
                    multi_word_name.push(*self.consume_next());
                }
            }
        }

        multi_word_name
    }

    fn macro_statement(&mut self) -> StmtBox<'a> {
        let mut macro_body = vec![];
        let mut ignore_newline = false;

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::Newline => {
                    if ignore_newline {
                        macro_body.push(*self.consume_next());
                    } else {
                        break;
                    }
                }

                TokenType::Backslash => {
                    macro_body.push(*self.consume_next());
                    ignore_newline = true;
                }

                TokenType::EOF => break,
                _ => {
                    ignore_newline = false;
                    macro_body.push(*self.consume_next());
                }
            }
        }

        StatementWrapper::new(Statement::Macro { macro_body }, false)
    }

    fn define_statement(&mut self) -> StmtBox<'a> {
        let script_name = self.expression();
        let mut body = vec![];

        while let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::EOF | TokenType::Define => {
                    break;
                }

                _ => {
                    body.push(self.statement());
                }
            }
        }

        StatementWrapper::new(Statement::Define { script_name, body }, false)
    }

    fn series_var_declaration(&mut self) -> StmtBox<'a> {
        self.check_next_consume(TokenType::Var);

        let mut var_decl = Vec::new();
        var_decl.push(self.var_declaration());

        while let Some(_) = self.iter.peek() {
            if self.check_next_consume(TokenType::Comma) {
                var_decl.push(self.var_declaration());
            } else {
                break;
            }
        }

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(Statement::VariableDeclList { var_decl }, has_semicolon)
    }

    fn var_declaration(&mut self) -> VariableDecl<'a> {
        let say_var = self.check_next_consume(TokenType::Var);

        let var_expr = self.primary();

        let assignment = if self.check_next(TokenType::Equal) {
            self.iter.next();
            let comments = self.get_newlines_and_comments();
            Some((comments, self.expression()))
        } else {
            None
        };

        VariableDecl {
            var_expr,
            assignment,
            say_var,
        }
    }

    fn block(&mut self) -> StmtBox<'a> {
        let mut statements = Vec::new();

        while let Some(_) = self.iter.peek() {
            if self.check_next_consume(TokenType::RightBrace) {
                break;
            } else {
                statements.push(self.statement());
            }
        }

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(Statement::Block { statements }, has_semicolon)
    }

    fn if_statement(&mut self) -> StmtBox<'a> {
        let mut has_surrounding_paren = (self.check_next_consume(TokenType::LeftParen), false);

        let condition = self.expression();
        has_surrounding_paren.1 = self.check_next_consume(TokenType::RightParen);

        let then_branch = self.statement();
        let else_branch = if self.check_next_consume(TokenType::Else) {
            Some(self.statement())
        } else {
            None
        };

        StatementWrapper::new(
            Statement::If {
                condition,
                has_surrounding_paren,
                then_branch,
                else_branch,
            },
            false,
        )
    }

    fn while_statement(&mut self) -> StmtBox<'a> {
        let mut has_surrounding_paren = (self.check_next_consume(TokenType::LeftParen), false);
        let condition = self.expression();
        has_surrounding_paren.1 = self.check_next_consume(TokenType::RightParen);

        let body = self.statement();

        StatementWrapper::new(
            Statement::While {
                condition,
                body,
                has_surrounding_paren,
            },
            false,
        )
    }

    fn do_until_statement(&mut self) -> StmtBox<'a> {
        let body = self.statement();

        let mut has_surrounding_paren = (self.check_next_consume(TokenType::LeftParen), false);
        self.check_next_consume(TokenType::Until);
        let condition = self.expression();
        has_surrounding_paren.1 = self.check_next_consume(TokenType::RightParen);

        StatementWrapper::new(
            Statement::DoUntil {
                condition,
                body,
                has_surrounding_paren,
            },
            false,
        )
    }

    fn switch_statement(&mut self) -> StmtBox<'a> {
        let mut has_surrounding_paren = (self.check_next_consume(TokenType::LeftParen), false);

        let condition = self.expression();
        has_surrounding_paren.1 = self.check_next_consume(TokenType::RightParen);

        self.check_next_consume(TokenType::LeftBrace);

        self.eat_all_newlines();

        let mut cases: Option<Vec<Case<'a>>> = None;
        let mut default: Option<Vec<Case<'a>>> = None;

        while let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::Case => match &mut cases {
                    Some(vec) => vec.push(self.case_statement()),
                    None => cases = Some(vec![self.case_statement()]),
                },

                // @jack this is borked. default case can't have a constant in it.
                TokenType::DefaultCase => match &mut default {
                    Some(vec) => vec.push(self.case_statement()),
                    None => default = Some(vec![self.case_statement()]),
                },
                _ => break,
            }
        }

        self.eat_all_newlines();
        self.check_next_consume(TokenType::RightBrace);

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(
            Statement::Switch {
                cases,
                has_surrounding_paren,
                condition,
                default,
            },
            has_semicolon,
        )
    }

    fn case_statement(&mut self) -> Case<'a> {
        if self.check_next(TokenType::Case) || self.check_next(TokenType::DefaultCase) {
            self.consume_next();
        }

        let constant = self.expression();
        self.check_next_consume(TokenType::Colon);

        let mut statements = Vec::new();

        while let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::DefaultCase | TokenType::Case => {
                    break;
                }
                TokenType::RightBrace => {
                    break;
                }
                _ => {
                    statements.push(self.statement());
                }
            }
        }

        Case { statements, constant }
    }

    fn repeat_statement(&mut self) -> StmtBox<'a> {
        let mut has_surrounding_paren = (self.check_next_consume(TokenType::LeftParen), false);
        let condition = self.expression();
        has_surrounding_paren.1 = self.check_next_consume(TokenType::RightParen);

        let body = self.statement();

        StatementWrapper::new(
            Statement::Repeat {
                condition,
                body,
                has_surrounding_paren,
            },
            false,
        )
    }

    fn for_statement(&mut self) -> StmtBox<'a> {
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

        StatementWrapper::new(
            Statement::For {
                initializer,
                condition,
                increment,
                body,
            },
            false,
        )
    }

    fn return_statement(&mut self) -> StmtBox<'a> {
        let expression = if self.check_next(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression())
        };

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        StatementWrapper::new(Statement::Return { expression }, has_semicolon)
    }

    fn break_statement(&mut self) -> StmtBox<'a> {
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        StatementWrapper::new(Statement::Break, has_semicolon)
    }

    fn exit_statment(&mut self) -> StmtBox<'a> {
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        StatementWrapper::new(Statement::Exit, has_semicolon)
    }

    fn enum_declaration(&mut self) -> StmtBox<'a> {
        let name = self.expression();

        self.check_next_consume(TokenType::LeftBrace);
        self.eat_all_newlines();

        let mut members = Vec::new();

        while let Some(token) = self.iter.peek() {
            if let TokenType::Identifier(_) = token.token_type {
                let name = self.iter.next().unwrap();

                let value = if self.check_next_consume(TokenType::Equal) {
                    Some(self.expression())
                } else {
                    None
                };

                members.push(EnumMemberDecl { name: *name, value });

                self.check_next_consume(TokenType::Comma);
                self.eat_all_newlines();
            } else {
                break;
            }
        }

        self.check_next_consume(TokenType::RightBrace);
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(Statement::EnumDeclaration { name, members }, has_semicolon)
    }

    fn expression_statement(&mut self) -> StmtBox<'a> {
        let expr = self.expression();

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        StatementWrapper::new(Statement::ExpresssionStatement { expression: expr }, has_semicolon)
    }

    fn expression(&mut self) -> ExprBox<'a> {
        self.assignment()
    }

    fn assignment(&mut self) -> ExprBox<'a> {
        let mut expr = self.ternary();

        if let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::Equal
                | TokenType::PlusEquals
                | TokenType::MinusEquals
                | TokenType::StarEquals
                | TokenType::SlashEquals
                | TokenType::BitXorEquals
                | TokenType::BitOrEquals
                | TokenType::BitAndEquals
                | TokenType::ModEquals => {
                    let operator = self.iter.next().unwrap();
                    let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                    let assignment_expr = self.assignment();

                    expr = self.create_expr_box_no_comment(Expr::Assign {
                        left: expr,
                        operator: *operator,
                        comments_and_newlines_between_op_and_r,
                        right: assignment_expr,
                    });
                }

                _ => {}
            }
        }

        expr
    }

    fn ternary(&mut self) -> ExprBox<'a> {
        let mut expr = self.or();

        if self.check_next_consume(TokenType::Hook) {
            let comments_and_newlines_after_q = self.get_newlines_and_comments();
            let left = self.ternary();
            self.check_next_consume(TokenType::Colon);
            let comments_and_newlines_after_colon = self.get_newlines_and_comments();
            let right = self.ternary();

            expr = self.create_expr_box_no_comment(Expr::Ternary {
                conditional: expr,
                comments_and_newlines_after_q,
                left,
                comments_and_newlines_after_colon,
                right,
            });
        }

        expr
    }

    // parse our Logical Operands here
    fn or(&mut self) -> ExprBox<'a> {
        let mut left = self.and();

        if self.check_next(TokenType::LogicalOr) || self.check_next(TokenType::OrAlias) {
            let token = self.iter.next().unwrap();
            let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
            let right = self.equality();

            left = self.create_expr_box_no_comment(Expr::Logical {
                left,
                operator: *token,
                comments_and_newlines_between_op_and_r,
                right,
            });
        }

        left
    }

    fn and(&mut self) -> ExprBox<'a> {
        let mut left = self.xor();

        if self.check_next_either(TokenType::LogicalAnd, TokenType::AndAlias) {
            let token = self.iter.next().unwrap();
            let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
            let right = self.xor();

            left = self.create_expr_box_no_comment(Expr::Logical {
                left,
                operator: *token,
                comments_and_newlines_between_op_and_r,
                right,
            });
        }
        left
    }

    fn xor(&mut self) -> ExprBox<'a> {
        let mut left = self.equality();

        if self.check_next_either(TokenType::LogicalXor, TokenType::XorAlias) {
            let token = self.iter.next().unwrap();
            let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
            let right = self.equality();

            left = self.create_expr_box_no_comment(Expr::Logical {
                left,
                operator: *token,
                comments_and_newlines_between_op_and_r,
                right,
            })
        }

        left
    }

    fn equality(&mut self) -> ExprBox<'a> {
        let mut expr = self.comparison();

        while let Some(t) = self.iter.peek() {
            if t.token_type == TokenType::EqualEqual || t.token_type == TokenType::BangEqual {
                let comments_and_newlines_between_l_and_op = self.get_newlines_and_comments();
                let token = self.iter.next().unwrap();
                let comments_and_newlines_between_r_and_op = self.get_newlines_and_comments();
                let right = self.comparison();

                expr = self.create_expr_box_no_comment(Expr::Binary {
                    left: expr,
                    comments_and_newlines_between_l_and_op,
                    operator: *token,
                    comments_and_newlines_between_r_and_op,
                    right,
                });
            } else {
                break;
            }
        }

        expr
    }

    fn comparison(&mut self) -> ExprBox<'a> {
        let mut expr = self.binary();

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
                    let comments_and_newlines_between_l_and_op = self.get_newlines_and_comments();
                    let t = self.iter.next().unwrap();
                    let comments_and_newlines_between_r_and_op = self.get_newlines_and_comments();
                    let right = self.binary();

                    expr = self.create_expr_box_no_comment(Expr::Binary {
                        left: expr,
                        comments_and_newlines_between_l_and_op,
                        operator: *t,
                        comments_and_newlines_between_r_and_op,
                        right,
                    });
                }
                _ => break,
            };
        }

        expr
    }

    fn binary(&mut self) -> ExprBox<'a> {
        let mut expr = self.bitshift();

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::BitAnd | TokenType::BitOr | TokenType::BitXor => {
                    let comments_and_newlines_between_l_and_op = self.get_newlines_and_comments();
                    let t = self.iter.next().unwrap();
                    let comments_and_newlines_between_r_and_op = self.get_newlines_and_comments();
                    let right = self.bitshift();

                    expr = self.create_expr_box_no_comment(Expr::Binary {
                        left: expr,
                        comments_and_newlines_between_l_and_op,
                        operator: *t,
                        comments_and_newlines_between_r_and_op,
                        right,
                    });
                }
                _ => break,
            }
        }

        expr
    }

    fn bitshift(&mut self) -> ExprBox<'a> {
        let mut expr = self.addition();

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::BitLeft | TokenType::BitRight => {
                    let comments_and_newlines_between_l_and_op = self.get_newlines_and_comments();
                    let t = self.iter.next().unwrap();
                    let comments_and_newlines_between_r_and_op = self.get_newlines_and_comments();
                    let right = self.addition();

                    expr = self.create_expr_box_no_comment(Expr::Binary {
                        left: expr,
                        comments_and_newlines_between_l_and_op,
                        operator: *t,
                        comments_and_newlines_between_r_and_op,
                        right,
                    });
                }
                _ => break,
            }
        }

        expr
    }

    fn addition(&mut self) -> ExprBox<'a> {
        let mut expr = self.multiplication();

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::Minus | TokenType::Plus => {
                    let comments_and_newlines_between_l_and_op = self.get_newlines_and_comments();
                    let token = self.iter.next().unwrap();
                    let comments_and_newlines_between_r_and_op = self.get_newlines_and_comments();
                    let right = self.multiplication();

                    expr = self.create_expr_box_no_comment(Expr::Binary {
                        left: expr,
                        comments_and_newlines_between_l_and_op,
                        operator: *token,
                        comments_and_newlines_between_r_and_op,
                        right,
                    });
                }
                _ => break,
            };
        }

        expr
    }

    fn multiplication(&mut self) -> ExprBox<'a> {
        let mut expr = self.unary();

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::Slash | TokenType::Star | TokenType::Mod | TokenType::ModAlias | TokenType::Div => {
                    let comments_and_newlines_between_l_and_op = self.get_newlines_and_comments();
                    let token = self.iter.next().unwrap();
                    let comments_and_newlines_between_r_and_op = self.get_newlines_and_comments();
                    let right = self.unary();

                    expr = self.create_expr_box_no_comment(Expr::Binary {
                        left: expr,
                        comments_and_newlines_between_l_and_op,
                        operator: *token,
                        comments_and_newlines_between_r_and_op,
                        right,
                    });
                }
                _ => break,
            };
        }

        expr
    }

    fn unary(&mut self) -> ExprBox<'a> {
        if let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::Bang | TokenType::Minus | TokenType::Plus => {
                    let t = self.iter.next().unwrap();
                    let right = self.unary();

                    return self.create_expr_box_no_comment(Expr::Unary { operator: *t, right });
                }

                TokenType::Incrementer | TokenType::Decrementer => {
                    let t = self.iter.next().unwrap();
                    let expr = self.unary();

                    return self.create_expr_box_no_comment(Expr::Prefix { operator: *t, expr });
                }

                _ => {}
            }
        }

        self.postfix()
    }

    fn postfix(&mut self) -> ExprBox<'a> {
        let mut expr = self.call();

        if self.check_next_either(TokenType::Incrementer, TokenType::Decrementer) {
            let t = self.iter.next().unwrap();
            expr = self.create_expr_box_no_comment(Expr::Postfix { operator: *t, expr });
        }

        expr
    }

    fn call(&mut self) -> ExprBox<'a> {
        let mut expression = self.primary();

        if self.check_next_consume(TokenType::LeftParen) {
            let comments_and_newlines_after_lparen = self.get_newlines_and_comments();
            let arguments = self.finish_call(TokenType::RightParen, TokenType::Comma);

            expression = self.create_expr_box_no_comment(Expr::Call {
                procedure_name: expression,
                arguments,
                comments_and_newlines_after_lparen,
            });
        }

        while let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::Dot => {
                    self.consume_next();
                    if let Some(t) = self.iter.peek() {
                        if let TokenType::Identifier(_) = t.token_type {
                            let instance_variable = self.expression();
                            expression = self.create_expr_box_no_comment(Expr::DotAccess {
                                object_name: expression,
                                instance_variable,
                            });
                        }
                    }
                }

                TokenType::LeftBracket | TokenType::ArrayIndexer | TokenType::MapIndexer | TokenType::ListIndexer => {
                    let token = self.iter.next().unwrap();
                    let comments_and_newlines_between_access_and_expr = self.get_newlines_and_comments();
                    let access_expr = self.expression();

                    self.check_next_consume(TokenType::RightBracket);

                    // stupid non-chained access..
                    return self.create_expr_box_no_comment(Expr::DataStructureAccess {
                        ds_name: expression,
                        comments_and_newlines_between_access_and_expr,
                        access_type: *token,
                        access_expr,
                    });
                }

                TokenType::GridIndexer => {
                    let token = self.iter.next().unwrap();
                    let comments_and_newlines_between_access_type_and_row_expr = self.get_newlines_and_comments();
                    let row_expr = self.expression();
                    self.check_next_consume(TokenType::Comma);
                    let comments_and_newlines_after_comma = self.get_newlines_and_comments();
                    let column_expr = self.expression();
                    self.check_next_consume(TokenType::RightBracket);

                    // stupid non-chained access..
                    return self.create_expr_box_no_comment(Expr::GridDataStructureAccess {
                        ds_name: expression,
                        access_type: *token,
                        column_expr,
                        row_expr,
                        comments_and_newlines_between_access_type_and_row_expr,
                        comments_and_newlines_after_comma,
                    });
                }

                _ => break,
            }
        }

        expression
    }

    fn finish_call(&mut self, end_token_type: TokenType, delimiter_type: TokenType) -> Arguments<'a> {
        let mut arguments = Vec::new();
        if self.check_next(end_token_type) == false {
            loop {
                if self.check_next(end_token_type) {
                    break;
                }

                arguments.push((
                    self.get_newlines_and_comments(),
                    self.expression(),
                    self.get_newlines_and_comments(),
                ));

                if self.check_next_consume(delimiter_type) == false {
                    break;
                }
            }
        };

        self.check_next_consume(end_token_type);

        arguments
    }

    fn primary(&mut self) -> ExprBox<'a> {
        if let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::False | TokenType::True => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    return self.create_comment_expr_box(Expr::Literal {
                        literal_token: *t,
                        comments,
                    });
                }
                TokenType::Number(_) | TokenType::String(_) => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    return self.create_comment_expr_box(Expr::Literal {
                        literal_token: *t,
                        comments,
                    });
                }
                TokenType::NumberStartDot(_) => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    return self.create_comment_expr_box(Expr::NumberStartDot {
                        literal_token: *t,
                        comments,
                    });
                }
                TokenType::NumberEndDot(_) => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    return self.create_comment_expr_box(Expr::NumberEndDot {
                        literal_token: *t,
                        comments,
                    });
                }
                TokenType::Identifier(_) => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    return self.create_comment_expr_box(Expr::Identifier { name: *t, comments });
                }
                TokenType::LeftParen => {
                    self.consume_next();
                    let comments_and_newlines_after_lparen = self.get_newlines_and_comments();

                    let mut expressions = vec![];
                    expressions.push(self.expression());
                    while self.check_next_consume(TokenType::RightParen) == false {
                        expressions.push(self.expression());
                    }

                    let comments_and_newlines_before_rparen = self.get_newlines_and_comments();

                    return self.create_comment_expr_box(Expr::Grouping {
                        expressions,
                        comments_and_newlines_after_lparen,
                        comments_and_newlines_before_rparen,
                    });
                }

                TokenType::LeftBracket => {
                    self.consume_next();
                    let comments_and_newlines_after_lbracket = self.get_newlines_and_comments();
                    let arguments = self.finish_call(TokenType::RightBracket, TokenType::Comma);

                    return self.create_expr_box_no_comment(Expr::ArrayLiteral {
                        comments_and_newlines_after_lbracket,
                        arguments,
                    });
                }

                TokenType::Newline => {
                    let t = self.consume_next();
                    return self.create_expr_box_no_comment(Expr::Newline { token: *t });
                }
                _ => {
                    let t = self.consume_next();
                    return self.create_comment_expr_box(Expr::UnidentifiedAsLiteral { literal_token: *t });
                }
            }
        }

        self.create_expr_box_no_comment(Expr::UnexpectedEnd)
    }

    fn check_next(&mut self, token_type: TokenType) -> bool {
        if let Some(t) = self.iter.peek() {
            return t.token_type == token_type;
        }

        false
    }

    fn check_next_either(&mut self, token_type1: TokenType, token_type2: TokenType) -> bool {
        if let Some(t) = self.iter.peek() {
            return t.token_type == token_type1 || t.token_type == token_type2;
        }

        false
    }

    fn check_next_consume(&mut self, token_type: TokenType) -> bool {
        if self.check_next(token_type) {
            self.consume_next();
            true
        } else {
            false
        }
    }
    fn get_newlines_and_comments(&mut self) -> Vec<Token<'a>> {
        let mut vec = vec![];
        while let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::Newline => {
                    let token = self.iter.next().unwrap();
                    vec.push(*token);
                }
                TokenType::Comment(_) | TokenType::MultilineComment(_) => {
                    let token = self.iter.next().unwrap();
                    vec.push(*token);
                }

                _ => break,
            }
        }

        vec
    }

    fn eat_all_newlines(&mut self) {
        while let Some(token) = self.iter.peek() {
            if token.token_type == TokenType::Newline {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    fn consume_next(&mut self) -> &'a Token<'a> {
        self.iter.next().unwrap()
    }

    fn create_comment_expr_box(&mut self, expr: Expr<'a>) -> ExprBox<'a> {
        Box::new((expr, self.get_newlines_and_comments()))
    }

    fn create_expr_box_no_comment(&self, expr: Expr<'a>) -> ExprBox<'a> {
        Box::new((expr, vec![]))
    }
}
