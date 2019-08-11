use super::expressions::*;
use super::lex_token::TokenType;
use super::lex_token::*;
use super::statements::*;
use std::iter::Peekable;
use std::slice;

pub struct Parser<'a> {
    pub ast: Vec<StmtBox<'a>>,
    pub failure: Option<String>,
    allow_unidentified: bool,
    iter: Peekable<slice::Iter<'a, Token<'a>>>,
    can_pair: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Parser<'a> {
        Parser {
            ast: Vec::new(),
            iter: tokens.iter().peekable(),
            failure: None,
            allow_unidentified: false,
            can_pair: true,
        }
    }

    pub fn build_ast(&mut self) {
        while let Some(t) = self.iter.peek() {
            if let Some(_) = self.failure {
                break;
            }
            self.can_pair = true;
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
                TokenType::While | TokenType::With | TokenType::Repeat => {
                    let token = self.consume_next();
                    return self.while_with_repeat(*token);
                }
                TokenType::Switch => {
                    self.consume_next();
                    return self.switch_statement();
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
                TokenType::Newline(_) => break,
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
                TokenType::Newline(_) => {
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
        let comments_after_control_word = self.get_newlines_and_comments();
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

        StatementWrapper::new(
            Statement::Define {
                comments_after_control_word,
                script_name,
                body,
            },
            false,
        )
    }

    fn series_var_declaration(&mut self) -> StmtBox<'a> {
        self.check_next_consume(TokenType::Var);
        let comments_after_control_word = self.get_newlines_and_comments();
        let var_decl = self.var_declaration();
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        StatementWrapper::new(
            Statement::VariableDeclList {
                var_decl,
                comments_after_control_word,
            },
            has_semicolon,
        )
    }

    fn var_declaration(&mut self) -> DelimitedLines<'a, VariableDecl<'a>> {
        let mut arguments: Vec<DelimitedLine<'a, VariableDecl<'a>>> = Vec::new();

        let end_delimiter;
        loop {
            if self.check_next(TokenType::Semicolon) {
                end_delimiter = true;
                break;
            }

            let say_var = self.check_next_consume(TokenType::Var);
            let say_var_comments = if say_var {
                Some(self.get_newlines_and_comments())
            } else {
                None
            };

            let var_decl = VariableDecl {
                say_var,
                say_var_comments,
                var_expr: self.expression(),
            };
            let do_break = self.check_next_consume(TokenType::Comma) == false;
            let trailing_comment = self.get_newlines_and_comments();

            arguments.push(DelimitedLine {
                expr: var_decl,
                trailing_comment,
            });

            if do_break {
                end_delimiter = false;
                break;
            }
        }
        DelimitedLines {
            lines: arguments,
            has_end_delimiter: end_delimiter,
        }
    }

    fn block(&mut self) -> StmtBox<'a> {
        let comments_after_lbrace = self.get_newlines_and_comments();

        let mut statements = Vec::new();

        while let Some(_) = self.iter.peek() {
            if self.check_next_consume(TokenType::RightBrace) {
                break;
            } else {
                statements.push(self.statement());
            }
        }

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(
            Statement::Block {
                statements,
                comments_after_lbrace,
            },
            has_semicolon,
        )
    }

    fn if_statement(&mut self) -> StmtBox<'a> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let condition = self.expression();
        let then_branch = self.statement();
        let comments_between = self.get_newlines_and_comments();
        let else_branch = if self.check_next_consume(TokenType::Else) {
            Some(self.statement())
        } else {
            None
        };
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(
            Statement::If {
                comments_after_control_word,
                condition,
                then_branch,
                comments_between,
                else_branch,
            },
            has_semicolon,
        )
    }

    fn while_with_repeat(&mut self, token: Token<'a>) -> StmtBox<'a> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let condition = self.expression();
        let body = self.statement();
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(
            Statement::WhileWithRepeat {
                token,
                condition,
                body,
                comments_after_control_word,
            },
            has_semicolon,
        )
    }

    fn do_until_statement(&mut self) -> StmtBox<'a> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let body = self.statement();
        let comments_between = self.get_newlines_and_comments();
        self.check_next_consume(TokenType::Until);
        let condition = self.expression();
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(
            Statement::DoUntil {
                comments_after_control_word,
                comments_between,
                condition,
                body,
            },
            has_semicolon,
        )
    }

    fn switch_statement(&mut self) -> StmtBox<'a> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let condition = self.expression();
        self.check_next_consume(TokenType::LeftBrace);
        let comments_after_lbrace = self.get_newlines_and_comments();

        let mut cases: Vec<Case<'a>> = vec![];

        while let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::Case => {
                    self.consume_next();
                    let comments_after_control_word = self.get_newlines_and_comments();
                    let constant = self.expression();
                    self.check_next_consume(TokenType::Colon);
                    let comments_after_colon = self.get_newlines_and_comments();

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

                    cases.push(Case {
                        comments_after_control_word,
                        control_word: CaseType::Case(constant),
                        comments_after_colon,
                        statements,
                    });
                }

                TokenType::DefaultCase => {
                    self.consume_next();
                    let comments_after_control_word = self.get_newlines_and_comments();
                    self.check_next_consume(TokenType::Colon);
                    let comments_after_colon = self.get_newlines_and_comments();

                    let mut statements = Vec::new();
                    while let Some(token) = self.iter.peek() {
                        match token.token_type {
                            TokenType::DefaultCase | TokenType::Case | TokenType::RightBrace => {
                                break;
                            }
                            _ => {
                                statements.push(self.statement());
                            }
                        }
                    }

                    cases.push(Case {
                        comments_after_control_word,
                        control_word: CaseType::Default,
                        comments_after_colon,
                        statements,
                    });
                }

                TokenType::RightBrace => break,

                _ => {
                    self.failure = Some(format!("Unknown token {} in Switch statement", token));
                    break;
                }
            }
        }

        self.check_next_consume(TokenType::RightBrace);

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(
            Statement::Switch {
                comments_after_control_word,
                comments_after_lbrace,
                cases,
                condition,
            },
            has_semicolon,
        )
    }

    fn for_statement(&mut self) -> StmtBox<'a> {
        let comments_after_control_word = self.get_newlines_and_comments();

        self.check_next_consume(TokenType::LeftParen);
        let comments_after_lparen = self.get_newlines_and_comments();

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
        let trailing_comments = self.get_newlines_and_comments();

        let body = self.statement();
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(
            Statement::For {
                comments_after_control_word,
                comments_after_lparen,
                initializer,
                condition,
                increment,
                body,
                trailing_comments,
            },
            has_semicolon,
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
        let comments_after_control_word = self.get_newlines_and_comments();
        let name = self.expression();

        self.check_next_consume(TokenType::LeftBrace);
        let comments_after_lbrace = self.get_newlines_and_comments();
        let members = self.finish_call(TokenType::RightBrace, TokenType::Comma);
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        StatementWrapper::new(
            Statement::EnumDeclaration {
                comments_after_control_word,
                name,
                comments_after_lbrace,
                members,
            },
            has_semicolon,
        )
    }

    fn expression_statement(&mut self) -> StmtBox<'a> {
        let expr = self.expression();
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        StatementWrapper::new(Statement::ExpresssionStatement { expression: expr }, has_semicolon)
    }

    fn expression(&mut self) -> ExprBox<'a> {
        self.allow_unidentified = true;
        let ret = self.assignment();
        self.can_pair = true;
        self.allow_unidentified = false;
        ret
    }

    fn assignment(&mut self) -> ExprBox<'a> {
        let mut expr = self.ternary();

        if self.can_pair {
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

        if self.check_next_either(TokenType::LogicalOr, TokenType::OrAlias) {
            let token = self.iter.next().unwrap();
            let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
            let right = self.or();

            left = self.create_expr_box_no_comment(Expr::Binary {
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
            let right = self.and();

            left = self.create_expr_box_no_comment(Expr::Binary {
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
            let right = self.xor();

            left = self.create_expr_box_no_comment(Expr::Binary {
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

        if self.can_pair {
            while let Some(t) = self.iter.peek() {
                if t.token_type == TokenType::EqualEqual || t.token_type == TokenType::BangEqual {
                    let token = self.iter.next().unwrap();
                    let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                    let right = self.comparison();

                    expr = self.create_expr_box_no_comment(Expr::Binary {
                        left: expr,
                        operator: *token,
                        comments_and_newlines_between_op_and_r,
                        right,
                    });
                } else {
                    break;
                }
            }
        }

        expr
    }

    fn comparison(&mut self) -> ExprBox<'a> {
        let mut expr = self.binary();

        if self.can_pair {
            while let Some(t) = self.iter.peek() {
                match t.token_type {
                    TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
                        let t = self.iter.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.binary();

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: *t,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                };
            }
        }

        expr
    }

    fn binary(&mut self) -> ExprBox<'a> {
        let mut expr = self.bitshift();

        if self.can_pair {
            while let Some(t) = self.iter.peek() {
                match t.token_type {
                    TokenType::BitAnd | TokenType::BitOr | TokenType::BitXor => {
                        let t = self.iter.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.bitshift();

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: *t,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                }
            }
        }

        expr
    }

    fn bitshift(&mut self) -> ExprBox<'a> {
        let mut expr = self.addition();

        if self.can_pair {
            while let Some(t) = self.iter.peek() {
                match t.token_type {
                    TokenType::BitLeft | TokenType::BitRight => {
                        let t = self.iter.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.addition();

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: *t,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                }
            }
        }

        expr
    }

    fn addition(&mut self) -> ExprBox<'a> {
        let mut expr = self.multiplication();

        if self.can_pair {
            while let Some(t) = self.iter.peek() {
                match t.token_type {
                    TokenType::Minus | TokenType::Plus => {
                        let token = self.iter.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.multiplication();

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: *token,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                };
            }
        }

        expr
    }

    fn multiplication(&mut self) -> ExprBox<'a> {
        let mut expr = self.unary();

        if self.can_pair {
            while let Some(t) = self.iter.peek() {
                match t.token_type {
                    TokenType::Slash | TokenType::Star | TokenType::Mod | TokenType::ModAlias | TokenType::Div => {
                        let token = self.iter.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.unary();

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: *token,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                };
            }
        }

        expr
    }

    fn unary(&mut self) -> ExprBox<'a> {
        if self.can_pair {
            if let Some(t) = self.iter.peek() {
                match t.token_type {
                    TokenType::Bang | TokenType::Minus | TokenType::Plus => {
                        let t = self.iter.next().unwrap();
                        let comments_and_newlines_between = self.get_newlines_and_comments();
                        let right = self.unary();

                        return self.create_expr_box_no_comment(Expr::Unary {
                            operator: *t,
                            comments_and_newlines_between,
                            right,
                        });
                    }

                    TokenType::Incrementer | TokenType::Decrementer => {
                        let t = self.iter.next().unwrap();
                        let comments_and_newlines_between = self.get_newlines_and_comments();
                        let right = self.unary();

                        return self.create_expr_box_no_comment(Expr::Unary {
                            operator: *t,
                            comments_and_newlines_between,
                            right,
                        });
                    }

                    _ => {}
                }
            }
        }

        self.postfix()
    }

    fn postfix(&mut self) -> ExprBox<'a> {
        let mut expr = self.call();

        if self.check_next_either(TokenType::Incrementer, TokenType::Decrementer) {
            let t = self.iter.next().unwrap();

            let comments_and_newlines_between = self.get_newlines_and_comments();
            expr = self.create_expr_box_no_comment(Expr::Postfix {
                operator: *t,
                comments_and_newlines_between,
                expr,
            });
        }

        expr
    }

    fn call(&mut self) -> ExprBox<'a> {
        let mut expression = self.primary();

        if self.check_next_consume(TokenType::LeftParen) {
            let comments_and_newlines_after_lparen = self.get_newlines_and_comments();
            let arguments = self.finish_call(TokenType::RightParen, TokenType::Comma);

            expression = self.create_comment_expr_box(Expr::Call {
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
                            expression = self.create_comment_expr_box(Expr::DotAccess {
                                object_name: expression,
                                instance_variable,
                            });
                        }
                    }
                }

                TokenType::LeftBracket
                | TokenType::ArrayIndexer
                | TokenType::MapIndexer
                | TokenType::ListIndexer
                | TokenType::GridIndexer => {
                    let access_type = self.iter.next().unwrap();
                    let mut access_exprs = vec![];

                    while let Some(token) = self.iter.peek() {
                        if token.token_type == TokenType::RightBracket {
                            break;
                        }

                        access_exprs.push((self.get_newlines_and_comments(), self.expression()));

                        if self.check_next_consume(TokenType::Comma) == false {
                            break;
                        }
                    }

                    self.check_next_consume(TokenType::RightBracket);
                    expression = self.create_comment_expr_box(Expr::DataStructureAccess {
                        ds_name: expression,
                        access_type: *access_type,
                        access_exprs,
                    });
                }

                _ => break,
            }
        }

        expression
    }

    fn primary(&mut self) -> ExprBox<'a> {
        if let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::Number(_) | TokenType::String(_) => {
                    let t = self.consume_next();
                    let comments = self.get_comments_until_newline();
                    return self.create_expr_box_no_comment(Expr::Literal {
                        literal_token: *t,
                        comments,
                    });
                }
                TokenType::NumberStartDot(_) => {
                    let t = self.consume_next();
                    let comments = self.get_comments_until_newline();
                    return self.create_expr_box_no_comment(Expr::NumberStartDot {
                        literal_token: *t,
                        comments,
                    });
                }
                TokenType::NumberEndDot(_) => {
                    let t = self.consume_next();
                    let comments = self.get_comments_until_newline();
                    return self.create_expr_box_no_comment(Expr::NumberEndDot {
                        literal_token: *t,
                        comments,
                    });
                }
                TokenType::Identifier(_) => {
                    let t = self.consume_next();
                    let comments = self.get_comments_until_newline();
                    return self.create_expr_box_no_comment(Expr::Identifier { name: *t, comments });
                }
                TokenType::LeftParen => {
                    self.consume_next();
                    let comments_and_newlines_after_lparen = self.get_newlines_and_comments();

                    let mut expressions = vec![];
                    expressions.push(self.expression());
                    while self.check_next_consume(TokenType::RightParen) == false {
                        expressions.push(self.expression());
                    }

                    let comments_and_newlines_after_rparen = self.get_newlines_and_comments();

                    return self.create_expr_box_no_comment(Expr::Grouping {
                        expressions,
                        comments_and_newlines_after_lparen,
                        comments_and_newlines_after_rparen,
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

                TokenType::Newline(_) => {
                    self.consume_next();
                    self.can_pair = false;
                    return self.create_expr_box_no_comment(Expr::Newline);
                }
                TokenType::Comment(_) => {
                    let comment = self.consume_next();
                    self.can_pair = false;
                    return self.create_expr_box_no_comment(Expr::Comment { comment: *comment });
                }
                TokenType::MultilineComment(_) => {
                    let multiline_comment = self.consume_next();
                    self.can_pair = false;
                    return self.create_expr_box_no_comment(Expr::MultilineComment {
                        multiline_comment: *multiline_comment,
                    });
                }
                _ => {
                    let t = self.consume_next();
                    if self.allow_unidentified == false {
                        self.failure = Some(format!("Error parsing {}", *t));
                    }

                    return self.create_comment_expr_box(Expr::UnidentifiedAsLiteral { literal_token: *t });
                }
            }
        }

        self.failure = Some("Unexpected end.".to_owned());
        self.create_expr_box_no_comment(Expr::UnexpectedEnd)
    }

    fn finish_call(&mut self, end_token_type: TokenType, delimiter_type: TokenType) -> DelimitedLines<'a, ExprBox<'a>> {
        let mut arguments = Vec::new();

        let mut end_delimiter = true;
        if self.check_next(end_token_type) == false {
            loop {
                if self.check_next(end_token_type) {
                    end_delimiter = true;
                    break;
                }

                let expr = self.expression();
                let do_break = self.check_next_consume(delimiter_type) == false;

                let trailing_comment = self.get_newlines_and_comments();

                arguments.push(DelimitedLine { expr, trailing_comment });

                if do_break {
                    end_delimiter = false;
                    break;
                }
            }
        };
        self.check_next_consume(end_token_type);

        DelimitedLines {
            lines: arguments,
            has_end_delimiter: end_delimiter,
        }
    }

    fn check_next(&mut self, token_type: TokenType) -> bool {
        if self.can_pair == false {
            return false;
        }

        if let Some(t) = self.iter.peek() {
            return t.token_type == token_type;
        }

        false
    }

    fn check_next_either(&mut self, token_type1: TokenType, token_type2: TokenType) -> bool {
        if self.can_pair == false {
            return false;
        }

        if let Some(t) = self.iter.peek() {
            return t.token_type == token_type1 || t.token_type == token_type2;
        }

        false
    }

    fn check_next_consume(&mut self, token_type: TokenType) -> bool {
        if self.can_pair == false {
            return false;
        }

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
                TokenType::Newline(_) => {
                    let token = self.iter.next().unwrap();
                    vec.push(*token);
                }
                TokenType::Comment(_) | TokenType::MultilineComment(_) => {
                    let token = self.iter.next().unwrap();
                    vec.push(*token);
                }

                TokenType::RegionBegin | TokenType::RegionEnd => {
                    vec.push(*self.iter.next().unwrap());
                    vec.append(&mut self.get_remaining_tokens_on_line());
                }
                _ => break,
            }
        }

        vec
    }

    fn get_comments_until_newline(&mut self) -> Vec<Token<'a>> {
        let mut vec = vec![];
        while let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::Comment(_) | TokenType::MultilineComment(_) => {
                    let token = self.iter.next().unwrap();
                    vec.push(*token);
                }

                TokenType::RegionBegin | TokenType::RegionEnd => {
                    vec.push(*self.iter.next().unwrap());
                    vec.append(&mut self.get_remaining_tokens_on_line());
                }
                _ => break,
            }
        }

        vec
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
