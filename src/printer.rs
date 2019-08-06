use super::expressions::*;
use super::lex_token::{Token, TokenType};
use super::statements::*;

type StmtBox<'a> = Box<StatementWrapper<'a>>;

const SPACE: &str = " ";
const TAB: &str = "    ";
const NEWLINE: &str = "\n";
const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";
const COMMA: &str = ",";
const SEMICOLON: &str = ";";

pub struct Printer<'a> {
    pub output: Vec<&'a str>,
    indentation: i32,
    default_handler: DefaultWhitespaceHandler,
    can_replace_handler: bool,
}

impl<'a> Printer<'a> {
    pub fn new() -> Printer<'a> {
        Printer {
            output: Vec::new(),
            indentation: 0,
            default_handler: DefaultWhitespaceHandler {},
            can_replace_handler: true,
        }
    }

    pub fn get_output(vec_output: &Vec<&'a str>) -> String {
        let mut output = String::new();

        for this_one in vec_output {
            output.push_str(this_one);
        }

        output
    }

    pub fn autoformat(&mut self, ast: &'a [StmtBox<'a>]) {
        for this_statement in ast {
            self.print_statement(this_statement);
        }
    }

    fn print_statement(&mut self, stmt: &'a StatementWrapper<'a>) {
        match &stmt.statement {
            Statement::VariableDeclList { var_decl } => {
                self.print("var", true);

                let mut iter = var_decl.into_iter().peekable();
                while let Some(this_decl) = iter.next() {
                    self.print_expr(&this_decl.var_expr);

                    if let Some((comments, expr_box)) = &this_decl.assignment {
                        self.print("=", true);
                        DefaultWhitespaceHandler {}.print_comments_and_newlines(self, comments, IndentationMove::Stay);
                        self.print_expr(expr_box);
                    }
                    self.backspace();

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                    }

                    self.print_semicolon(stmt.has_semicolon);
                }
            }
            Statement::EnumDeclaration { name, members } => {
                self.print_expr(name);
                self.print(LBRACE, false);
                self.print_newline(IndentationMove::Right);

                let mut iter = members.into_iter().peekable();
                while let Some(this_member) = iter.next() {
                    self.print_token(&this_member.name, true);

                    if let Some(expr_box) = &this_member.value {
                        self.print("=", true);
                        self.print_expr(expr_box);
                    }
                    self.backspace();

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                        self.print_newline(IndentationMove::Stay);
                    } else {
                        break;
                    }
                }

                self.print_newline(IndentationMove::Left);
                self.print(RBRACE, false);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::ExpresssionStatement { expression } => {
                self.print_expr(expression);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Block { statements } => {
                self.print(LBRACE, false);

                self.indentation += 1;
                for this_stmt in statements {
                    self.print_statement(this_stmt);
                }
                if self.on_whitespace_line() {
                    self.backspace_till_newline();
                    self.print_indentation(IndentationMove::Left);
                }
                self.print(RBRACE, false);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.print("if", true);
                self.print_expr_parentheses(condition);
                self.print_statement(then_branch);

                if let Some(else_branch) = else_branch {
                    self.print(SPACE, false);
                    self.print("else", true);

                    self.print_statement(else_branch);
                }
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::While { condition, body } => {
                self.print("while", true);
                self.print_expr_parentheses(condition);
                self.print_statement(body);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Repeat { condition, body } => {
                self.print("repeat", true);
                self.print_expr_parentheses(condition);
                self.print_statement(body);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                self.print("for", true);
                self.print(LPAREN, false);

                if let Some(initializer) = initializer {
                    self.print_statement(initializer);
                }

                self.backspace();
                self.print(SEMICOLON, true);

                if let Some(condition) = condition {
                    self.print_expr(condition);
                } else {
                    self.backspace();
                }

                self.backspace();
                self.print(SEMICOLON, true);

                if let Some(increment) = increment {
                    self.print_expr(increment);
                } else {
                    self.backspace();
                }

                self.backspace();
                self.print(RPAREN, true);

                self.print_statement(body);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Return { expression } => {
                self.print("return", false);

                if let Some(expression) = expression {
                    self.print(SPACE, false);
                    self.print_expr(expression);
                }
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Break => {
                self.print("break", false);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Exit => {
                self.print("exit", false);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Switch {
                condition,
                cases,
                default,
            } => {
                self.print("switch", true);
                self.print_expr_parentheses(condition);

                self.print(LBRACE, true);
                self.print_newline(IndentationMove::Right);

                if let Some(cases) = cases {
                    for this_case in cases {
                        self.print("case", true);
                        self.print_expr(&this_case.constant);
                        self.backspace();
                        self.print(":", true);

                        for this_case in &this_case.statements {
                            self.print_statement(this_case);
                        }
                    }
                }

                if let Some(default) = default {
                    for this_case in default {
                        self.print("default", true);
                        self.print_expr(&this_case.constant);
                        self.backspace();
                        self.print(":", true);

                        for this_case in &this_case.statements {
                            self.print_statement(this_case);
                        }
                    }
                }

                self.print_newline(IndentationMove::Left);
                self.print(RBRACE, false);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Comment { comment } => self.print_token(comment, false),
            Statement::MultilineComment { multiline_comment } => self.print_token(multiline_comment, false),
            Statement::RegionBegin { multi_word_name } => {
                self.print("#region", true);

                for this_word in multi_word_name {
                    self.print_token(this_word, true);
                }
                self.backspace();
            }
            Statement::RegionEnd => self.print("#endregion", false),
            Statement::Macro { macro_body } => {
                self.print("#macro", true);

                for this_word in macro_body {
                    self.print_token(this_word, true);
                }
                self.backspace();
            }
            Statement::Define { script_name, body } => {
                self.print("#define", true);
                self.print_expr(script_name);
                self.backspace();

                for this_stmt in body {
                    self.print_statement(this_stmt);
                }
            }
            Statement::EOF => {
                self.print(NEWLINE, false);
            }
        }
    }

    fn print_expr(&mut self, expr: &'a ExprBox<'a>) {
        let mut new_handle = DefaultWhitespaceHandler {};
        self.print_expr_white_space_controller(expr, &mut new_handle);
    }

    fn print_expr_white_space_controller(
        &mut self,
        expr: &'a ExprBox<'a>,
        whitespace_handler: &mut WhiteSpaceHandler<'a>,
    ) {
        match &expr.0 {
            Expr::Call {
                procedure_name,
                comments_and_newlines_after_lparen,
                arguments,
            } => {
                self.print_expr(procedure_name);
                self.backspace();

                self.print(LPAREN, false);
                let did_move = whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_after_lparen,
                    IndentationMove::Right,
                );

                let mut iter = arguments.into_iter().peekable();
                while let Some((first_comments, this_argument, these_comments)) = iter.next() {
                    whitespace_handler.print_comments_and_newlines(self, first_comments, IndentationMove::Stay);
                    self.print_expr(this_argument);
                    self.backspace();

                    whitespace_handler.print_comments_and_newlines(self, these_comments, IndentationMove::Stay);

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                    }
                }
                if did_move {
                    self.print_newline(IndentationMove::Left);
                }
                self.print(RPAREN, false);
            }

            Expr::Binary {
                left,
                comments_and_newlines_between_l_and_op,
                operator,
                comments_and_newlines_between_r_and_op,
                right,
            } => {
                self.print_expr(left);
                whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_between_l_and_op,
                    IndentationMove::Stay,
                );
                self.print_token(operator, true);
                whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_between_r_and_op,
                    IndentationMove::Stay,
                );
                self.print_expr(right);
            }

            Expr::Grouping {
                expression,
                comments_and_newlines_after_lparen,
                comments_and_newlines_before_rparen,
            } => {
                self.print(LPAREN, false);
                let did_move = whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_after_lparen,
                    IndentationMove::Right,
                );
                self.print_expr(expression);
                self.backspace();
                whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_before_rparen,
                    IndentationMove::Stay,
                );
                if did_move {
                    if self.on_whitespace_line() {
                        self.backspace_till_newline();
                        self.print_indentation(IndentationMove::Left);
                    } else {
                        self.print_newline(IndentationMove::Left);
                    }
                }
                self.print(RPAREN, true);
            }

            Expr::ArrayLiteral {
                comments_and_newlines_after_lbracket,
                arguments,
            } => {
                self.print("[", false);
                let did_move = whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_after_lbracket,
                    IndentationMove::Right,
                );

                let mut iter = arguments.into_iter().peekable();
                while let Some((initial_comment, this_argument, trailing_comment)) = iter.next() {
                    whitespace_handler.print_comments_and_newlines(self, initial_comment, IndentationMove::Stay);
                    self.print_expr(this_argument);
                    self.backspace();

                    whitespace_handler.print_comments_and_newlines(self, trailing_comment, IndentationMove::Stay);

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                    }
                }
                if did_move {
                    self.print_newline(IndentationMove::Left);
                }
                self.print("]", false);
            }

            Expr::Literal {
                literal_token,
                comments,
            } => {
                self.print_token(&literal_token, true);
                whitespace_handler.print_comments_and_newlines(self, comments, IndentationMove::Stay);
            }
            Expr::Unary { operator, right } => {
                self.print_token(&operator, false);
                self.print_expr(right);
            }
            Expr::Prefix { operator, expr } => {
                self.print_token(&operator, false);
                self.print_expr(expr);
            }
            Expr::Postfix { operator, expr } => {
                self.print_expr(expr);
                self.backspace();
                self.print_token(&operator, false);
            }
            Expr::Logical {
                left,
                operator,
                comments_and_newlines_between_op_and_r,
                right,
            } => {
                self.print_expr(left);
                self.print_token(&operator, true);
                whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_between_op_and_r,
                    IndentationMove::Stay,
                );
                self.print_expr(right);
            }
            Expr::Assign {
                left,
                operator,
                comments_and_newlines_between_op_and_r,
                right,
            } => {
                self.print_expr(left);
                self.print_token(&operator, true);
                whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_between_op_and_r,
                    IndentationMove::Stay,
                );
                self.print_expr(right);
            }
            Expr::Identifier { name, comments } => {
                self.print_token(name, true);
                whitespace_handler.print_comments_and_newlines(self, comments, IndentationMove::Stay);
            }

            Expr::DotAccess {
                object_name,
                instance_variable,
            } => {
                if self.can_replace_handler {
                    self.can_replace_handler = false;
                    let mut our_handle = DotWhitespaceHandler::new();

                    self.print_expr_white_space_controller(object_name, &mut our_handle);
                    self.backspace();
                    self.print(".", false);

                    self.print_expr_white_space_controller(instance_variable, &mut our_handle);
                    self.can_replace_handler = true;
                } else {
                    self.print_expr_white_space_controller(object_name, whitespace_handler);
                    self.backspace();
                    self.print(".", false);

                    self.print_expr_white_space_controller(instance_variable, whitespace_handler);
                }
            }
            Expr::DataStructureAccess {
                ds_name,
                access_type,
                comments_and_newlines_between_access_and_expr,
                access_expr,
            } => {
                self.print_expr(ds_name);
                self.backspace();

                self.print_token(&access_type, access_type.token_type != TokenType::LeftBracket);
                whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_between_access_and_expr,
                    IndentationMove::Stay,
                );
                self.print_expr(access_expr);

                self.backspace();
                self.print("]", true);
            }
            Expr::GridDataStructureAccess {
                ds_name,
                access_type,
                comments_and_newlines_between_access_type_and_row_expr,
                row_expr,
                comments_and_newlines_after_comma,
                column_expr,
            } => {
                self.print_expr(ds_name);
                self.print_token(&access_type, true);
                whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_between_access_type_and_row_expr,
                    IndentationMove::Stay,
                );
                self.print_expr(row_expr);

                self.print(COMMA, true);
                whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_after_comma,
                    IndentationMove::Stay,
                );

                self.print_expr(column_expr);

                self.backspace();
                self.print("]", true);
            }
            /*
            result = foo == bar  ? result1 :
                     foo == baz  ? result2 :
                     foo == qux  ? result3 :
                     foo == quux ? result4 :
                                     fail_result;
            */
            Expr::Ternary {
                conditional,
                comments_and_newlines_after_q,
                left,
                comments_and_newlines_after_colon,
                right,
            } => {
                self.print_expr(conditional);
                self.print("?", true);
                if self.only_newlines(comments_and_newlines_after_q) == false {
                    whitespace_handler.print_comments_and_newlines(
                        self,
                        comments_and_newlines_after_q,
                        IndentationMove::Right,
                    );
                }
                self.print_expr(left);
                self.print(":", true);
                let did_move = whitespace_handler.print_comments_and_newlines(
                    self,
                    comments_and_newlines_after_colon,
                    IndentationMove::Right,
                );
                self.print_expr(right);

                if did_move {
                    self.indentation -= 1;
                }
            }

            Expr::Newline { token: _ } => self.print_newline(IndentationMove::Stay),
            Expr::UnidentifiedAsLiteral { literal_token } => {
                self.print_token(&literal_token, false);
            }
            Expr::UnexpectedEnd => {}
        }
    }

    fn print_token(&mut self, token: &Token<'a>, space_after: bool) {
        self.print(token.print_name(), space_after);
    }

    fn print(&mut self, this_string: &'a str, space_after: bool) {
        self.output.push(this_string);
        if space_after {
            self.output.push(SPACE);
        }
    }

    fn print_expr_parentheses(&mut self, expr: &'a ExprBox<'a>) {
        self.print(LPAREN, false);
        self.print_expr(expr);
        self.backspace();
        self.print(RPAREN, true);
    }

    fn on_whitespace_line(&self) -> bool {
        let mut pos = self.output.len();
        if pos == 0 {
            return false;
        };

        pos -= 1;

        while pos != 0 {
            match self.output[pos] {
                SPACE | TAB => {
                    pos -= 1;
                }
                NEWLINE => return true,
                _ => break,
            }
        }

        false
    }

    fn prev_line_was_whitespace(&self) -> bool {
        let mut pos = self.output.len();
        if pos == 0 {
            return false;
        };

        pos -= 1;
        let mut ignore_newline = true;

        while pos != 0 {
            match self.output[pos] {
                SPACE | TAB => {
                    pos -= 1;
                }
                NEWLINE => {
                    if ignore_newline {
                        pos -= 1;
                        ignore_newline = false;
                    } else {
                        return true;
                    }
                }
                _ => break,
            }
        }

        false
    }

    fn backspace_till_newline(&mut self) {
        let mut pos = self.output.len();
        if pos == 0 {
            return;
        };

        pos -= 1;

        while pos != 0 {
            match self.output[pos] {
                NEWLINE => break,
                _ => {
                    self.output.remove(pos);
                    pos -= 1;
                }
            };
        }
    }

    fn backspace(&mut self) {
        let pos = self.output.len();
        if pos != 0 && self.output[pos - 1] == SPACE {
            self.output.remove(pos - 1);
        }
    }

    fn print_newline(&mut self, indentation_move: IndentationMove) {
        if self.prev_line_was_whitespace() {
            return;
        }
        if self.output.len() == 0 {
            return;
        }

        self.backspace();

        self.print(NEWLINE, false);
        self.print_indentation(indentation_move);
    }

    fn print_indentation(&mut self, indentation_move: IndentationMove) {
        match indentation_move {
            IndentationMove::Right => self.indentation += 1,
            IndentationMove::Stay => {}
            IndentationMove::Left => {
                self.indentation -= 1;
                if self.indentation < 0 {
                    self.indentation = 0;
                }
            }
        }

        for _ in 0..self.indentation {
            self.print(TAB, false);
        }
    }

    fn only_newlines(&mut self, vec: &'a Vec<Token<'a>>) -> bool {
        for this_token in vec {
            if this_token.token_type == TokenType::Newline {
                continue;
            }
            return false;
        }
        true
    }

    fn print_semicolon(&mut self, do_it: bool) {
        if do_it {
            self.backspace();
            self.print(SEMICOLON, false);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum IndentationMove {
    Right,
    Stay,
    Left,
}

trait WhiteSpaceHandler<'a> {
    fn print_comments_and_newlines(
        &mut self,
        printer: &mut Printer<'a>,
        vec: &'a Vec<Token<'a>>,
        indentation_move: IndentationMove,
    ) -> bool {
        let mut did_move = false;

        for this_one in vec {
            match this_one.token_type {
                TokenType::Newline => {
                    if did_move {
                        printer.print_newline(IndentationMove::Stay);
                    } else {
                        did_move = true;
                        printer.print_newline(indentation_move);
                    }
                }
                TokenType::Comment(_) | TokenType::MultilineComment(_) => printer.print_token(this_one, false),
                _ => {
                    println!(
                        "Printing {} which isn't a newline or comment in a comment_newline section...",
                        this_one
                    );
                    printer.print_token(this_one, false);
                }
            }
        }

        did_move
    }
}

struct DefaultWhitespaceHandler {}
impl<'a> WhiteSpaceHandler<'a> for DefaultWhitespaceHandler {}

struct DotWhitespaceHandler {
    do_it: bool,
}

impl DotWhitespaceHandler {
    fn new() -> DotWhitespaceHandler {
        DotWhitespaceHandler { do_it: true }
    }
}

impl<'a> WhiteSpaceHandler<'a> for DotWhitespaceHandler {
    fn print_comments_and_newlines(
        &mut self,
        printer: &mut Printer<'a>,
        vec: &'a Vec<Token<'a>>,
        indentation_move: IndentationMove,
    ) -> bool {
        let mut did_move = false;

        for this_one in vec {
            match this_one.token_type {
                TokenType::Newline => {
                    if did_move {
                        printer.print_newline(IndentationMove::Stay);
                    } else {
                        did_move = true;
                        if self.do_it {
                            printer.print_newline(IndentationMove::Right);
                            self.do_it = false;
                        } else {
                            printer.print_newline(indentation_move);
                        }
                    }
                }

                TokenType::Comment(_) | TokenType::MultilineComment(_) => printer.print_token(this_one, false),

                _ => {
                    println!(
                        "Printing {} which isn't a newline or comment in a comment_newline section...",
                        this_one
                    );
                    printer.print_token(this_one, false);
                }
            }
        }

        did_move
    }
}
