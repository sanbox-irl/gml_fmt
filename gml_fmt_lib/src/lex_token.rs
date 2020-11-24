#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType<'a> {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Slash,
    Backslash,
    Star,
    Mod,
    Hashtag,

    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    BitXorEquals,
    BitOrEquals,
    BitAndEquals,
    ModEquals,

    ListIndexer,
    MapIndexer,
    GridIndexer,
    ArrayIndexer,

    Minus,
    Plus,
    Incrementer,
    Decrementer,
    Bang,
    Hook,
    Tilde,

    LessThanGreaterThan,

    LogicalAnd,
    LogicalOr,
    LogicalXor,
    BitAnd,
    BitOr,
    BitXor,
    BitLeft,
    BitRight,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Macro(&'a str),
    RegionBegin(&'a str),
    RegionEnd(&'a str),
    Define,

    Var,
    GlobalVar,
    If,
    Else,
    Function,
    Constructor,
    New,
    Delete,
    Return,
    For,
    Repeat,
    With,
    While,
    Do,
    Until,
    Switch,
    Case,
    DefaultCase,
    Break,
    Exit,
    Enum,

    AndAlias,
    OrAlias,
    XorAlias,
    NotAlias,
    ModAlias,
    Div,
    Then,

    Newline(usize),
    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),
    NumberStartDot(&'a str),
    NumberEndDot(&'a str),

    Comment(&'a str),
    MultilineComment(&'a str),

    UnidentifiedInput(&'a str),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub line_number: u32,
    pub column_number: u32,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, line_number: u32, column_number: u32) -> Token {
        Token {
            token_type,
            line_number,
            column_number,
        }
    }
}

use std::fmt;
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {:#?} on {}:{}.",
            self.token_type, self.line_number, self.column_number
        )
    }
}
