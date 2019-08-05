use super::lex_token::*;
type ExprBox<'a> = Box<Expr<'a>>;
type CommentsAndNewlines<'a> = Vec<Token<'a>>;
#[derive(Debug)]
pub enum Expr<'a> {
    Call {
        procedure_name: ExprBox<'a>,
        comments_and_newlines_before_lparen: CommentsAndNewlines<'a>,
        comments_and_newlines_after_lparen: CommentsAndNewlines<'a>,
        arguments: Vec<(CommentsAndNewlines<'a>, ExprBox<'a>, CommentsAndNewlines<'a>)>,
    },
    Binary {
        left: ExprBox<'a>,
        // comments_and_newlines_between_l_and_op: CommentsAndNewlines<'a>,
        operator: Token<'a>,
        // comments_and_newlines_between_r_and_op: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Grouping {
        // comments_and_newlines_after_lparen: CommentsAndNewlines<'a>,
        expression: ExprBox<'a>,
        // comments_and_newlines_before_rparen: CommentsAndNewlines<'a>,
    },
    Literal {
        literal_token: Token<'a>,
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
        // comments_and_newlines_between_l_and_op: CommentsAndNewlines<'a>,
        operator: Token<'a>,
        // comments_and_newlines_between_r_and_op: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Assign {
        left: ExprBox<'a>,
        // comments_and_newlines_between_l_and_op: CommentsAndNewlines<'a>,
        operator: Token<'a>,
        // comments_and_newlines_between_r_and_op: CommentsAndNewlines<'a>,
        right: ExprBox<'a>,
    },
    Identifier {
        name: Token<'a>,
    },
    DotAccess {
        object_name: ExprBox<'a>,
        // comments_and_newlines_between: CommentsAndNewlines<'a>,
        instance_variable: Token<'a>,
    },
    DataStructureAccess {
        ds_name: ExprBox<'a>,
        // comments_and_newlines_between_name_and_access: CommentsAndNewlines<'a>,
        access_type: Token<'a>,
        // comments_and_newlines_between_access_and_expr: CommentsAndNewlines<'a>,
        access_expr: ExprBox<'a>,
        // comments_and_newlines_before_rbracket: CommentsAndNewlines<'a>,
    },
    GridDataStructureAccess {
        ds_name: ExprBox<'a>,
        // comments_and_newlines_between_name_and_access: CommentsAndNewlines<'a>,
        access_type: Token<'a>,
        // comments_and_newlines_between_access_type_and_row_expr: CommentsAndNewlines<'a>,
        row_expr: ExprBox<'a>,
        // comments_and_newlines_between_row_expr_and_column: CommentsAndNewlines<'a>,
        column_expr: ExprBox<'a>,
        // comments_and_newlines_before_rbracket: CommentsAndNewlines<'a>,
    },
    Ternary {
        conditional: ExprBox<'a>,
        // comments_and_newlines_after_conditions: CommentsAndNewlines<'a>,
        left: ExprBox<'a>,
        // comments_and_newlines_between_l_and_r: CommentsAndNewlines<'a>,
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
