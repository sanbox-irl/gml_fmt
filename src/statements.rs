use super::expressions::*;

#[derive(Debug)]
pub enum Statement<'a> {
    VariableDecl {
        var_expr: Box<Expr<'a>>,
        assignment: Option<Box<Expr<'a>>>,
    },
    VariableDeclList {
        var_decl: Vec<Box<Statement<'a>>>,
    },
    Expresssion {
        expression: Box<Expr<'a>>,
    },
    Block {
        statements: Vec<Box<Statement<'a>>>,
    },
    If {
        condition: Box<Expr<'a>>,
        then_branch: Box<Statement<'a>>,
        else_branch: Option<Box<Statement<'a>>>,
    },
    While {
        condition: Box<Expr<'a>>,
        body: Box<Statement<'a>>,
    },
    Repeat {
        condition: Box<Expr<'a>>,
        body: Box<Statement<'a>>,
    },
    For {
        initializer: Option<Box<Statement<'a>>>,
        condition: Option<Box<Expr<'a>>>,
        increment: Option<Box<Expr<'a>>>,
        body: Box<Statement<'a>>,
    },
    Return {
        expression: Option<Box<Expr<'a>>>,
    },
}
