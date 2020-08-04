use super::expressions::*;
use super::lex_token::{Token, TokenType};
use super::statements::*;
use super::LangConfig;
use bitflags;

type StmtBox<'a> = Box<StatementWrapper<'a>>;

const SPACE: &str = " ";
const TAB: &str = "\t";
const NEWLINE: &str = "\n";
const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";
const COMMA: &str = ",";
const SEMICOLON: &str = ";";

pub struct Printer<'a> {
    output: Vec<&'a str>,
    lang_config: &'a LangConfig,
    indentation: usize,
    do_not_print_single_newline_statement: bool,
    block_instructions: Vec<BlockInstruction>,
    group_instructions: Vec<GroupInstruction>,
    user_indentation_instructions: Vec<usize>,
    do_dot_indent: bool,
    in_a_for_loop: Vec<()>,
    do_not_need_semicolon: Vec<()>,
}

impl<'a> Printer<'a> {
    pub fn new(size: usize, lang_config: &'a LangConfig) -> Printer<'a> {
        Printer {
            output: Vec::with_capacity(size),
            lang_config,
            indentation: 0,
            do_not_print_single_newline_statement: false,
            block_instructions: Vec::new(),
            group_instructions: Vec::new(),
            user_indentation_instructions: Vec::new(),
            do_dot_indent: true,
            in_a_for_loop: Vec::new(),
            do_not_need_semicolon: Vec::new(),
        }
    }

    pub fn get_output(self, size: usize) -> String {
        let mut output = String::with_capacity(size);

        for this_one in self.output {
            output.push_str(this_one);
        }

        output
    }

    pub fn autoformat(mut self, ast: &'a [StmtBox<'a>]) -> Printer {
        for this_statement in ast {
            self.print_statement(this_statement);
        }

        // Make sure we only have one blank line:
        // this is our emergency break!
        let mut pos = self.output.len();
        if pos != 0 {
            pos -= 1;
            loop {
                match self.output[pos] {
                    SPACE | TAB | NEWLINE => {
                        self.output.remove(pos);
                        if pos == 0 {
                            break;
                        } else {
                            pos -= 1;
                        }
                    }

                    _ => {
                        for _ in 0..self.lang_config.newlines_at_end {
                            self.print(NEWLINE, false);
                        }
                        break;
                    }
                };
            }
        }
        self
    }

    fn print_statement(&mut self, stmt: &'a StatementWrapper<'a>) {
        match &stmt.statement {
            Statement::VariableDeclList {
                starting_var_type,
                comments_after_control_word,
                var_decl: var_decl_list,
            } => {
                self.print_token(starting_var_type, true);

                let already_started_indent = self.print_comments_and_newlines(
                    comments_after_control_word,
                    CommentAndNewlinesInstruction::new(IndentationMove::Right, LeadingNewlines::One),
                );

                // let mut forced_semicolon_already = false;
                let mut indented_vars = already_started_indent;
                let mut iter = var_decl_list.lines.iter().peekable();
                let mut interrupt_eol_formatting = false;
                while let Some(delimited_var) = iter.next() {
                    if let Some(var_token) = &delimited_var.expr.say_var {
                        self.print_token(&var_token, true);

                        if let Some(comments) = &delimited_var.expr.say_var_comments {
                            let did_move = self.print_comments_and_newlines(
                                comments,
                                CommentAndNewlinesInstruction::new(
                                    if indented_vars {
                                        IndentationMove::Stay
                                    } else {
                                        IndentationMove::Right
                                    },
                                    LeadingNewlines::One,
                                ),
                            );

                            if did_move {
                                indented_vars = true;
                            }
                        }
                    };
                    self.allow_user_indentation();
                    self.print_expr(&delimited_var.expr.var_expr);
                    self.backspace();
                    self.rewind_user_indentation();

                    let last_line = iter.peek().is_none();

                    if last_line == false {
                        self.print(COMMA, true);
                    } else {
                        if var_decl_list.has_end_delimiter {
                            self.print(COMMA, true);
                        } else {
                            interrupt_eol_formatting = self.do_not_need_semicolon.len() > 0;

                            if interrupt_eol_formatting == false {
                                self.print(SEMICOLON, true);
                            }
                        }
                    };

                    if let Some(_) = &delimited_var.trailing_comment {
                        self.allow_user_indentation();
                        let did_newlines = self.print_comments_and_newlines(
                            &delimited_var.trailing_comment,
                            CommentAndNewlinesInstruction::new_respect_users(
                                if indented_vars {
                                    IndentationMove::Stay
                                } else {
                                    IndentationMove::Right
                                },
                                LeadingNewlines::All,
                            ),
                        );

                        if did_newlines {
                            indented_vars = true;
                        } else {
                            self.rewind_user_indentation();
                        }
                    }
                }

                if indented_vars {
                    if self.on_whitespace_line() == false {
                        self.print_newline(IndentationMove::Left);
                    } else {
                        self.backspace_till_newline();
                        self.print_indentation(IndentationMove::Left);
                    }
                }

                if !interrupt_eol_formatting && self.on_whitespace_line() == false && self.in_a_for_loop.is_empty() {
                    self.print_newline(IndentationMove::Stay);
                    self.do_not_print_single_newline_statement = true;
                }

                // let did_newline = if stmt.has_semicolon && !forced_semicolon_already {
                //     self.print_semicolon(true);
                //     false
                // } else {
                //     self.print_newline(if indented_vars {
                //         IndentationMove::Left
                //     } else {
                //         IndentationMove::Stay
                //     });
                //     true
                // };

                // if indented_vars && did_newline == false {
                //     self.print_newline(IndentationMove::Left);
                // }
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
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );

                self.print_expr(name);
                self.print(LBRACE, true);

                let did_move = self.print_comments_and_newlines(
                    comments_after_lbrace,
                    CommentAndNewlinesInstruction::new(IndentationMove::Right, LeadingNewlines::One),
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
                // let final_newlines: Option<&CommentsAndNewlines> = {
                //     if stmt.has_semicolon {
                //         None
                //     } else {
                //         if let Some(trailing_comments) = &expression.trailing_comments {
                //             if Printer::only_newlines(&trailing_comments) {
                //                 Some(&expression.trailing_comments)
                //             } else {
                //                 None
                //             }
                //         } else {
                //             None
                //         }
                //     }
                // };
                self.print_expr(expression);

                // if let Some(trailing_newline) = final_newlines {
                //     self.print_semicolon(true);
                //     self.print_comments_and_newlines(
                //         trailing_newline,
                //         IndentationMove::Stay,
                //         LeadingNewlines::One,
                //         true,
                //     );
                // } else {
                self.print_semicolon(stmt.has_semicolon);
                // }
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
                let must_indent = block_instructions.contains(BlockInstruction::MUST_INDENT)
                    || statements.len() > 1
                    || (statements.len() == 1 && statements[0].hold_expr() == false);
                let did_move = self.print_comments_and_newlines(
                    comments_after_lbrace,
                    CommentAndNewlinesInstruction::new(IndentationMove::Right, LeadingNewlines::One),
                );
                if must_indent && did_move == false {
                    self.print_newline(IndentationMove::Right);
                }
                let did_newline = did_move || must_indent;
                if did_newline == false {
                    self.ensure_space();
                }

                // don't worry about semicolon or newline if only one statement
                if statements.len() == 1 {
                    self.do_not_need_semicolon.push(());
                }

                for stmt in statements {
                    self.print_statement(stmt);

                    if did_newline & stmt.has_semicolon {
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
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
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
                    CommentAndNewlinesInstruction {
                        indentation_move: if forcible_indent {
                            IndentationMove::Left
                        } else {
                            IndentationMove::Stay
                        },
                        leading_newlines: LeadingNewlines::All,
                        respect_user_newline: true,
                        trailing_comment: true,
                    },
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
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
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
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );

                self.block_instructions
                    .push(BlockInstruction::NO_NEWLINE_AFTER_BLOCK | BlockInstruction::MUST_INDENT);
                self.print_statement(body);
                self.print_comments_and_newlines(
                    comments_between,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::None),
                );

                self.backspace_whitespace();
                if self.last_entry().unwrap() == RBRACE {
                    self.ensure_space();
                } else {
                    self.print_newline(IndentationMove::Stay);
                }
                self.print("until", true);
                self.print_expr(condition);
                self.backspace();
                self.print_semicolon_and_newline(stmt.has_semicolon, IndentationMove::Stay);
            }
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
            } => {
                self.print("for", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );
                self.print(LPAREN, false);
                self.in_a_for_loop.push(());

                let did_move = self.print_comments_and_newlines(
                    comments_after_lparen,
                    CommentAndNewlinesInstruction::new_respect_users(IndentationMove::Right, LeadingNewlines::One),
                );

                if let Some(initializer) = initializer {
                    self.print_statement(initializer);
                    self.ensure_space();
                } else {
                    self.print(SEMICOLON, true);
                }
                self.print_comments_and_newlines(
                    comments_after_initializer,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );

                if let Some(condition) = condition {
                    self.print_expr(condition);
                }
                self.backspace();
                self.print(SEMICOLON, true);
                self.print_comments_and_newlines(
                    comments_after_condition,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );

                if let Some(increment) = increment {
                    self.print_expr(increment);
                } else {
                    self.backspace();
                }

                let did_move_final_comment = self.print_comments_and_newlines(
                    comments_after_increment,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );
                if did_move_final_comment == false {
                    self.backspace();
                }

                if did_move {
                    self.print_newline(IndentationMove::Left);
                }
                self.print(RPAREN, true);
                self.in_a_for_loop.pop();

                self.print_comments_and_newlines(
                    comments_after_rparen,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );

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
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );

                self.print_expr(condition);

                self.ensure_space();
                self.print(LBRACE, true);
                let did_newline = self.print_comments_and_newlines(
                    comments_after_lbrace,
                    CommentAndNewlinesInstruction::new(IndentationMove::Right, LeadingNewlines::One),
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
                                CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
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
                        CommentAndNewlinesInstruction::new(IndentationMove::Right, LeadingNewlines::One),
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
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );

                self.print_expr(script_name);
                self.backspace();
                self.print_newline(IndentationMove::Stay);

                for this_stmt in body {
                    self.print_statement(this_stmt);
                }
            }
        }

        // no semicolon

        //         Statement::Comment { comment } => self.print_token(comment, true),
        // Statement::MultilineComment { multiline_comment } => self.print_token(multiline_comment, true),
        // Statement::RegionBegin(comment) | Statement::RegionEnd(comment) | Statement::Macro(comment) => {
        //     self.print_token(comment, false);
        //     self.backspace();
        // }
        // Statement::Define {

        let okay_to_not_have_semicolon = self.do_not_need_semicolon.pop().is_some();

        if stmt.has_semicolon == false && okay_to_not_have_semicolon == false {
            match stmt.statement {
                Statement::Comment { .. }
                | Statement::MultilineComment { .. }
                | Statement::RegionBegin { .. }
                | Statement::RegionEnd { .. }
                | Statement::Macro { .. } => {}

                _ => {
                    // we do this so we *always* print a newline.
                    let mut newlines = self.backspace_whitespace();

                    if let Some(last_entry) = self.last_entry() {
                        match last_entry {
                            RBRACE => {}

                            SEMICOLON => {
                                newlines = usize::max(newlines, 1);
                            }

                            _ => {
                                self.print_semicolon(true);
                                newlines = usize::max(newlines, 1);
                            }
                        }
                        if last_entry != RBRACE && last_entry != SEMICOLON {}
                        for _ in 0..newlines {
                            self.print_newline(IndentationMove::Stay);
                        }
                    }
                }
            };
        };
    }

    fn print_expr(&mut self, expr: &'a ExprBox<'a>) {
        match &expr.expr {
            Expr::Call {
                procedure_name,
                comments_and_newlines_after_lparen,
                arguments,
            } => {
                // For variable functions
                if let Expr::UnidentifiedAsLiteral { literal_token } = procedure_name.expr {
                    if literal_token.token_type == TokenType::Function {
                        self.do_not_need_semicolon.push(());
                    }
                }

                self.print_expr(procedure_name);
                self.backspace();

                self.print(LPAREN, false);
                let did_move = self.print_comments_and_newlines(
                    comments_and_newlines_after_lparen,
                    CommentAndNewlinesInstruction::new_respect_users(IndentationMove::Right, LeadingNewlines::One),
                );

                // for passing lambdas as arguments
                let mut is_function = false;
                let mut iter = arguments.lines.iter().peekable();

                while let Some(delimited_line) = iter.next() {
                    if let Expr::Call { procedure_name, .. } = &delimited_line.expr.expr {
                        if let Expr::UnidentifiedAsLiteral { literal_token } = procedure_name.expr {
                            is_function = literal_token.token_type == TokenType::Function;
                        }
                    }
                }

                self.print_delimited_lines(arguments, COMMA, false, false);
                self.backspace_whitespace();

                if did_move {
                    self.print_newline(IndentationMove::Left);
                }

                if !is_function {
                    self.print(RPAREN, true);
                } else {
                    self.block_instructions.push(BlockInstruction::NO_NEWLINE_AFTER_BLOCK);
                }
            }

            Expr::Function {
                comments_after_control_word,
                call,
                comments_after_rparen,
                is_constructor,
            } => {
                self.print("function", true);
                self.print_comments_and_newlines(
                    comments_after_control_word,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );
                self.print_expr(call);
                if !*is_constructor {
                    self.backspace_whitespace();
                }
                
                self.print_comments_and_newlines(
                    comments_after_rparen,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );

                if *is_constructor {
                    self.print("constructor", true);
                }

                self.do_not_need_semicolon.push(());
            }
            
            Expr::StructOperator {
                token,
                comments_before_expression,
                expression,
            } => {
                self.print_token(token, true);
                self.print_comments_and_newlines(
                    comments_before_expression,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::One),
                );
                self.print_expr(expression);
                self.backspace_whitespace();
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
                self.allow_user_indentation();
                self.print_comments_and_newlines(
                    comments_and_newlines_between_op_and_r,
                    CommentAndNewlinesInstruction::new_respect_users(IndentationMove::Stay, LeadingNewlines::All),
                );
                self.print_expr(right);
                self.rewind_user_indentation();
            }

            Expr::Grouping {
                expressions,
                comments_and_newlines_after_lparen,
                comments_and_newlines_after_rparen,
            } => {
                self.print(LPAREN, false);
                let did_move = self.print_comments_and_newlines(
                    comments_and_newlines_after_lparen,
                    CommentAndNewlinesInstruction::new_respect_users(IndentationMove::Right, LeadingNewlines::One),
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
                    CommentAndNewlinesInstruction {
                        indentation_move: instructions.force_indentation(),
                        leading_newlines: instructions.force_leading_newlines(),
                        respect_user_newline: instructions.force_respect(),
                        trailing_comment: false,
                    },
                );
            }

            Expr::ArrayLiteral {
                comments_and_newlines_after_lbracket,
                arguments,
            } => {
                self.print("[", false);
                let did_move = self.print_comments_and_newlines(
                    comments_and_newlines_after_lbracket,
                    CommentAndNewlinesInstruction::new_respect_users(IndentationMove::Right, LeadingNewlines::One),
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
                self.print_comments_and_newlines(
                    comments,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::All),
                );
            }

            Expr::NumberStartDot {
                literal_token,
                comments,
            } => {
                self.print("0", false);
                self.print_token(&literal_token, true);
                self.print_comments_and_newlines(
                    comments,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::All),
                );
            }

            Expr::NumberEndDot {
                literal_token,
                comments,
            } => {
                self.print_token(&literal_token, false);
                self.print("0", true);
                self.print_comments_and_newlines(
                    comments,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::All),
                );
            }

            Expr::Unary {
                operator,
                comments_and_newlines_between,
                right,
            } => {
                self.print_token(&operator, operator.token_type == TokenType::NotAlias);
                self.print_comments_and_newlines(
                    comments_and_newlines_between,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::All),
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
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::All),
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
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::All),
                );
                self.print_expr(right);
            }
            Expr::Identifier { name, comments } => {
                self.print_token(name, true);
                self.print_comments_and_newlines(
                    comments,
                    CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::All),
                );
            }

            Expr::DotAccess {
                object_name,
                comments_between,
                instance_variable,
            } => {
                self.print_expr(object_name);
                self.backspace();
                self.print(".", false);
                self.allow_user_indentation();

                let mut can_unlock = false;
                let indentation = if self.do_dot_indent {
                    can_unlock = true;
                    self.do_dot_indent = false;
                    IndentationMove::Right
                } else {
                    IndentationMove::Stay
                };
                self.print_comments_and_newlines(
                    comments_between,
                    CommentAndNewlinesInstruction::new_respect_users(indentation, LeadingNewlines::One),
                );
                self.print_expr(instance_variable);
                self.rewind_user_indentation();
                if can_unlock && self.do_dot_indent == false {
                    self.do_dot_indent = true;
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
                    self.allow_user_indentation();
                    self.print_comments_and_newlines(
                        comments,
                        CommentAndNewlinesInstruction::new(IndentationMove::Stay, LeadingNewlines::All),
                    );
                    self.print_expr(expr);
                    self.rewind_user_indentation();
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
                self.allow_user_indentation();

                self.print_comments_and_newlines(
                    comments_and_newlines_after_q,
                    CommentAndNewlinesInstruction::new(IndentationMove::Right, LeadingNewlines::All),
                );

                self.print_expr(left);
                self.print(":", true);
                let did_move = self.print_comments_and_newlines(
                    comments_and_newlines_after_colon,
                    CommentAndNewlinesInstruction::new(IndentationMove::Right, LeadingNewlines::One),
                );
                self.print_expr(right);
                self.rewind_user_indentation();

                if did_move {
                    self.set_indentation(IndentationMove::Left);
                }
            }

            Expr::Newline => {
                self.do_not_need_semicolon.push(());
                if self.do_not_print_single_newline_statement == false {
                    self.print_newline(IndentationMove::Stay);
                }
            }

            Expr::Comment { comment } => {
                self.do_not_need_semicolon.push(());
                self.print_token(comment, false);
            }
            Expr::MultilineComment { multiline_comment } => {
                self.do_not_need_semicolon.push(());
                self.print_token(multiline_comment, false);
            }

            Expr::UnidentifiedAsLiteral { literal_token } => {
                self.print_token(&literal_token, true);

                match literal_token.token_type {
                    TokenType::Constructor => {
                        self.do_not_need_semicolon.push(());
                    }
                    _ => {}
                }

            }
        }

        self.print_comments_and_newlines(
            &expr.trailing_comments,
            CommentAndNewlinesInstruction {
                indentation_move: IndentationMove::Stay,
                leading_newlines: LeadingNewlines::All,
                respect_user_newline: true,
                trailing_comment: true,
            },
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
        if pos < 2 {
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

    fn backspace_whitespace(&mut self) -> usize {
        let mut pos = self.output.len();
        if pos == 0 {
            return 0;
        };

        let mut newline_number = 0;

        pos -= 1;

        while pos != 0 {
            match self.output[pos] {
                NEWLINE => {
                    self.output.remove(pos);
                    pos -= 1;
                    newline_number += 1;
                }
                TAB | SPACE => {
                    self.output.remove(pos);
                    pos -= 1;
                }
                _ => break,
            };
        }
        newline_number
    }

    fn backspace(&mut self) {
        let pos = self.output.len();
        if pos != 0 && self.on_whitespace_line() == false && self.output[pos - 1] == SPACE {
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
        if self.output.len() == 0 || self.prev_line_was_whitespace() {
            return;
        }
        self.backspace();

        self.print(NEWLINE, false);
        self.print_indentation(indentation_move);
    }

    fn print_indentation(&mut self, indentation_move: IndentationMove) {
        self.set_indentation(indentation_move);
        self.print_indentation_final();
    }

    fn print_indentation_raw(&mut self, indent_size: usize) {
        self.indentation = indent_size;
        self.print_indentation_final();
    }

    fn print_indentation_final(&mut self) {
        for _ in 0..self.indentation {
            if self.lang_config.use_spaces {
                for _ in 0..self.lang_config.space_size {
                    self.print(SPACE, false);
                }
            } else {
                self.print(TAB, false);
            }
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

    fn check_indentation(&self, indentation_move: IndentationMove) -> usize {
        match indentation_move {
            IndentationMove::Right => self.indentation + 1,
            IndentationMove::Stay => self.indentation,
            IndentationMove::Left => self.indentation - 1,
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
        instructions: CommentAndNewlinesInstruction,
    ) -> bool {
        if let Some(vec) = vec {
            if vec.len() == 0 || (Printer::only_newlines(&vec) && instructions.respect_user_newline == false) {
                return false;
            }
            let mut did_move = false;
            let mut ignore_newline = instructions.leading_newlines != LeadingNewlines::All;

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
                            if instructions.leading_newlines == LeadingNewlines::None {
                                continue;
                            }
                        }

                        if did_move {
                            self.print_newline(IndentationMove::Stay);
                        } else {
                            did_move = true;

                            // check for a force indentation
                            if self.user_indentation_instructions.is_empty() == false
                                && instructions.respect_user_newline
                                && user_indentation >= self.check_indentation(instructions.indentation_move)
                            {
                                if self.prev_line_was_whitespace() {
                                    self.backspace_till_newline();
                                } else {
                                    self.backspace();
                                    self.print(NEWLINE, false);
                                }
                                self.print_indentation_raw(user_indentation);
                            } else {
                                self.print_newline(instructions.indentation_move);
                            }
                        }
                    }

                    TokenType::Comment(_) | TokenType::MultilineComment(_) => {
                        if instructions.trailing_comment {
                            self.do_not_need_semicolon.push(());
                        }
                        self.ensure_space();
                        self.print_token(&this_one, false);
                        ignore_newline = false;
                    }

                    TokenType::RegionEnd(_) | TokenType::RegionBegin(_) => {
                        if instructions.trailing_comment {
                            self.do_not_need_semicolon.push(())
                        }
                        self.ensure_space();
                        self.print_token(&this_one, true);
                        ignore_newline = false;
                    }

                    TokenType::Then => {
                        if instructions.trailing_comment {
                            self.do_not_need_semicolon.push(())
                        }
                        self.ensure_space();
                        self.print_token(&this_one, true);
                        ignore_newline = false;
                    }

                    _ => {
                        println!(
                            "Printing {} which isn't newline, comment, or region in a comment_newline section...",
                            this_one
                        );
                        if instructions.trailing_comment {
                            self.do_not_need_semicolon.push(())
                        }
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
            TokenType::Tilde => "~",

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
            TokenType::GlobalVar => "globalvar",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
            TokenType::For => "for",
            TokenType::Function => "function",
            TokenType::Constructor => "constructor",
            TokenType::New => "new",
            TokenType::Delete => "delete",
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
                    CommentAndNewlinesInstruction::new_respect_users(IndentationMove::Stay, LeadingNewlines::All),
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

    fn print_semicolon_and_newline(&mut self, do_it: bool, indentation_move: IndentationMove) -> bool {
        if do_it {
            self.print_semicolon(true);
            false
        } else {
            self.print_semicolon(true);
            self.print_newline(indentation_move);
            true
        }
    }

    fn allow_user_indentation(&mut self) {
        self.user_indentation_instructions.push(self.indentation);
    }

    fn rewind_user_indentation(&mut self) {
        self.indentation = self.user_indentation_instructions.pop().unwrap();
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
        const MUST_INDENT               = 0b00000010;
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

struct CommentAndNewlinesInstruction {
    indentation_move: IndentationMove,
    leading_newlines: LeadingNewlines,
    respect_user_newline: bool,
    trailing_comment: bool,
}

impl CommentAndNewlinesInstruction {
    pub fn new(indentation_move: IndentationMove, leading_newlines: LeadingNewlines) -> Self {
        CommentAndNewlinesInstruction {
            indentation_move,
            leading_newlines,
            ..Default::default()
        }
    }

    pub fn new_respect_users(indentation_move: IndentationMove, leading_newlines: LeadingNewlines) -> Self {
        CommentAndNewlinesInstruction {
            indentation_move,
            leading_newlines,
            respect_user_newline: true,
            ..Default::default()
        }
    }
}

impl Default for CommentAndNewlinesInstruction {
    fn default() -> Self {
        CommentAndNewlinesInstruction {
            indentation_move: IndentationMove::Stay,
            leading_newlines: LeadingNewlines::One,
            respect_user_newline: false,
            trailing_comment: false,
        }
    }
}
