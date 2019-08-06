use super::expressions::*;
use super::lex_token::Token;

pub type StmtBox<'a> = Box<StatementWrapper<'a>>;

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
}

#[derive(Debug)]
pub enum Statement<'a> {
    VariableDeclList {
        var_decl: Vec<VariableDecl<'a>>,
    },
    EnumDeclaration {
        name: ExprBox<'a>,
        members: Vec<EnumMemberDecl<'a>>,
    },
    ExpresssionStatement {
        expression: ExprBox<'a>,
    },
    Block {
        statements: Vec<StmtBox<'a>>,
    },
    If {
        condition: ExprBox<'a>,
        then_branch: StmtBox<'a>,
        else_branch: Option<StmtBox<'a>>,
    },
    While {
        condition: ExprBox<'a>,
        body: StmtBox<'a>,
    },
    Repeat {
        condition: ExprBox<'a>,
        body: StmtBox<'a>,
    },
    For {
        initializer: Option<StmtBox<'a>>,
        condition: Option<ExprBox<'a>>,
        increment: Option<ExprBox<'a>>,
        body: StmtBox<'a>,
    },
    Return {
        expression: Option<ExprBox<'a>>,
    },
    Break,
    Exit,
    Switch {
        condition: ExprBox<'a>,
        cases: Option<Vec<Case<'a>>>,
        default: Option<Vec<Case<'a>>>,
    },
    Comment {
        comment: Token<'a>,
    },
    MultilineComment {
        multiline_comment: Token<'a>,
    },
    RegionBegin {
        multi_word_name: Vec<Token<'a>>,
    },
    RegionEnd,
    Macro {
        macro_body: Vec<Token<'a>>,
    },
    Define {
        script_name: ExprBox<'a>,
        body: Vec<StmtBox<'a>>,
    },
    EOF
}

#[derive(Debug)]
pub struct Case<'a> {
    pub constant: ExprBox<'a>,
    pub statements: Vec<StmtBox<'a>>,
}

#[derive(Debug)]
pub struct VariableDecl<'a> {
    pub var_expr: ExprBox<'a>,
    pub assignment: Option<ExprBox<'a>>,
}

#[derive(Debug)]
pub struct EnumMemberDecl<'a> {
    pub name: Token<'a>,
    pub value: Option<ExprBox<'a>>,
}
