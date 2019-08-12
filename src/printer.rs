use super::expressions::*;
use super::lex_token::{Token, TokenType};
use super::statements::*;
use bitflags;

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
    do_not_print_single_newline_statement: bool,
    block_instructions: Vec<BlockInstruction>,
    group_instructions: Vec<GroupInstruction>,
    user_indentation_instructions: Vec<usize>,
}

impl<'a> Printer<'a> {
    pub fn new(size: usize) -> Printer<'a> {
        Printer {
            output: Vec::with_capacity(size),
            indentation: 0,
            do_not_print_single_newline_statement: false,
            block_instructions: Vec::new(),
            group_instructions: Vec::new(),
            user_indentation_instructions: Vec::new(),
        }
    }

    pub fn get_output(vec_output: &Vec<&'a str>, size: usize) -> String {
        let mut output = String::with_capacity(size);

        for this_one in vec_output {
            output.push_str(this_one);
        }

        output
    }

    pub fn autoformat(mut self, ast: &'a [StmtBox<'a>]) -> Printer {
        for this_statement in ast {
            self.print_statement(this_statement);
        }

        // Print Ending Newline
        if self.on_whitespace_line() || self.output.len() == 0 {
            return self;
        }
        self.backspace();
        self.print(NEWLINE, false);

        self
    }

    fn print_statement(&mut self, stmt: &'a StatementWrapper<'a>) {
        match &stmt.statement {
            Statement::VariableDeclList {
                comments_after_control_word,
                var_decl: var_decl_list,
            } => {
                self.print("var", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    IndentationMove::Stay,
                    LeadingNewlines::One,
                    false,
                );

                let mut indented_vars = false;
                let mut iter = var_decl_list.lines.iter().peekable();
                while let Some(delimited_var) = iter.next() {
                    if delimited_var.expr.say_var {
                        self.print("var", true);

                        if let Some(comments) = &delimited_var.expr.say_var_comments {
                            let did_move = self.print_comments_and_newlines(
                                comments,
                                if indented_vars {
                                    IndentationMove::Stay
                                } else {
                                    IndentationMove::Right
                                },
                                LeadingNewlines::One,
                                false,
                            );

                            if did_move {
                                indented_vars = true;
                            }
                        }
                    };
                    self.print_expr(&delimited_var.expr.var_expr);
                    self.backspace();

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                        false
                    } else {
                        if var_decl_list.has_end_delimiter {
                            self.print(COMMA, true);
                        }
                        true
                    };

                    if let Some(comment) = &delimited_var.trailing_comment {
                        if comment.len() != 0 {
                            let did_newlines = self.print_comments_and_newlines(
                                &delimited_var.trailing_comment,
                                if indented_vars {
                                    IndentationMove::Stay
                                } else {
                                    IndentationMove::Right
                                },
                                LeadingNewlines::One,
                                true,
                            );

                            if did_newlines {
                                indented_vars = true;
                            }
                        }
                    }
                }

                self.print_semicolon_and_newline(
                    stmt.has_semicolon,
                    if indented_vars {
                        IndentationMove::Left
                    } else {
                        IndentationMove::Stay
                    },
                );
            }
            Statement::EnumDeclaration {
                comments_after_control_word,
                name,
                comments_after_lbrace,
                members,
            } => {
                self.print("enum", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    IndentationMove::Stay,
                    LeadingNewlines::One,
                    false,
                );

                self.print_expr(name);
                self.print(LBRACE, true);

                let did_move = self.print_comments_and_newlines(
                    comments_after_lbrace,
                    IndentationMove::Right,
                    LeadingNewlines::One,
                    false,
                );
                if did_move == false {
                    self.print_newline(IndentationMove::Right);
                }
                self.backspace();
                self.print_delimited_lines(members, COMMA, true, true);

                self.set_indentation(IndentationMove::Left);
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
                if self.on_whitespace_line() == false {
                    self.ensure_space();
                }

                let block_instructions = if self.block_instructions.is_empty() {
                    BlockInstruction::NONE
                } else {
                    self.block_instructions.pop().unwrap()
                };

                self.print(LBRACE, false);

                // if we have more than one statement, or if our statement isn't an expression statement, then we indent.
                let must_indent = statements.len() > 1 || (statements.len() == 1 && statements[0].hold_expr() == false);
                let did_move = self.print_comments_and_newlines(
                    comments_after_lbrace,
                    IndentationMove::Right,
                    LeadingNewlines::One,
                    false,
                );
                if must_indent && did_move == false {
                    self.print_newline(IndentationMove::Right);
                }

                let did_newline = did_move || must_indent;
                if did_newline == false {
                    self.ensure_space();
                }

                for stmt in statements {
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
                    self.backspace();
                    if self.last_entry().unwrap() != "{" {
                        self.ensure_space();
                    }
                }

                self.print(RBRACE, false);
                self.print_semicolon(stmt.has_semicolon);

                if block_instructions.contains(BlockInstruction::NO_NEWLINE_AFTER_BLOCK) == false {
                    self.ensure_newline(IndentationMove::Stay);
                }

                self.do_not_print_single_newline_statement = true;
            }
            Statement::If {
                comments_after_control_word,
                condition,
                then_branch,
                comments_between,
                else_branch,
            } => {
                self.print("if", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    IndentationMove::Stay,
                    LeadingNewlines::One,
                    false,
                );

                let has_block = if let Statement::Block { .. } = &then_branch.statement {
                    self.block_instructions.push(BlockInstruction::NO_NEWLINE_AFTER_BLOCK);
                    true
                } else {
                    false
                };

                let current_indentation = self.indentation;
                if has_block == false {
                    if let Expr::Grouping { .. } = &condition.expr {
                        self.group_instructions.push(GroupInstruction {
                            force_respect: Some(true),
                            force_indentation: Some(IndentationMove::Right),
                            ..Default::default()
                        });
                    }
                }
                self.print_expr(condition);
                let forcible_indent = self.indentation != current_indentation && has_block == false;
                self.print_statement(then_branch);

                let did_move = self.print_comments_and_newlines(
                    comments_between,
                    if forcible_indent {
                        IndentationMove::Left
                    } else {
                        IndentationMove::Stay
                    },
                    LeadingNewlines::All,
                    true,
                );
                if did_move == false {
                    self.print_newline(if forcible_indent {
                        IndentationMove::Left
                    } else {
                        IndentationMove::Stay
                    });
                }

                if let Some(else_branch) = else_branch {
                    if forcible_indent == false {
                        self.backspace_whitespace();
                    }
                    self.ensure_space();
                    self.print("else", true);
                    self.print_statement(else_branch);
                }
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::WhileWithRepeat {
                token,
                condition,
                body,
                comments_after_control_word,
            } => {
                self.print_token(token, true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    IndentationMove::Stay,
                    LeadingNewlines::One,
                    false,
                );

                self.print_expr(condition);

                self.print_statement(body);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::DoUntil {
                comments_after_control_word,
                condition,
                comments_between,
                body,
            } => {
                self.print("do", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    IndentationMove::Stay,
                    LeadingNewlines::One,
                    false,
                );

                self.block_instructions.push(BlockInstruction::NO_NEWLINE_AFTER_BLOCK);
                self.print_statement(body);
                self.print_comments_and_newlines(comments_between, IndentationMove::Stay, LeadingNewlines::None, false);

                self.ensure_space();
                self.print("until", true);
                self.print_expr(condition);
                self.backspace();
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::For {
                comments_after_control_word,
                comments_after_lparen,
                initializer,
                condition,
                increment,
                trailing_comments,
                body,
            } => {
                self.print("for", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    IndentationMove::Stay,
                    LeadingNewlines::One,
                    false,
                );
                self.print(LPAREN, false);

                let did_move = self.print_comments_and_newlines(
                    comments_after_lparen,
                    IndentationMove::Right,
                    LeadingNewlines::One,
                    true,
                );

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
                if did_move {
                    self.print_newline(IndentationMove::Left);
                }
                self.print(RPAREN, true);
                self.print_comments_and_newlines(trailing_comments, IndentationMove::Stay, LeadingNewlines::One, false);

                self.print_statement(body);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Return { expression } => {
                self.print("return", false);

                if let Some(expression) = expression {
                    self.print(SPACE, false);
                    self.print_expr(expression);
                }
                self.print_semicolon_and_newline(stmt.has_semicolon, IndentationMove::Stay);
            }
            Statement::Break => {
                self.print("break", false);
                self.print_semicolon_and_newline(stmt.has_semicolon, IndentationMove::Stay);
            }

            Statement::Exit => {
                self.print("exit", false);
                self.print_semicolon_and_newline(stmt.has_semicolon, IndentationMove::Stay);
            }
            Statement::Switch {
                comments_after_control_word,
                condition,
                comments_after_lbrace,
                cases,
            } => {
                self.print("switch", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    IndentationMove::Stay,
                    LeadingNewlines::One,
                    false,
                );

                self.print_expr(condition);

                self.ensure_space();
                self.print(LBRACE, true);
                let did_newline = self.print_comments_and_newlines(
                    comments_after_lbrace,
                    IndentationMove::Right,
                    LeadingNewlines::One,
                    false,
                );
                if did_newline == false {
                    self.print_newline(IndentationMove::Right);
                }

                for case in cases {
                    match &case.control_word {
                        CaseType::Case(case_constant) => {
                            self.print("case", true);
                            self.print_comments_and_newlines(
                                &case.comments_after_control_word,
                                IndentationMove::Stay,
                                LeadingNewlines::One,
                                false,
                            );

                            self.print_expr(case_constant);
                        }

                        CaseType::Default => {
                            self.print("default", true);
                        }
                    }
                    self.backspace();
                    self.print(":", true);
                    let saved_indentation = self.indentation;
                    let did_move = self.print_comments_and_newlines(
                        &case.comments_after_colon,
                        IndentationMove::Right,
                        LeadingNewlines::One,
                        false,
                    );
                    if did_move == false {
                        self.print_newline(IndentationMove::Right);
                    }
                    // @jack do we handle blocks here in a special way?
                    for this_statement in &case.statements {
                        self.print_statement(this_statement);
                    }

                    self.backspace_till_newline();
                    self.print_indentation_raw(saved_indentation);
                }

                self.backspace_whitespace();
                self.print_newline(IndentationMove::Left);

                self.print(RBRACE, false);
                self.print_semicolon(stmt.has_semicolon);
            }
            Statement::Comment { comment } => self.print_token(comment, true),
            Statement::MultilineComment { multiline_comment } => self.print_token(multiline_comment, true),
            Statement::RegionBegin(comment) | Statement::RegionEnd(comment) | Statement::Macro(comment) => {
                self.print_token(comment, false);
                self.backspace();
            }
            Statement::Define {
                comments_after_control_word,
                script_name,
                body,
            } => {
                self.print("#define", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    IndentationMove::Stay,
                    LeadingNewlines::One,
                    false,
                );

                self.print_expr(script_name);
                self.backspace();

                for this_stmt in body {
                    self.print_statement(this_stmt);
                }
            }
        }
    }

    fn print_expr(&mut self, expr: &'a ExprBox<'a>) {
        match &expr.expr {
            Expr::Call {
                procedure_name,
                comments_and_newlines_after_lparen,
                arguments,
            } => {
                self.print_expr(procedure_name);
                self.backspace();

                self.print(LPAREN, false);
                let did_move = self.print_comments_and_newlines(
                    comments_and_newlines_after_lparen,
                    IndentationMove::Right,
                    LeadingNewlines::One,
                    true,
                );

                self.print_delimited_lines(arguments, COMMA, false, false);
                self.backspace_whitespace();
                if did_move {
                    self.print_newline(IndentationMove::Left);
                }
                self.print(RPAREN, true);
            }

            Expr::Binary {
                left,
                operator,
                comments_and_newlines_between_op_and_r,
                right,
            } => {
                self.print_expr(left);
                self.ensure_space();
                self.print_token(operator, true);
                self.print_comments_and_newlines(
                    comments_and_newlines_between_op_and_r,
                    IndentationMove::Stay,
                    LeadingNewlines::All,
                    true,
                );
                self.print_expr(right);
            }

            Expr::Grouping {
                expressions,
                comments_and_newlines_after_lparen,
                comments_and_newlines_after_rparen,
            } => {
                self.print(LPAREN, false);
                let did_move = self.print_comments_and_newlines(
                    comments_and_newlines_after_lparen,
                    IndentationMove::Right,
                    LeadingNewlines::One,
                    true,
                );

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
                let instructions = match self.group_instructions.pop() {
                    Some(instruction) => instruction,
                    None => Default::default(),
                };

                self.print_comments_and_newlines(
                    comments_and_newlines_after_rparen,
                    instructions.force_indentation(),
                    instructions.force_leading_newlines(),
                    instructions.force_respect(),
                );

                // if let Some(instruction) =
            }

            Expr::ArrayLiteral {
                comments_and_newlines_after_lbracket,
                arguments,
            } => {
                self.print("[", false);
                let did_move = self.print_comments_and_newlines(
                    comments_and_newlines_after_lbracket,
                    IndentationMove::Right,
                    LeadingNewlines::One,
                    true,
                );

                self.print_delimited_lines(arguments, COMMA, false, false);
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
                self.print_comments_and_newlines(comments, IndentationMove::Stay, LeadingNewlines::All, false);
            }

            Expr::NumberStartDot {
                literal_token,
                comments,
            } => {
                self.print("0", false);
                self.print_token(&literal_token, true);
                self.print_comments_and_newlines(comments, IndentationMove::Stay, LeadingNewlines::All, false);
            }

            Expr::NumberEndDot {
                literal_token,
                comments,
            } => {
                self.print_token(&literal_token, false);
                self.print("0", true);
                self.print_comments_and_newlines(comments, IndentationMove::Stay, LeadingNewlines::All, false);
            }

            Expr::Unary {
                operator,
                comments_and_newlines_between,
                right,
            } => {
                self.print_token(&operator, false);
                self.print_comments_and_newlines(
                    comments_and_newlines_between,
                    IndentationMove::Stay,
                    LeadingNewlines::All,
                    false,
                );
                self.print_expr(right);
            }
            Expr::Postfix {
                operator,
                comments_and_newlines_between,
                expr,
            } => {
                self.print_expr(expr);
                self.backspace();
                self.print_token(&operator, true);
                self.print_comments_and_newlines(
                    comments_and_newlines_between,
                    IndentationMove::Stay,
                    LeadingNewlines::All,
                    false,
                );
            }
            Expr::Assign {
                left,
                operator,
                comments_and_newlines_between_op_and_r,
                right,
            } => {
                self.print_expr(left);
                self.print_token(&operator, true);
                self.print_comments_and_newlines(
                    comments_and_newlines_between_op_and_r,
                    IndentationMove::Stay,
                    LeadingNewlines::All,
                    false,
                );
                self.print_expr(right);
            }
            Expr::Identifier { name, comments } => {
                self.print_token(name, true);
                self.print_comments_and_newlines(comments, IndentationMove::Stay, LeadingNewlines::All, false);
            }

            Expr::DotAccess {
                object_name,
                comments_between,
                instance_variable,
            } => {
                self.print_expr(object_name);
                self.backspace();
                self.print(".", false);
                // self.user_indentation_instructions.push(self.indentation);
                self.print_comments_and_newlines(comments_between, IndentationMove::Stay, LeadingNewlines::One, true);
                self.print_expr(instance_variable);
                // self.indentation = self.user_indentation_instructions.pop().unwrap();
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
                    self.print_comments_and_newlines(comments, IndentationMove::Stay, LeadingNewlines::All, false);
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
                self.print_comments_and_newlines(
                    comments_and_newlines_after_q,
                    IndentationMove::Right,
                    LeadingNewlines::All,
                    false,
                );

                self.print_expr(left);
                self.print(":", true);
                let did_move = self.print_comments_and_newlines(
                    comments_and_newlines_after_colon,
                    IndentationMove::Right,
                    LeadingNewlines::One,
                    false,
                );
                self.print_expr(right);

                if did_move {
                    self.set_indentation(IndentationMove::Left);
                }
            }

            Expr::Newline => {
                if self.do_not_print_single_newline_statement == false {
                    self.print_newline(IndentationMove::Stay);
                }
            }

            Expr::Comment { comment } => self.print_token(comment, false),
            Expr::MultilineComment { multiline_comment } => self.print_token(multiline_comment, false),

            Expr::UnidentifiedAsLiteral { literal_token } => {
                self.print_token(&literal_token, true);
            }
            Expr::UnexpectedEnd => {}
        }

        self.print_comments_and_newlines(
            &expr.trailing_comments,
            IndentationMove::Stay,
            LeadingNewlines::All,
            false,
        );
        self.do_not_print_single_newline_statement = false;
    }

    fn print_token(&mut self, token: &'a Token<'a>, space_after: bool) {
        self.print(Printer::get_token_name(&token.token_type), space_after);
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
            return true;
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
            if last_entry == SPACE || last_entry == TAB || last_entry == NEWLINE {
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

    fn print_indentation_raw(&mut self, indent_size: usize) {
        self.indentation = indent_size;

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

    fn only_newlines(vec: &'a Vec<Token<'a>>) -> bool {
        for this_token in vec {
            if let TokenType::Newline(_) = this_token.token_type {
                continue;
            }
            return false;
        }
        true
    }

    fn print_comments_and_newlines(
        &mut self,
        vec: &'a Option<Vec<Token<'a>>>,
        indentation_move: IndentationMove,
        leading_newlines: LeadingNewlines,
        respect_user_newline: bool,
    ) -> bool {
        if let Some(vec) = vec {
            if vec.len() == 0 || (Printer::only_newlines(&vec) && respect_user_newline == false) {
                return false;
            }
            let mut did_move = false;
            let mut ignore_newline = leading_newlines != LeadingNewlines::All;

            let mut iter = vec.into_iter().peekable();
            while let Some(this_one) = iter.next() {
                match this_one.token_type {
                    TokenType::Newline(user_indentation) => {
                        if ignore_newline {
                            while let Some(next_one) = iter.peek() {
                                if let TokenType::Newline(_) = next_one.token_type {
                                    iter.next();
                                } else {
                                    break;
                                }
                            }
                            ignore_newline = false;
                            if leading_newlines == LeadingNewlines::None {
                                continue;
                            }
                        }

                        if did_move {
                            self.print_newline(IndentationMove::Stay);
                        } else {
                            did_move = true;

                            // check for a force indentation
                            if self.user_indentation_instructions.is_empty() == false
                                && respect_user_newline
                                && user_indentation > self.indentation
                            {
                                self.backspace();
                                self.print(NEWLINE, false);
                                self.print_indentation_raw(user_indentation);
                            } else {
                                self.print_newline(indentation_move);
                            }
                        }
                    }

                    TokenType::Comment(_) | TokenType::MultilineComment(_) => {
                        self.ensure_space();
                        self.print_token(&this_one, false);
                        ignore_newline = false;
                    }

                    TokenType::RegionEnd(_) | TokenType::RegionBegin(_) => {
                        self.ensure_space();
                        self.print_token(&this_one, true);
                        ignore_newline = false;
                    }

                    TokenType::Then => {
                        self.ensure_space();
                        self.print_token(&this_one, true);
                        ignore_newline = false;
                    }

                    _ => {
                        println!(
                            "Printing {} which isn't newline, comment, or region in a comment_newline section...",
                            this_one
                        );
                        self.print_token(&this_one, true);
                    }
                }
            }
            did_move
        } else {
            false
        }
    }

    pub fn get_token_name(token_type: &'a TokenType<'a>) -> &'a str {
        match token_type {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Colon => ":",
            TokenType::Semicolon => ";",
            TokenType::Slash => "/",
            TokenType::Backslash => "\\",
            TokenType::Star => "*",
            TokenType::Mod => "%",
            TokenType::Hashtag => "#",
            TokenType::Then => "then",

            TokenType::ListIndexer => "[|",
            TokenType::MapIndexer => "[?",
            TokenType::GridIndexer => "[#",
            TokenType::ArrayIndexer => "[@",

            TokenType::LessThanGreaterThan => "<>",

            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Incrementer => "++",
            TokenType::Decrementer => "--",
            TokenType::Bang => "!",
            TokenType::Hook => "?",

            TokenType::PlusEquals => "+=",
            TokenType::MinusEquals => "-=",
            TokenType::StarEquals => "*=",
            TokenType::SlashEquals => "/=",
            TokenType::BitXorEquals => "^=",
            TokenType::BitOrEquals => "|=",
            TokenType::BitAndEquals => "&=",
            TokenType::ModEquals => "%=",

            TokenType::LogicalAnd => "&&",
            TokenType::LogicalOr => "||",
            TokenType::LogicalXor => "^^",
            TokenType::BitAnd => "&",
            TokenType::BitOr => "|",
            TokenType::BitXor => "^",
            TokenType::BitLeft => "<<",
            TokenType::BitRight => ">>",
            TokenType::BangEqual => "!=",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",

            TokenType::Define => "#define",

            TokenType::Var => "var",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
            TokenType::For => "for",
            TokenType::Repeat => "repeat",
            TokenType::While => "while",
            TokenType::With => "with",
            TokenType::Do => "do",
            TokenType::Until => "until",
            TokenType::Switch => "switch",
            TokenType::Case => "case",
            TokenType::DefaultCase => "default",
            TokenType::Break => "break",
            TokenType::Exit => "exit",
            TokenType::Enum => "enum",

            TokenType::AndAlias => "and",
            TokenType::OrAlias => "or",
            TokenType::XorAlias => "xor",
            TokenType::NotAlias => "not",
            TokenType::ModAlias => "mod",
            TokenType::Div => "div",
            TokenType::Newline(_) => "\n",

            TokenType::Macro(literal)
            | TokenType::RegionBegin(literal)
            | TokenType::RegionEnd(literal)
            | TokenType::Identifier(literal)
            | TokenType::String(literal)
            | TokenType::Number(literal)
            | TokenType::NumberStartDot(literal)
            | TokenType::NumberEndDot(literal)
            | TokenType::Comment(literal)
            | TokenType::MultilineComment(literal)
            | TokenType::UnidentifiedInput(literal) => literal,
        }
    }

    fn print_delimited_lines(
        &mut self,
        delimited_lines: &'a DelimitedLines<'a, ExprBox<'a>>,
        delimiter: &'static str,
        force_newline_between: bool,
        force_newline_at_end: bool,
    ) {
        let mut iter = delimited_lines.lines.iter().peekable();
        while let Some(delimited_line) = iter.next() {
            self.print_expr(&delimited_line.expr);
            self.backspace();

            let at_end = if let Some(_) = iter.peek() {
                self.print(delimiter, true);
                false
            } else {
                if delimited_lines.has_end_delimiter {
                    self.print(delimiter, true);
                }
                true
            };

            if let Some(_) = &delimited_line.trailing_comment {
                let did_newlines = self.print_comments_and_newlines(
                    &delimited_line.trailing_comment,
                    IndentationMove::Stay,
                    LeadingNewlines::All,
                    true,
                );

                if did_newlines == false && force_newline_between {
                    self.print_newline(IndentationMove::Stay);
                }
            } else {
                if at_end {
                    if force_newline_at_end {
                        self.print_newline(IndentationMove::Stay);
                    }
                } else {
                    if force_newline_between {
                        self.print_newline(IndentationMove::Stay);
                    }
                }
            }
        }
    }

    fn print_semicolon(&mut self, do_it: bool) {
        if do_it {
            self.backspace();
            self.print(SEMICOLON, true);
        }
    }

    fn print_semicolon_and_newline(&mut self, do_it: bool, indentation_move: IndentationMove) {
        if do_it {
            self.print_semicolon(true);
        } else {
            self.print_semicolon(true);
            self.print_newline(indentation_move);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum IndentationMove {
    Right,
    Stay,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum LeadingNewlines {
    All,
    One,
    None,
}

bitflags::bitflags! {
    pub struct BlockInstruction: u8 {
        const NONE                      = 0b00000000;
        const NO_NEWLINE_AFTER_BLOCK    = 0b00000001;
    }
}

#[derive(Default)]
struct GroupInstruction {
    force_indentation: Option<IndentationMove>,
    force_leading_newlines: Option<LeadingNewlines>,
    force_respect: Option<bool>,
}

impl GroupInstruction {
    fn force_indentation(&self) -> IndentationMove {
        if let Some(ret) = self.force_indentation {
            ret
        } else {
            IndentationMove::Stay
        }
    }

    fn force_leading_newlines(&self) -> LeadingNewlines {
        if let Some(ret) = self.force_leading_newlines {
            ret
        } else {
            LeadingNewlines::All
        }
    }

    fn force_respect(&self) -> bool {
        if let Some(ret) = self.force_respect {
            ret
        } else {
            false
        }
    }
}
