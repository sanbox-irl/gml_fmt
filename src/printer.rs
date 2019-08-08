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
    indentation: usize,
    can_replace_handler: bool,
    force_indentation: Option<IndentationMove>,
    do_not_print_newline_comments: bool,
    do_not_print_single_blankline_comments: bool,
    do_not_print_single_newline_statement: bool,
    do_not_print_newline_after_block: bool,
    // accept_original_indentation: bool,
}

impl<'a> Printer<'a> {
    pub fn new() -> Printer<'a> {
        Printer {
            output: Vec::new(),
            indentation: 0,
            can_replace_handler: true,
            force_indentation: None,
            do_not_print_newline_comments: false,
            do_not_print_single_blankline_comments: false,
            do_not_print_single_newline_statement: false,
            do_not_print_newline_after_block: false,
            // accept_original_indentation: false,
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
                    if this_decl.say_var {
                        self.print("var", true);
                    }
                    self.print_expr(&this_decl.var_expr);

                    if let Some((comments, expr_box)) = &this_decl.assignment {
                        self.print("=", true);
                        self.print_comments_and_newlines(comments, IndentationMove::Stay);
                        self.print_expr(expr_box);
                    }
                    self.backspace();

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                    }
                }
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::EnumDeclaration {
                name,
                comments_after_lbrace,
                members,
            } => {
                self.print("enum", true);
                self.print_expr(name);
                self.print(LBRACE, true);

                let did_move = self.print_comments_and_newlines(comments_after_lbrace, IndentationMove::Right);
                if did_move == false {
                    self.print_newline(IndentationMove::Right);
                }
                self.backspace();

                let mut iter = members.into_iter().peekable();
                while let Some(delimited_line) = iter.next() {
                    self.print_expr(&delimited_line.expr);
                    self.backspace();

                    let at_end = if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                        false
                    } else {
                        true
                    };

                    match &delimited_line.trailing_comment {
                        Some(comment) => {
                            let did_newlines = self.print_comments_and_newlines(&comment, IndentationMove::Stay);

                            if did_newlines == false {
                                self.print_newline(IndentationMove::Stay);
                            }
                        }
                        None => {
                            self.backspace();
                            if at_end == false {
                                self.print_newline(IndentationMove::Stay);
                            }
                        }
                    };
                }
                self.indentation -= 1;
                self.backspace_till_newline();

                self.print(RBRACE, false);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::ExpresssionStatement { expression } => {
                self.print_expr(expression);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Block {
                statements,
                comments_after_lbrace,
            } => {
                self.ensure_space();
                self.print(LBRACE, false);

                // Back it an indented block if we have more than one statement...
                let must_indent = statements.len() > 1;
                if must_indent {
                    self.do_not_print_newline_comments = true;
                };
                let did_move = self.print_comments_and_newlines(comments_after_lbrace, IndentationMove::Right);
                if did_move == false && must_indent {
                    self.print_newline(IndentationMove::Right);
                }
                self.do_not_print_newline_comments = false;
                let did_newline = did_move || must_indent;

                if did_newline == false {
                    self.ensure_space();
                }

                let mut iter = statements.into_iter().peekable();
                while let Some(stmt) = iter.next() {
                    self.print_statement(stmt);
                    if did_newline {
                        if self.on_whitespace_line() == false {
                            self.print_newline(IndentationMove::Stay);
                            self.do_not_print_single_newline_statement = true;
                        }
                    }
                }

                if did_newline {
                    self.backspace_whitespace();
                    self.print_newline(IndentationMove::Left);
                } else {
                    self.ensure_space();
                }

                self.print(RBRACE, false);
                self.print_semicolon(stmt.has_semicolon);

                if self.do_not_print_newline_after_block == false {
                    self.ensure_newline(IndentationMove::Stay);
                }

                self.do_not_print_single_newline_statement = true;
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.print("if", true);
                self.print_expr(condition);
                self.print_statement(then_branch);

                if let Some(else_branch) = else_branch {
                    self.backspace_whitespace();
                    self.print(SPACE, false);
                    self.print("else", true);
                    self.print_statement(else_branch);
                }
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::While { condition, body } => {
                self.print("while", true);
                self.print_expr(condition);
                self.print_statement(body);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::DoUntil { condition, body } => {
                self.print("do", true);
                self.do_not_print_newline_after_block = true;
                self.print_statement(body);
                self.do_not_print_newline_after_block = false;
                self.ensure_space();
                self.print("until", true);
                self.print_expr(condition);
                self.backspace();
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Repeat { condition, body } => {
                self.print("repeat", true);
                self.print_expr(condition);
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
                    self.ensure_space();
                } else {
                    self.print(SEMICOLON, true);
                }

                if let Some(condition) = condition {
                    self.print_expr(condition);
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
                comments_after_lbrace,
                cases,
            } => {
                self.print("switch", true);
                self.do_not_print_newline_comments = true;
                self.print_expr(condition);
                self.do_not_print_newline_comments = false;

                self.ensure_space();
                self.print(LBRACE, true);
                let did_move = self.print_comments_and_newlines(comments_after_lbrace, IndentationMove::Right);
                if did_move == false {
                    self.print_newline(IndentationMove::Right);
                }

                let mut iter = cases.into_iter().peekable();
                while let Some(case) = iter.next() {
                    if let CaseType::Case(case_constant) = &case.case_type {
                        self.print("case", true);
                        self.print_expr(case_constant);
                    } else {
                        self.print("default", false);
                    }

                    self.print_comments_and_newlines(&case.comments_after_case, IndentationMove::Stay);
                    self.backspace();
                    self.print(":", true);

                    self.do_not_print_single_blankline_comments = true;
                    let did_move = self.print_comments_and_newlines(&case.comments_after_colon, IndentationMove::Right);
                    self.do_not_print_single_blankline_comments = false;
                    if did_move == false {
                        self.print_newline(IndentationMove::Right);
                    }

                    // @jack do we handle blocks here in a special way?
                    for this_statement in &case.statements {
                        self.print_statement(this_statement);
                    }

                    // No blank lines on final iteration!
                    if let Some(_) = iter.peek() {
                        if self.on_whitespace_line() == false {
                            self.print_newline(IndentationMove::Left);
                        } else {
                            self.backspace_till_newline();
                            self.print_indentation(IndentationMove::Left);
                        }
                    } else {
                        self.set_indentation(IndentationMove::Left);
                    }
                }

                if self.on_whitespace_line() {
                    self.backspace_till_newline();
                    self.print_indentation(IndentationMove::Left);
                } else {
                    self.print_newline(IndentationMove::Left);
                }

                self.print(RBRACE, false);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Comment { comment } => self.print_token(comment, false),
            Statement::MultilineComment { multiline_comment } => self.print_token(multiline_comment, true),
            Statement::RegionBegin { multi_word_name } => {
                self.print("#region", true);

                for this_word in multi_word_name {
                    self.print_token(this_word, true);
                }
                self.backspace();
            }
            Statement::RegionEnd { multi_word_name } => {
                self.print("#endregion", true);
                for this_word in multi_word_name {
                    self.print_token(this_word, true);
                }
                self.backspace();
            }
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
                if self.on_whitespace_line() {
                    return;
                }
                if self.output.len() == 0 {
                    return;
                }

                self.backspace();

                self.print(NEWLINE, false);
            }
        }
    }

    fn print_expr(&mut self, expr: &'a ExprBox<'a>) {
        match &**expr {
            Expr::Call {
                procedure_name,
                comments_and_newlines_after_lparen,
                arguments,
            } => {
                self.print_expr(procedure_name);
                self.backspace();

                self.print(LPAREN, false);
                let did_move =
                    self.print_comments_and_newlines(comments_and_newlines_after_lparen, IndentationMove::Right);

                let mut iter = arguments.into_iter().peekable();
                while let Some((first_comments, this_argument, these_comments)) = iter.next() {
                    self.print_comments_and_newlines(first_comments, IndentationMove::Stay);
                    self.print_expr(this_argument);
                    self.backspace();

                    self.print_comments_and_newlines(these_comments, IndentationMove::Stay);

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
                self.print_comments_and_newlines(comments_and_newlines_between_l_and_op, IndentationMove::Stay);
                self.print_token(operator, true);
                self.print_comments_and_newlines(comments_and_newlines_between_r_and_op, IndentationMove::Stay);
                self.print_expr(right);
            }

            Expr::Grouping {
                expressions,
                comments_and_newlines_after_lparen,
                comments_and_newlines_after_rparen,
            } => {
                self.print(LPAREN, false);
                let did_move =
                    self.print_comments_and_newlines(comments_and_newlines_after_lparen, IndentationMove::Right);
                for expression in expressions {
                    self.print_expr(expression);
                }
                self.backspace();
                if did_move {
                    if self.on_whitespace_line() {
                        self.backspace_till_newline();
                        self.print_indentation(IndentationMove::Left);
                    } else {
                        self.print_newline(IndentationMove::Left);
                    }
                }
                self.print(RPAREN, true);
                self.print_comments_and_newlines(comments_and_newlines_after_rparen, IndentationMove::Stay);
                self.backspace();
            }

            Expr::ArrayLiteral {
                comments_and_newlines_after_lbracket,
                arguments,
            } => {
                self.print("[", false);
                let did_move =
                    self.print_comments_and_newlines(comments_and_newlines_after_lbracket, IndentationMove::Right);

                let mut iter = arguments.into_iter().peekable();
                while let Some((initial_comment, this_argument, trailing_comment)) = iter.next() {
                    self.print_comments_and_newlines(initial_comment, IndentationMove::Stay);
                    self.print_expr(this_argument);
                    self.backspace();

                    self.print_comments_and_newlines(trailing_comment, IndentationMove::Stay);

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
                self.print_comments_and_newlines(comments, IndentationMove::Stay);
            }

            Expr::NumberStartDot {
                literal_token,
                comments,
            } => {
                self.print("0", false);
                self.print_token(&literal_token, true);
                self.print_comments_and_newlines(comments, IndentationMove::Stay);
            }

            Expr::NumberEndDot {
                literal_token,
                comments,
            } => {
                self.print_token(&literal_token, false);
                self.print("0", true);
                self.print_comments_and_newlines(comments, IndentationMove::Stay);
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
                self.print_comments_and_newlines(comments_and_newlines_between_op_and_r, IndentationMove::Stay);
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
                self.print_comments_and_newlines(comments_and_newlines_between_op_and_r, IndentationMove::Stay);
                self.print_expr(right);
            }
            Expr::Identifier { name, comments } => {
                self.print_token(name, true);
                self.print_comments_and_newlines(comments, IndentationMove::Stay);
            }

            Expr::DotAccess {
                object_name,
                instance_variable,
            } => {
                if self.can_replace_handler {
                    self.can_replace_handler = false;
                    self.force_indentation = Some(IndentationMove::Right);

                    self.print_expr(object_name);
                    self.backspace();
                    self.print(".", false);

                    self.print_expr(instance_variable);

                    self.force_indentation = None;
                    self.can_replace_handler = true;
                } else {
                    self.print_expr(object_name);
                    self.backspace();
                    self.print(".", false);

                    self.print_expr(instance_variable);
                }
            }
            Expr::DataStructureAccess {
                ds_name,
                access_type,
                access_exprs,
            } => {
                self.print_expr(ds_name);
                self.backspace();

                self.print_token(&access_type, access_type.token_type != TokenType::LeftBracket);

                let mut iter = access_exprs.into_iter().peekable();
                while let Some((comments, expr)) = iter.next() {
                    self.print_comments_and_newlines(comments, IndentationMove::Stay);
                    self.print_expr(expr);
                    self.backspace();

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                    }
                }

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
                    self.print_comments_and_newlines(comments_and_newlines_after_q, IndentationMove::Right);
                }
                self.print_expr(left);
                self.print(":", true);
                let did_move =
                    self.print_comments_and_newlines(comments_and_newlines_after_colon, IndentationMove::Right);
                self.print_expr(right);

                if did_move {
                    self.indentation -= 1;
                }
            }

            Expr::Newline { newlines } => {
                let mut start = 0;
                if self.do_not_print_single_newline_statement {
                    start = 1;
                }
                for _ in start..newlines.len() {
                    self.print_newline(IndentationMove::Stay);
                }
            }
            Expr::UnidentifiedAsLiteral { literal_token } => {
                self.print_token(&literal_token, false);
            }
            Expr::UnexpectedEnd => {}
        }

        self.do_not_print_single_newline_statement = false;
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

    fn backspace_whitespace(&mut self) {
        let mut pos = self.output.len();
        if pos == 0 {
            return;
        };

        pos -= 1;

        while pos != 0 {
            match self.output[pos] {
                NEWLINE | TAB | SPACE => {
                    self.output.remove(pos);
                    pos -= 1;
                }
                _ => break,
            };
        }
    }

    fn backspace(&mut self) {
        let pos = self.output.len();
        if pos != 0 && self.output[pos - 1] == SPACE {
            self.output.remove(pos - 1);
        }
    }

    fn ensure_space(&mut self) {
        if let Some(last_entry) = self.last_entry() {
            if last_entry == SPACE {
                return;
            }
        }
        self.print(SPACE, false);
    }

    fn ensure_newline(&mut self, indentation_move: IndentationMove) {
        if let Some(last_entry) = self.last_entry() {
            if last_entry == NEWLINE {
                return;
            }
        }
        self.print_newline(indentation_move);
    }

    fn last_entry(&mut self) -> Option<&'a str> {
        let pos = self.output.len();
        if pos != 0 {
            Some(self.output[pos - 1])
        } else {
            None
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
        self.set_indentation(indentation_move);

        for _ in 0..self.indentation {
            self.print(TAB, false);
        }
    }

    fn set_indentation(&mut self, indentation_move: IndentationMove) {
        match indentation_move {
            IndentationMove::Right => self.indentation += 1,
            IndentationMove::Stay => {}
            IndentationMove::Left => {
                self.indentation -= 1;
            }
        }
    }

    fn only_newlines(&mut self, vec: &'a Vec<Token<'a>>) -> bool {
        for this_token in vec {
            if let TokenType::Newline(_) = this_token.token_type {
                continue;
            }
            return false;
        }
        true
    }

    fn print_comments_and_newlines(&mut self, vec: &'a Vec<Token<'a>>, indentation_move: IndentationMove) -> bool {
        if self.do_not_print_newline_comments && self.only_newlines(vec) {
            return false;
        }

        if self.do_not_print_single_blankline_comments && self.only_newlines(vec) && vec.len() == 2 {
            return false;
        }

        let mut did_move = false;

        for this_one in vec {
            match this_one.token_type {
                TokenType::Newline(_) => {
                    if did_move {
                        self.print_newline(IndentationMove::Stay);
                    } else {
                        did_move = true;
                        if let Some(indent) = self.force_indentation {
                            self.print_newline(indent);
                            self.force_indentation = None;
                        // } 
                        // else if self.accept_original_indentation {
                        //     self.indentation = original_indentation;
                        //     self.print_newline(IndentationMove::Stay);
                        } else {
                            self.print_newline(indentation_move);
                        }
                    }
                }

                TokenType::Comment(_) | TokenType::MultilineComment(_) => self.print_token(this_one, false),

                _ => {
                    println!(
                        "Printing {} which isn't a newline or comment in a comment_newline section...",
                        this_one
                    );
                    self.print_token(this_one, false);
                }
            }
        }

        did_move
    }

    fn print_semicolon(&mut self, do_it: bool) {
        if do_it {
            self.backspace();
            self.print(SEMICOLON, true);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum IndentationMove {
    Right,
    Stay,
    Left,
}
