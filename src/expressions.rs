use super::lex_token::*;
pub type ExprBox<'a> = Box<(Expr<'a>, CommentsAndNewlines<'a>)>;
pub type CommentsAndNewlines<'a> = Vec<Token<'a>>;
pub type Arguments<'a> = Vec<(CommentsAndNewlines<'a>,ExprBox<'a>,CommentsAndNewlines<'a>)>;

#[derive(Debug)]
pub enum Expr<'a> {
    // @jack need to handle bad arguments
    Call {
        procedure_name: ExprBox<'a>,
        comments_and_newlines_after_lparen: CommentsAndNewlines<'a>,
        arguments: Arguments<'a>,
    },
    Binary {
        left: ExprBox<'a>,
        comments_and_newlines_between_l_and_op: CommentsAndNewlines<'a>,
        operator: Token<'a>,
        comments_and_newlines_between_r_and_op: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Grouping {
        comments_and_newlines_after_lparen: CommentsAndNewlines<'a>,
        expression: ExprBox<'a>,
        comments_and_newlines_before_rparen: CommentsAndNewlines<'a>,
    },
    ArrayLiteral {
        comments_and_newlines_after_lbracket: CommentsAndNewlines<'a>,
        arguments: Arguments<'a>,
    },
    Literal {
        literal_token: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    Unary {
        operator: Token<'a>,
        // comments_and_newlines_between: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Prefix {
        operator: Token<'a>,
        // comments_and_newlines_between: CommentsAndNewlines<'a>,
        expr: ExprBox<'a>,
    },
    Postfix {
        operator: Token<'a>,
        // comments_and_newlines_between: CommentsAndNewlines<'a>,
        expr: ExprBox<'a>,
    },
    Logical {
        left: ExprBox<'a>,
        operator: Token<'a>,
        comments_and_newlines_between_op_and_r: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Assign {
        left: ExprBox<'a>,
        operator: Token<'a>,
        comments_and_newlines_between_op_and_r: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Identifier {
        name: Token<'a>,
        comments: CommentsAndNewlines<'a>,
    },
    DotAccess {
        object_name: ExprBox<'a>,
        instance_variable: ExprBox<'a>,
    },
    DataStructureAccess {
        ds_name: ExprBox<'a>,
        access_type: Token<'a>,
        comments_and_newlines_between_access_and_expr: CommentsAndNewlines<'a>,
        access_expr: ExprBox<'a>,
    },
    GridDataStructureAccess {
        ds_name: ExprBox<'a>,
        access_type: Token<'a>,
        comments_and_newlines_between_access_type_and_row_expr: CommentsAndNewlines<'a>,
        row_expr: ExprBox<'a>,
        comments_and_newlines_after_comma: CommentsAndNewlines<'a>,
        column_expr: ExprBox<'a>,
    },
    // x ? y : z;
    Ternary {
        conditional: ExprBox<'a>,
        comments_and_newlines_after_q: CommentsAndNewlines<'a>,
        left: ExprBox<'a>,
        comments_and_newlines_after_colon: CommentsAndNewlines<'a>,
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
