use super::expressions::*;

#[derive(Debug)]
pub enum Statement<'a> {
    VariableDecl {
        var_expr: Box<Expr<'a>>
    },
    VariableDeclAssignment {
        var_expr: Box<Expr<'a>>,
        assignment: Box<Expr<'a>>
    },
    Expresssion {
        expression: Box<Expr<'a>>
    }
}