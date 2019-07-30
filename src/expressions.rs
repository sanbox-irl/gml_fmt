use super::lex_token::*;

#[derive(Debug)]
pub enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        literal_token: Token<'a>,
    },
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Assign {
        left: Box<Expr<'a>>,
        right: Box<Expr<'a>>
    },
    Identifier {
        name: Token<'a>
    },
    UnidentifiedAsLiteral {
        literal_token: Token<'a>
    },
    UnexpectedEnd,
}
