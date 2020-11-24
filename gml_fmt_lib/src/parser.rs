use super::expressions::*;
use super::lex_token::TokenType;
use super::lex_token::*;
use super::scanner::Scanner;
use super::statements::*;
use anyhow::Result as AnyResult;
use std::iter::Peekable;

pub struct Parser<'a> {
    pub ast: Vec<StmtBox<'a>>,
    allow_unidentified: bool,
    scanner: Peekable<Scanner<'a>>,
    can_pair: bool,
    leftover_stmts: Vec<StmtBox<'a>>,
    check_leftovers: bool,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            ast: Vec::new(),
            scanner: Scanner::new(input).into_iter().peekable(),
            allow_unidentified: false,
            can_pair: true,
            leftover_stmts: Vec::new(),
            check_leftovers: false,
        }
    }

    pub fn build_ast(mut self) -> AnyResult<Vec<StmtBox<'a>>> {
        while let Some(_) = self.scanner.peek() {
            self.can_pair = true;
            let ret = self.statement()?;
            self.ast.push(ret);

            if self.check_leftovers {
                self.ast.append(&mut self.leftover_stmts);
                self.check_leftovers = false;
            }
        }

        Ok(self.ast)
    }

    fn statement(&mut self) -> AnyResult<StmtBox<'a>> {
        if let Some(token) = self.scanner.peek() {
            match token.token_type {
                TokenType::Comment(_) => {
                    let comment = self.consume_next();
                    return Ok(StatementWrapper::new(Statement::Comment { comment }, false));
                }
                TokenType::MultilineComment(_) => {
                    let multiline_comment = self.consume_next();
                    return Ok(StatementWrapper::new(
                        Statement::MultilineComment { multiline_comment },
                        false,
                    ));
                }
                TokenType::RegionBegin(_) => {
                    let token = self.consume_next();
                    return Ok(StatementWrapper::new(Statement::RegionBegin(token), false));
                }
                TokenType::RegionEnd(_) => {
                    let token = self.consume_next();
                    return Ok(StatementWrapper::new(Statement::RegionEnd(token), false));
                }
                TokenType::Macro(_) => {
                    let token = self.consume_next();
                    return Ok(StatementWrapper::new(Statement::Macro(token), false));
                }
                TokenType::Define => {
                    self.consume_next();
                    return self.define_statement();
                }
                TokenType::Var | TokenType::GlobalVar => {
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
                    return self.while_with_repeat(token);
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

    fn define_statement(&mut self) -> AnyResult<StmtBox<'a>> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let script_name = self.expression()?;
        let mut body = vec![];

        while let Some(token) = self.scanner.peek() {
            match token.token_type {
                TokenType::Define => {
                    break;
                }

                _ => {
                    body.push(self.statement()?);
                }
            }
        }

        Ok(StatementWrapper::new(
            Statement::Define {
                comments_after_control_word,
                script_name,
                body,
            },
            false,
        ))
    }

    fn series_var_declaration(&mut self) -> AnyResult<StmtBox<'a>> {
        let starting_var_type = self.scanner.next().unwrap();
        let comments_after_control_word = self.get_newlines_and_comments();
        let var_decl = self.var_declaration()?;
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::VariableDeclList {
                starting_var_type,
                var_decl,
                comments_after_control_word,
            },
            has_semicolon,
        ))
    }

    fn var_declaration(&mut self) -> AnyResult<DelimitedLines<'a, VariableDecl<'a>>> {
        let mut arguments: Vec<DelimitedLine<'a, VariableDecl<'a>>> = Vec::new();

        let end_delimiter;
        loop {
            if self.check_next(TokenType::Semicolon) {
                end_delimiter = true;
                break;
            }

            let has_var = self.check_next_either(TokenType::Var, TokenType::GlobalVar);

            let mut say_var = None;
            let mut say_var_comments = None;

            if has_var {
                say_var = Some(self.scanner.next().unwrap());
                say_var_comments = Some(self.get_newlines_and_comments());
            }

            // If we've said var, and then had an expression, we deserve suffering.
            if has_var == false {
                if let Some(next) = self.scanner.peek() {
                    if let TokenType::Identifier(_) = next.token_type {
                    } else {
                        // EEK! We had a `,` and then some comments and now we're
                        // somewhere else. If you write code like this, you
                        // do not live in the light of the lord
                        end_delimiter = false;
                        break;
                    }
                } else {
                    // ACK! we're in a dingus's `,` but we're out of tokens.
                    // Who would do this? Do they deserve formatting?
                    // What are we eve doing here. Is this good for America?
                    end_delimiter = true;
                    break;
                }
            }

            let var_expr = self.expression()?;

            match var_expr.expr {
                Expr::Identifier { .. } | Expr::Assign { .. } => {}

                _ => {
                    // Ah shit you suck.
                    let has_semicolon = self.check_next_consume(TokenType::Semicolon);
                    self.check_leftovers = true;
                    self.leftover_stmts.push(StatementWrapper::new(
                        Statement::ExpresssionStatement { expression: var_expr },
                        has_semicolon,
                    ));
                    end_delimiter = true; // we never woulda gotten here if not for you cursed end delimiters!
                    break;
                }
            };

            let var_decl = VariableDecl {
                say_var,
                say_var_comments,
                var_expr,
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

        Ok(DelimitedLines {
            lines: arguments,
            has_end_delimiter: end_delimiter,
        })
    }

    fn block(&mut self) -> AnyResult<StmtBox<'a>> {
        let comments_after_lbrace = self.get_newlines_and_comments();

        let mut statements = Vec::new();

        while let Some(_) = self.scanner.peek() {
            if self.check_next_consume(TokenType::RightBrace) {
                break;
            } else {
                statements.push(self.statement()?);
            }
        }

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::Block {
                statements,
                comments_after_lbrace,
            },
            has_semicolon,
        ))
    }

    fn if_statement(&mut self) -> AnyResult<StmtBox<'a>> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let condition = self.expression()?;
        let then_branch = self.statement()?;
        let comments_between = self.get_newlines_and_comments();
        let else_branch = if self.check_next_consume(TokenType::Else) {
            Some(self.statement()?)
        } else {
            None
        };
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::If {
                comments_after_control_word,
                condition,
                then_branch,
                comments_between,
                else_branch,
            },
            has_semicolon,
        ))
    }

    fn while_with_repeat(&mut self, token: Token<'a>) -> AnyResult<StmtBox<'a>> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let condition = self.expression()?;
        let body = self.statement()?;
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::WhileWithRepeat {
                token,
                condition,
                body,
                comments_after_control_word,
            },
            has_semicolon,
        ))
    }

    fn do_until_statement(&mut self) -> AnyResult<StmtBox<'a>> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let body = self.statement()?;
        let comments_between = self.get_newlines_and_comments();
        self.check_next_consume(TokenType::Until);
        let condition = self.expression()?;
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::DoUntil {
                comments_after_control_word,
                comments_between,
                condition,
                body,
            },
            has_semicolon,
        ))
    }

    fn switch_statement(&mut self) -> AnyResult<StmtBox<'a>> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let condition = self.expression()?;
        self.check_next_consume(TokenType::LeftBrace);
        let comments_after_lbrace = self.get_newlines_and_comments();

        let mut cases: Vec<Case<'a>> = vec![];

        while let Some(token) = self.scanner.peek() {
            match token.token_type {
                TokenType::Case => {
                    self.consume_next();
                    let comments_after_control_word = self.get_newlines_and_comments();
                    let constant = self.expression()?;
                    self.check_next_consume(TokenType::Colon);
                    let comments_after_colon = self.get_newlines_and_comments();

                    let mut statements = Vec::new();
                    while let Some(token) = self.scanner.peek() {
                        match token.token_type {
                            TokenType::DefaultCase | TokenType::Case => {
                                break;
                            }
                            TokenType::RightBrace => {
                                break;
                            }
                            _ => {
                                statements.push(self.statement()?);
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
                    while let Some(token) = self.scanner.peek() {
                        match token.token_type {
                            TokenType::DefaultCase | TokenType::Case | TokenType::RightBrace => {
                                break;
                            }
                            _ => {
                                statements.push(self.statement()?);
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
                    anyhow::bail!("Unknown token {} in Switch statement", token);
                }
            }
        }

        self.check_next_consume(TokenType::RightBrace);

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::Switch {
                comments_after_control_word,
                comments_after_lbrace,
                cases,
                condition,
            },
            has_semicolon,
        ))
    }

    fn for_statement(&mut self) -> AnyResult<StmtBox<'a>> {
        let comments_after_control_word = self.get_newlines_and_comments();

        self.check_next_consume(TokenType::LeftParen);
        let comments_after_lparen = self.get_newlines_and_comments();

        let initializer = if self.check_next_consume(TokenType::Semicolon) {
            None
        } else if self.check_next(TokenType::Var) {
            Some(self.series_var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        let comments_after_initializer = self.get_newlines_and_comments();

        let condition = if self.check_next_consume(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };
        self.check_next_consume(TokenType::Semicolon);
        let comments_after_condition = self.get_newlines_and_comments();
        let increment = if self.check_next(TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.check_next_consume(TokenType::Semicolon);
        let comments_after_increment = self.get_newlines_and_comments();

        self.check_next_consume(TokenType::RightParen);
        let comments_after_rparen = self.get_newlines_and_comments();

        let body = self.statement()?;
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::For {
                comments_after_control_word,
                comments_after_lparen,
                initializer,
                comments_after_initializer,
                condition,
                comments_after_condition,
                increment,
                comments_after_increment,
                comments_after_rparen,
                body,
            },
            has_semicolon,
        ))
    }

    fn return_statement(&mut self) -> AnyResult<StmtBox<'a>> {
        let expression = if self.check_next(TokenType::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        Ok(StatementWrapper::new(
            Statement::Return { expression },
            has_semicolon,
        ))
    }

    fn break_statement(&mut self) -> AnyResult<StmtBox<'a>> {
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        Ok(StatementWrapper::new(Statement::Break, has_semicolon))
    }

    fn exit_statment(&mut self) -> AnyResult<StmtBox<'a>> {
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);
        Ok(StatementWrapper::new(Statement::Exit, has_semicolon))
    }

    fn enum_declaration(&mut self) -> AnyResult<StmtBox<'a>> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let name = self.expression()?;

        self.check_next_consume(TokenType::LeftBrace);
        let comments_after_lbrace = self.get_newlines_and_comments();

        let members = self.finish_call(TokenType::RightBrace, TokenType::Comma)?;
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::EnumDeclaration {
                comments_after_control_word,
                name,
                comments_after_lbrace,
                members,
            },
            has_semicolon,
        ))
    }

    fn expression_statement(&mut self) -> AnyResult<StmtBox<'a>> {
        let expr = self.expression()?;
        let has_semicolon = self.check_next_consume(TokenType::Semicolon);

        Ok(StatementWrapper::new(
            Statement::ExpresssionStatement { expression: expr },
            has_semicolon,
        ))
    }

    fn expression(&mut self) -> AnyResult<ExprBox<'a>> {
        self.allow_unidentified = true;
        let ret = self.assignment()?;
        self.can_pair = true;
        self.allow_unidentified = false;

        Ok(ret)
    }

    fn function_declaration(&mut self) -> AnyResult<ExprBox<'a>> {
        let comments_after_control_word = self.get_newlines_and_comments();
        let call = self.expression()?;
        let comments_after_rparen = self.get_newlines_and_comments();
        let is_constructor = self.check_next_consume(TokenType::Constructor);

        Ok(self.create_comment_expr_box(Expr::Function {
            comments_after_control_word,
            call,
            comments_after_rparen,
            is_constructor,
        }))
    }

    fn struct_operation(&mut self, token: Token<'a>) -> AnyResult<ExprBox<'a>> {
        let comments_before_expression = self.get_newlines_and_comments();
        let expression = self.expression()?;

        Ok(self.create_comment_expr_box(Expr::StructOperator {
            token,
            comments_before_expression,
            expression,
        }))
    }

    fn assignment(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.ternary()?;

        if let Expr::UnidentifiedAsLiteral { literal_token } = expr.expr {
            match literal_token.token_type {
                TokenType::Function => {
                    expr = self.function_declaration()?;
                }
                TokenType::New | TokenType::Delete => {
                    expr = self.struct_operation(literal_token)?;
                }
                _ => {}
            }
        }

        if self.can_pair {
            if let Some(token) = self.scanner.peek() {
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
                        let operator = self.scanner.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();

                        let assignment_expr = self.assignment()?;

                        expr = self.create_expr_box_no_comment(Expr::Assign {
                            left: expr,
                            operator: operator,
                            comments_and_newlines_between_op_and_r,
                            right: assignment_expr,
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.or()?;

        if self.check_next_consume(TokenType::Hook) {
            let comments_and_newlines_after_q = self.get_newlines_and_comments();
            let left = self.ternary()?;
            self.check_next_consume(TokenType::Colon);
            let comments_and_newlines_after_colon = self.get_newlines_and_comments();
            let right = self.ternary()?;

            expr = self.create_expr_box_no_comment(Expr::Ternary {
                conditional: expr,
                comments_and_newlines_after_q,
                left,
                comments_and_newlines_after_colon,
                right,
            });
        }

        Ok(expr)
    }

    // parse our Logical Operands here
    fn or(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut left = self.and()?;

        if self.check_next_either(TokenType::LogicalOr, TokenType::OrAlias) {
            let token = self.scanner.next().unwrap();
            let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
            let right = self.or()?;

            left = self.create_expr_box_no_comment(Expr::Binary {
                left,
                operator: token,
                comments_and_newlines_between_op_and_r,
                right,
            });
        }

        Ok(left)
    }

    fn and(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut left = self.xor()?;

        if self.check_next_either(TokenType::LogicalAnd, TokenType::AndAlias) {
            let token = self.scanner.next().unwrap();
            let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
            let right = self.and()?;

            left = self.create_expr_box_no_comment(Expr::Binary {
                left,
                operator: token,
                comments_and_newlines_between_op_and_r,
                right,
            });
        }

        Ok(left)
    }

    fn xor(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut left = self.equality()?;

        if self.check_next_either(TokenType::LogicalXor, TokenType::XorAlias) {
            let token = self.scanner.next().unwrap();
            let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
            let right = self.xor()?;

            left = self.create_expr_box_no_comment(Expr::Binary {
                left,
                operator: token,
                comments_and_newlines_between_op_and_r,
                right,
            })
        }

        Ok(left)
    }

    fn equality(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.comparison()?;

        if self.can_pair {
            while let Some(t) = self.scanner.peek() {
                if t.token_type == TokenType::EqualEqual || t.token_type == TokenType::BangEqual {
                    let token = self.scanner.next().unwrap();
                    let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                    let right = self.comparison()?;

                    expr = self.create_expr_box_no_comment(Expr::Binary {
                        left: expr,
                        operator: token,
                        comments_and_newlines_between_op_and_r,
                        right,
                    });
                } else {
                    break;
                }
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.binary()?;

        if self.can_pair {
            while let Some(t) = self.scanner.peek() {
                match t.token_type {
                    TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
                        let t = self.scanner.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.binary()?;

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: t,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                };
            }
        }

        Ok(expr)
    }

    fn binary(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.bitshift()?;

        if self.can_pair {
            while let Some(t) = self.scanner.peek() {
                match t.token_type {
                    TokenType::BitAnd | TokenType::BitOr | TokenType::BitXor => {
                        let t = self.scanner.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.bitshift()?;

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: t,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                }
            }
        }

        Ok(expr)
    }

    fn bitshift(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.addition()?;

        if self.can_pair {
            while let Some(t) = self.scanner.peek() {
                match t.token_type {
                    TokenType::BitLeft | TokenType::BitRight => {
                        let t = self.scanner.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.addition()?;

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: t,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                }
            }
        }

        Ok(expr)
    }

    fn addition(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.multiplication()?;

        if self.can_pair {
            while let Some(t) = self.scanner.peek() {
                match t.token_type {
                    TokenType::Minus | TokenType::Plus => {
                        let token = self.scanner.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.multiplication()?;

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: token,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                };
            }
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.unary()?;

        if self.can_pair {
            while let Some(t) = self.scanner.peek() {
                match t.token_type {
                    TokenType::Slash | TokenType::Star | TokenType::Mod | TokenType::ModAlias | TokenType::Div => {
                        let token = self.scanner.next().unwrap();
                        let comments_and_newlines_between_op_and_r = self.get_newlines_and_comments();
                        let right = self.unary()?;

                        expr = self.create_expr_box_no_comment(Expr::Binary {
                            left: expr,
                            operator: token,
                            comments_and_newlines_between_op_and_r,
                            right,
                        });
                    }
                    _ => break,
                };
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> AnyResult<ExprBox<'a>> {
        if self.can_pair {
            if let Some(t) = self.scanner.peek() {
                match t.token_type {
                    TokenType::Bang | TokenType::Minus | TokenType::Plus | TokenType::Tilde | TokenType::NotAlias => {
                        let t = self.scanner.next().unwrap();
                        let comments_and_newlines_between = self.get_newlines_and_comments();
                        let right = self.unary()?;

                        return Ok(self.create_expr_box_no_comment(Expr::Unary {
                            operator: t,
                            comments_and_newlines_between,
                            right,
                        }));
                    }

                    TokenType::Incrementer | TokenType::Decrementer => {
                        let t = self.scanner.next().unwrap();
                        let comments_and_newlines_between = self.get_newlines_and_comments();
                        let right = self.unary()?;

                        return Ok(self.create_expr_box_no_comment(Expr::Unary {
                            operator: t,
                            comments_and_newlines_between,
                            right,
                        }));
                    }

                    _ => {}
                }
            }
        }

        self.postfix()
    }

    fn postfix(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expr = self.call()?;

        if self.check_next_either(TokenType::Incrementer, TokenType::Decrementer) {
            let t = self.scanner.next().unwrap();

            let comments_and_newlines_between = self.get_newlines_and_comments();
            expr = self.create_expr_box_no_comment(Expr::Postfix {
                operator: t,
                comments_and_newlines_between,
                expr,
            });
        }

        Ok(expr)
    }

    fn call(&mut self) -> AnyResult<ExprBox<'a>> {
        let mut expression = self.primary()?;

        if self.check_next_consume(TokenType::LeftParen) {
            let comments_and_newlines_after_lparen = self.get_newlines_and_comments();
            let arguments = self.finish_call(TokenType::RightParen, TokenType::Comma)?;

            expression = self.create_comment_expr_box(Expr::Call {
                procedure_name: expression,
                arguments,
                comments_and_newlines_after_lparen,
            });
        }

        while let Some(token) = self.scanner.peek() {
            match token.token_type {
                TokenType::Dot => {
                    self.consume_next();
                    let comments_between = self.get_newlines_and_comments();
                    let instance_variable = self.call()?;
                    expression = self.create_comment_expr_box(Expr::DotAccess {
                        object_name: expression,
                        comments_between,
                        instance_variable,
                    });
                }

                TokenType::LeftBracket
                | TokenType::ArrayIndexer
                | TokenType::MapIndexer
                | TokenType::ListIndexer
                | TokenType::GridIndexer => {
                    let access_type = self.scanner.next().unwrap();
                    let mut access_exprs = vec![];

                    while let Some(token) = self.scanner.peek() {
                        if token.token_type == TokenType::RightBracket {
                            break;
                        }

                        access_exprs.push((self.get_newlines_and_comments(), self.expression()?));

                        if self.check_next_consume(TokenType::Comma) == false {
                            break;
                        }
                    }

                    self.check_next_consume(TokenType::RightBracket);
                    expression = self.create_comment_expr_box(Expr::DataStructureAccess {
                        ds_name: expression,
                        access_type,
                        access_exprs,
                    });
                }

                _ => break,
            }
        }

        Ok(expression)
    }

    fn primary(&mut self) -> AnyResult<ExprBox<'a>> {
        if let Some(t) = self.scanner.peek() {
            let output = match t.token_type {
                TokenType::Number(_) | TokenType::String(_) => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    self.create_expr_box_no_comment(Expr::Literal {
                        literal_token: t,
                        comments,
                    })
                }
                TokenType::NumberStartDot(_) => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    self.create_expr_box_no_comment(Expr::NumberStartDot {
                        literal_token: t,
                        comments,
                    })
                }
                TokenType::NumberEndDot(_) => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    self.create_expr_box_no_comment(Expr::NumberEndDot {
                        literal_token: t,
                        comments,
                    })
                }
                TokenType::Identifier(_) => {
                    let t = self.consume_next();
                    let comments = self.get_newlines_and_comments();
                    self.create_expr_box_no_comment(Expr::Identifier { name: t, comments })
                }
                TokenType::LeftParen => {
                    self.consume_next();
                    let comments_and_newlines_after_lparen = self.get_newlines_and_comments();

                    let mut expressions = vec![];
                    expressions.push(self.expression()?);
                    while self.check_next_consume(TokenType::RightParen) == false {
                        expressions.push(self.expression()?);
                    }

                    let comments_and_newlines_after_rparen = self.get_newlines_and_comments();

                    self.create_expr_box_no_comment(Expr::Grouping {
                        expressions,
                        comments_and_newlines_after_lparen,
                        comments_and_newlines_after_rparen,
                    })
                }

                TokenType::LeftBracket => {
                    self.consume_next();
                    let comments_and_newlines_after_lbracket = self.get_newlines_and_comments();
                    let arguments = self.finish_call(TokenType::RightBracket, TokenType::Comma)?;

                    self.create_expr_box_no_comment(Expr::ArrayLiteral {
                        comments_and_newlines_after_lbracket,
                        arguments,
                    })
                }

                TokenType::Newline(_) => {
                    self.consume_next();
                    self.can_pair = false;
                    self.create_expr_box_no_comment(Expr::Newline)
                }
                TokenType::Comment(_) => {
                    let comment = self.consume_next();
                    self.can_pair = false;
                    self.create_expr_box_no_comment(Expr::Comment { comment })
                }
                TokenType::MultilineComment(_) => {
                    let multiline_comment = self.consume_next();
                    self.can_pair = false;
                    self.create_expr_box_no_comment(Expr::MultilineComment { multiline_comment })
                }
                _ => {
                    let literal_token = self.consume_next();
                    if self.allow_unidentified == false {
                        anyhow::bail!("Error parsing {}", literal_token);
                    }

                    self.create_comment_expr_box(Expr::UnidentifiedAsLiteral { literal_token })
                }
            };

            return Ok(output);
        }

        anyhow::bail!("Unexpected end!");
    }

    fn finish_call(
        &mut self,
        end_token_type: TokenType,
        delimiter_type: TokenType,
    ) -> AnyResult<DelimitedLines<'a, ExprBox<'a>>> {
        let mut arguments = Vec::new();

        let mut end_delimiter = true;
        if self.check_next(end_token_type) == false {
            loop {
                if self.check_next(end_token_type) {
                    end_delimiter = true;
                    break;
                }

                let expr = self.expression()?;
                let do_break = self.check_next_consume(delimiter_type) == false;

                let trailing_comment = self.get_newlines_and_comments();

                arguments.push(DelimitedLine {
                    expr,
                    trailing_comment,
                });

                if do_break {
                    end_delimiter = false;
                    break;
                }
            }
        };
        self.check_next_consume(end_token_type);

        Ok(DelimitedLines {
            lines: arguments,
            has_end_delimiter: end_delimiter,
        })
    }

    fn check_next(&mut self, token_type: TokenType) -> bool {
        if self.can_pair == false {
            return false;
        }

        if let Some(t) = self.scanner.peek() {
            return t.token_type == token_type;
        }

        false
    }

    fn check_next_either(&mut self, token_type1: TokenType, token_type2: TokenType) -> bool {
        if self.can_pair == false {
            return false;
        }

        if let Some(t) = self.scanner.peek() {
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
    fn get_newlines_and_comments(&mut self) -> Option<Vec<Token<'a>>> {
        let mut ret: Option<Vec<Token<'a>>> = None;
        while let Some(token) = self.scanner.peek() {
            match token.token_type {
                TokenType::Newline(_)
                | TokenType::Comment(_)
                | TokenType::MultilineComment(_)
                | TokenType::RegionBegin(_)
                | TokenType::RegionEnd(_)
                | TokenType::Then => {
                    let token = self.scanner.next().unwrap();
                    if let Some(vec) = &mut ret {
                        vec.push(token);
                    } else {
                        ret = Some(vec![token]);
                    }
                }
                _ => break,
            }
        }

        ret
    }

    fn consume_next(&mut self) -> Token<'a> {
        self.scanner.next().unwrap()
    }

    fn create_comment_expr_box(&mut self, expr: Expr<'a>) -> ExprBox<'a> {
        Box::new(ExprBoxInterior {
            expr,
            trailing_comments: self.get_newlines_and_comments(),
        })
    }

    fn create_expr_box_no_comment(&self, expr: Expr<'a>) -> ExprBox<'a> {
        Box::new(ExprBoxInterior {
            expr,
            trailing_comments: None,
        })
    }
}
