use super::lex_token::*;

#[derive(Debug)]
pub enum Expr<'a> {
    Binary {
        left: Option<Box<Expr<'a>>>,
        operator: Token<'a>,
        right: Option<Box<Expr<'a>>>,
    },
    Grouping {
        expression: Option<Box<Expr<'a>>>,
    },
    Literal {
        literal_token: Token<'a>,
    },
    Unary {
        operator: Token<'a>,
        right: Option<Box<Expr<'a>>>,
    },
}
