use super::expressions::*;

#[derive(Debug)]
pub enum Statement<'a> {
    VariableDecl {
        var_expr: Box<Expr<'a>>,
        assignment: Option<Box<Expr<'a>>>
    },
    VariableDeclList {
        var_decl: Vec<Box<Statement<'a>>>
    },
    Expresssion {
        expression: Box<Expr<'a>>
    }
}