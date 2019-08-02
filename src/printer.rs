use super::expressions::*;
use super::lex_token::Token;
use super::statements::*;

type StmtBox<'a> = Box<Statement<'a>>;

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
}

impl<'a> Printer<'a> {
    pub fn new() -> Printer<'a> {
        Printer {
            output: Vec::new(),
            indentation: 0,
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

    fn print_statement(&mut self, stmt: &'a Statement<'a>) {
        match stmt {
            Statement::VariableDeclList { var_decl } => {
                self.print("var", true);

                let mut iter = var_decl.into_iter().peekable();
                while let Some(this_decl) = iter.next() {
                    self.print_expr(&*this_decl.var_expr);

                    if let Some(expr_box) = &this_decl.assignment {
                        self.print("=", true);
                        self.print_expr(expr_box);
                    }
                    self.backspace();

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                    }
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
                        self.print_newline(IndentationMove::Left);
                        self.print(RBRACE, false);
                    }
                }
            }
            Statement::ExpresssionStatement { expression } => {
                self.print_expr(expression);
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
            }
            Statement::While { condition, body } => {
                self.print("while", true);
                self.print_expr_parentheses(condition);
                self.print_statement(body);
            }
            Statement::Repeat { condition, body } => {
                self.print("repeat", true);
                self.print_expr_parentheses(condition);
                self.print_statement(body);
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
            }
            Statement::Return { expression } => {
                self.print("return", false);

                if let Some(expression) = expression {
                    self.print(SPACE, false);
                    self.print_expr(expression);
                }
            }
            Statement::Break => {
                self.print("break", false);
            }
            Statement::Exit => {
                self.print("exit", false);
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
                        self.print_expr(&*this_case.constant);
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
                        self.print_expr(&*this_case.constant);
                        self.backspace();
                        self.print(":", true);

                        for this_case in &this_case.statements {
                            self.print_statement(this_case);
                        }
                    }
                }

                self.print_newline(IndentationMove::Left);
                self.print(RBRACE, false);
            }
            Statement::Comment { comment } => self.print_token(comment, false),
            Statement::MultilineComment { multiline_comment } => {
                self.print_token(multiline_comment, false)
            }
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

                self.indentation += 1;
                for this_stmt in body {
                    self.print_statement(this_stmt);
                }
                self.indentation -= 1;
            }
        }
    }

    fn print_expr(&mut self, expr: &'a Expr<'a>) {
        match expr {
            Expr::Call {
                procedure_name,
                arguments,
            } => {
                self.print_expr(procedure_name);
                self.backspace();
                self.print(LPAREN, false);

                let mut iter = arguments.into_iter().peekable();
                while let Some(this_argument) = iter.next() {
                    self.print_expr(this_argument);
                    self.backspace();

                    if let Some(_) = iter.peek() {
                        self.print(COMMA, true);
                    } else {
                        self.print(RPAREN, false);
                    }
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                self.print_expr(left);
                self.print_token(operator, true);
                self.print_expr(right);
            }
            Expr::Grouping { expression } => {
                self.print(LPAREN, false);
                self.print_expr(expression);
                self.print(RPAREN, true);
            }
            Expr::Literal { literal_token } => self.print_token(literal_token, true),
            Expr::Unary { operator, right } => {
                self.print_token(operator, false);
                self.print_expr(right);
            }
            Expr::Prefix { operator, expr } => {
                self.print_token(operator, false);
                self.print_expr(expr);
            }
            Expr::Postfix { operator, expr } => {
                self.print_expr(expr);
                self.backspace();
                self.print_token(operator, false);
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                self.print_expr(left);
                self.print_token(operator, true);
                self.print_expr(right);
            }
            Expr::Assign { left, right } => {
                self.print_expr(left);
                self.print("=", true);
                self.print_expr(right);
            }
            Expr::Identifier { name } => self.print_token(name, true),

            Expr::DotAccess {
                object_name,
                instance_variable,
            } => {
                self.print_expr(&*object_name);
                self.backspace();
                self.print(".", false);
                self.print_token(instance_variable, true);

                // @jack figure out spaces here...
                // we want a space unless the next is DotAccess...
                // Everything else can take care of themselves.
            }
            Expr::DataStructureAccess {
                ds_name,
                access_type,
                access_expr,
            } => {
                self.print_expr(ds_name);
                self.print_token(access_type, true);
                self.print_expr(access_expr);
            }
            Expr::GridDataStructureAccess {
                ds_name,
                access_type,
                row_expr,
                column_expr,
            } => {
                self.print_expr(ds_name);
                self.print_token(access_type, true);
                self.print_expr(row_expr);

                self.print(COMMA, true);
                self.print_expr(column_expr);
            }
            Expr::Ternary {
                conditional,
                left,
                right,
            } => {
                self.print_expr(conditional);
                self.print("?", true);
                self.print_expr(left);
                self.print(":", true);
                self.print_expr(right);
            }

            Expr::Newline { token: _ } => self.print_newline(IndentationMove::Stay),
            Expr::UnidentifiedAsLiteral { literal_token } => {
                self.print_token(literal_token, false);
            }
            Expr::UnexpectedEnd => {}
        }
    }

    fn print_token(&mut self, token: &'a Token<'a>, space_after: bool) {
        self.print(token.print_name(), space_after);
    }

    fn print(&mut self, this_string: &'a str, space_after: bool) {
        self.output.push(this_string);
        if space_after {
            self.output.push(SPACE);
        }
    }

    fn print_expr_parentheses(&mut self, expr: &'a Expr<'a>) {
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
}

enum IndentationMove {
    Right,
    Stay,
    Left,
}
