use super::expressions::*;
use super::lex_token::Token;

pub type StmtBox<'a> = Box<StatementWrapper<'a>>;
#[derive(Debug)]
pub struct DelimitedLines<'a, T> {
    pub lines: Vec<DelimitedLine<'a, T>>,
    pub has_end_delimiter: bool,

}

#[derive(Debug)]
pub struct StatementWrapper<'a> {
    pub statement: Statement<'a>,
    pub has_semicolon: bool,
}

impl<'a> StatementWrapper<'a> {
    pub fn new(statement: Statement<'a>, has_semicolon: bool) -> Box<StatementWrapper<'a>> {
        Box::new(StatementWrapper {
            statement,
            has_semicolon,
        })
    }

    pub fn hold_expr(&self) -> bool {
        if let Statement::ExpresssionStatement { .. } = &self.statement {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    VariableDeclList {
        starting_var_type: Token<'a>,
        comments_after_control_word: CommentsAndNewlines<'a>,
        var_decl: DelimitedLines<'a, VariableDecl<'a>>,
    },
    EnumDeclaration {
        comments_after_control_word: CommentsAndNewlines<'a>,
        name: ExprBox<'a>,
        comments_after_lbrace: CommentsAndNewlines<'a>,
        members: DelimitedLines<'a, ExprBox<'a>>,
    },
    If {
        comments_after_control_word: CommentsAndNewlines<'a>,
        condition: ExprBox<'a>,
        then_branch: StmtBox<'a>,
        comments_between: CommentsAndNewlines<'a>,
        else_branch: Option<StmtBox<'a>>,
    },
    WhileWithRepeat {
        comments_after_control_word: CommentsAndNewlines<'a>,
        token: Token<'a>,
        condition: ExprBox<'a>,
        body: StmtBox<'a>,
    },
    DoUntil {
        comments_after_control_word: CommentsAndNewlines<'a>,
        body: StmtBox<'a>,
        comments_between: CommentsAndNewlines<'a>,
        condition: ExprBox<'a>,
    },
    For {
        comments_after_control_word: CommentsAndNewlines<'a>,
        comments_after_lparen: CommentsAndNewlines<'a>,
        initializer: Option<StmtBox<'a>>,
        comments_after_initializer: CommentsAndNewlines<'a>,
        condition: Option<ExprBox<'a>>,
        comments_after_condition: CommentsAndNewlines<'a>,
        increment: Option<ExprBox<'a>>,
        comments_after_increment: CommentsAndNewlines<'a>,
        comments_after_rparen: CommentsAndNewlines<'a>,
        body: StmtBox<'a>,
    },
    Switch {
        comments_after_control_word: CommentsAndNewlines<'a>,
        condition: ExprBox<'a>,
        comments_after_lbrace: CommentsAndNewlines<'a>,
        cases: Vec<Case<'a>>,
    },
    ExpresssionStatement {
        expression: ExprBox<'a>,
    },
    Block {
        comments_after_lbrace: CommentsAndNewlines<'a>,
        statements: Vec<StmtBox<'a>>,
    },
    Return {
        expression: Option<ExprBox<'a>>,
    },
    Break,
    Exit,
    Comment {
        comment: Token<'a>,
    },
    MultilineComment {
        multiline_comment: Token<'a>,
    },
    RegionBegin(Token<'a>),
    RegionEnd(Token<'a>),
    Macro(Token<'a>),
    Define {
        comments_after_control_word: CommentsAndNewlines<'a>,
        script_name: ExprBox<'a>,
        body: Vec<StmtBox<'a>>,
    },
}

#[derive(Debug)]
pub struct Case<'a> {
    pub control_word: CaseType<'a>,
    pub comments_after_control_word: CommentsAndNewlines<'a>,
    pub comments_after_colon: CommentsAndNewlines<'a>,
    pub statements: Vec<StmtBox<'a>>,
}

#[derive(Debug)]
pub enum CaseType<'a> {
    Case(ExprBox<'a>),
    Default,
}

#[derive(Debug)]
pub struct VariableDecl<'a> {
    pub var_expr: ExprBox<'a>,
    pub say_var: Option<Token<'a>>,
    pub say_var_comments: Option<CommentsAndNewlines<'a>>,
}

#[derive(Debug)]
pub struct DelimitedLine<'a, T> {
    pub expr: T,
    pub trailing_comment: CommentsAndNewlines<'a>,
}
