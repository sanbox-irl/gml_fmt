use super::expressions::*;
use super::lex_token::TokenType;
use super::lex_token::*;
use super::statements::*;
use std::iter::Peekable;
use std::slice;

type ExprBox<'a> = Box<Expr<'a>>;
type StmtBox<'a> = Box<StatementWrapper<'a>>;

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
                TokenType::EOF => break,
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
                    return StatementWrapper::new(Statement::MultilineComment {
                        multiline_comment: *multiline_comment,
                    }, false);
                }
                TokenType::RegionBegin => {
                    self.consume_next();
                    return self.region_begin();
                }
                TokenType::RegionEnd => {
                    self.consume_next();
                    return StatementWrapper::new(Statement::RegionEnd, false);
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

    fn region_begin(&mut self) -> StmtBox<'a> {
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

        StatementWrapper::new(Statement::RegionBegin { multi_word_name }, false)
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
        self.check_next_consume(TokenType::Var);

        let var_expr = self.primary();

        let assignment = if self.check_next(TokenType::Equal) {
            self.iter.next();
            Some(self.expression())
        } else {
            None
        };

        VariableDecl {
            var_expr,
            assignment,
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
        self.check_next_consume(TokenType::LeftParen);

        let condition = self.expression();

        self.check_next_consume(TokenType::RightParen);

        let then_branch = self.statement();
        let else_branch = if self.check_next_consume(TokenType::Else) {
            Some(self.statement())
        } else {
            None
        };

        StatementWrapper::new(
            Statement::If {
                condition,
                then_branch,
                else_branch,
            },
            false,
        )
    }

    fn while_statement(&mut self) -> StmtBox<'a> {
        self.check_next_consume(TokenType::LeftParen);

        let condition = self.expression();

        self.check_next_consume(TokenType::RightParen);

        let body = self.statement();

        StatementWrapper::new(Statement::While { condition, body }, false)
    }

    fn switch_statement(&mut self) -> StmtBox<'a> {
        self.check_next_consume(TokenType::LeftParen);

        let condition = self.expression();

        self.check_next_consume(TokenType::RightParen);
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

        Case {
            statements,
            constant,
        }
    }

    fn repeat_statement(&mut self) -> StmtBox<'a> {
        self.check_next_consume(TokenType::LeftParen);

        let condition = self.expression();

        self.check_next_consume(TokenType::RightParen);
        let body = self.statement();

        StatementWrapper::new(Statement::Repeat { condition, body }, false)
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
        StatementWrapper::new(
            Statement::ExpresssionStatement { expression: expr },
            has_semicolon,
        )
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
                    let assignment_expr = self.assignment();

                    expr = Box::new(Expr::Assign {
                        left: expr,
                        operator: *operator,
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
            let left = self.ternary();
            self.check_next_consume(TokenType::Colon);
            let right = self.ternary();

            expr = Box::new(Expr::Ternary {
                conditional: expr,
                left,
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

            let right = self.equality();

            left = Box::new(Expr::Logical {
                left,
                operator: *token,
                right,
            });
        }

        left
    }

    fn and(&mut self) -> ExprBox<'a> {
        let mut left = self.xor();

        if self.check_next_either(TokenType::LogicalAnd, TokenType::AndAlias) {
            let token = self.iter.next().unwrap();

            let right = self.xor();

            left = Box::new(Expr::Logical {
                left,
                operator: *token,
                right,
            });
        }
        left
    }

    fn xor(&mut self) -> ExprBox<'a> {
        let mut left = self.equality();

        if self.check_next_either(TokenType::LogicalXor, TokenType::XorAlias) {
            let token = self.iter.next().unwrap();
            let right = self.equality();

            left = Box::new(Expr::Logical {
                left,
                operator: *token,
                right,
            })
        }

        left
    }

    fn equality(&mut self) -> ExprBox<'a> {
        let mut expr = self.comparison();

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::EqualEqual | TokenType::BangEqual => {
                    let token = self.iter.next().unwrap();
                    let right = self.comparison();

                    expr = Box::new(Expr::Binary {
                        left: expr,
                        operator: *token,
                        right,
                    });
                }
                _ => break,
            };
        }

        expr
    }

    fn comparison(&mut self) -> ExprBox<'a> {
        let mut expr = self.binary();

        while let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::Less
                | TokenType::LessEqual => {
                    let t = self.iter.next().unwrap();
                    let right = self.binary();

                    expr = Box::new(Expr::Binary {
                        left: expr,
                        operator: *t,
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
                    let t = self.iter.next().unwrap();
                    let right = self.bitshift();

                    expr = Box::new(Expr::Binary {
                        left: expr,
                        operator: *t,
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
                    let t = self.iter.next().unwrap();
                    let right = self.addition();

                    expr = Box::new(Expr::Binary {
                        left: expr,
                        operator: *t,
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
                    let token = self.iter.next().unwrap();
                    let right = self.multiplication();

                    expr = Box::new(Expr::Binary {
                        left: expr,
                        operator: *token,
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
                TokenType::Slash
                | TokenType::Star
                | TokenType::Mod
                | TokenType::ModAlias
                | TokenType::Div => {
                    let token = self.iter.next().unwrap();
                    let right = self.unary();

                    expr = Box::new(Expr::Binary {
                        left: expr,
                        operator: *token,
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
                TokenType::Bang | TokenType::Minus => {
                    let t = self.iter.next().unwrap();
                    let right = self.unary();

                    return Box::new(Expr::Unary {
                        operator: *t,
                        right,
                    });
                }

                TokenType::Incrementer | TokenType::Decrementer => {
                    let t = self.iter.next().unwrap();
                    let expr = self.unary();

                    return Box::new(Expr::Prefix { operator: *t, expr });
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
            expr = Box::new(Expr::Postfix { operator: *t, expr })
        }

        expr
    }

    fn call(&mut self) -> ExprBox<'a> {
        let mut expression = self.primary();

        if self.check_next_consume(TokenType::LeftParen) {
            let arguments = self.finish_call();

            expression = Box::new(Expr::Call {
                procedure_name: expression,
                arguments,
            });
        }

        while let Some(token) = self.iter.peek() {
            match token.token_type {
                TokenType::Dot => {
                    self.consume_next();
                    if let Some(t) = self.iter.peek() {
                        if let TokenType::Identifier(_) = t.token_type {
                            let instance_variable = self.consume_next();

                            expression = Box::new(Expr::DotAccess {
                                instance_variable: *instance_variable,

                                object_name: expression,
                            });
                        }
                    }
                }

                TokenType::LeftBracket
                | TokenType::ArrayIndexer
                | TokenType::MapIndexer
                | TokenType::ListIndexer => {
                    let token = self.iter.next().unwrap();
                    let access_expr = self.expression();

                    self.check_next_consume(TokenType::RightBracket);

                    // stupid non-chained access..
                    return Box::new(Expr::DataStructureAccess {
                        ds_name: expression,
                        access_type: *token,
                        access_expr,
                    });
                }

                TokenType::GridIndexer => {
                    let token = self.iter.next().unwrap();
                    let row_expr = self.expression();
                    self.check_next_consume(TokenType::Comma);
                    let column_expr = self.expression();
                    self.check_next_consume(TokenType::RightBracket);

                    // stupid non-chained access..
                    return Box::new(Expr::GridDataStructureAccess {
                        ds_name: expression,
                        access_type: *token,
                        column_expr,
                        row_expr,
                    });
                }

                _ => break,
            }
        }

        expression
    }

    fn finish_call(&mut self) -> Vec<ExprBox<'a>> {
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

    fn primary(&mut self) -> ExprBox<'a> {
        if let Some(t) = self.iter.peek() {
            match t.token_type {
                TokenType::False | TokenType::True => {
                    let t = self.consume_next();
                    return Box::new(Expr::Literal { literal_token: *t });
                }
                TokenType::Number(_) | TokenType::String(_) => {
                    let t = self.consume_next();
                    return Box::new(Expr::Literal { literal_token: *t });
                }
                TokenType::Identifier(_) => {
                    let t = self.consume_next();
                    return Box::new(Expr::Identifier { name: *t });
                }
                TokenType::LeftParen => {
                    self.consume_next();
                    let expression = self.expression();

                    if let Some(t) = self.iter.peek() {
                        if t.token_type == TokenType::RightParen {
                            self.iter.next();
                        }
                    }

                    return Box::new(Expr::Grouping { expression });
                }

                TokenType::Newline => {
                    let t = self.consume_next();
                    return Box::new(Expr::Newline { token: *t });
                }
                _ => {
                    let t = self.consume_next();
                    return Box::new(Expr::UnidentifiedAsLiteral { literal_token: *t });
                }
            }
        }

        Box::new(Expr::UnexpectedEnd)
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

    #[allow(dead_code)]
    fn eat_all_newlines(&mut self) {
        while let Some(token) = self.iter.peek() {
            if token.token_type == TokenType::Newline {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    fn consume_next(&mut self) -> &Token<'a> {
        self.iter.next().unwrap()
    }
}
