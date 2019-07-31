use super::lex_token::*;
type ExprBox<'a> = Box<Expr<'a>>;

#[derive(Debug)]
pub enum Expr<'a> {
    Call {
        procedure_name: ExprBox<'a>,
        arguments: Vec<ExprBox<'a>>,
    },
    Binary {
        left: ExprBox<'a>,
        operator: Token<'a>,
        right: ExprBox<'a>,
    },
    Grouping {
        expression: ExprBox<'a>,
    },
    Literal {
        literal_token: Token<'a>,
    },
    Unary {
        operator: Token<'a>,
        right: ExprBox<'a>,
    },
    Prefix {
        operator: Token<'a>,
        expr: ExprBox<'a>
    },
    Postfix {
        operator: Token<'a>,
        expr: ExprBox<'a>
    },
    Logical {
        left: ExprBox<'a>,
        operator: Token<'a>,
        right: ExprBox<'a>,
    },
    Assign {
        left: ExprBox<'a>,
        right: ExprBox<'a>,
    },
    Identifier {
        name: Token<'a>,
    },
    DotAccess {
        object_name: ExprBox<'a>,
        instance_variable: Token<'a>,
    },
    DataStructureAccess {
        ds_name: ExprBox<'a>,
        access_type: Token<'a>,
        access_expr: ExprBox<'a>,
    },
    GridDataStructureAccess {
        ds_name: ExprBox<'a>,
        access_type: Token<'a>,
        row_expr: ExprBox<'a>,
        column_expr: ExprBox<'a>,
    },
    Ternary {
        conditional: ExprBox<'a>,
        left: ExprBox<'a>,
        right: ExprBox<'a>,
    },

    Newline {
        token: Token<'a>,
    },
    UnidentifiedAsLiteral {
        literal_token: Token<'a>,
    },
    UnexpectedEnd,
}
