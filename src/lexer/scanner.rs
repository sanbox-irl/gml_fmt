use super::error_tokens::*;
use super::lex_token;

pub struct Scanner<'a> {
    input: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Scanner {
        Scanner { input }
    }

    pub fn lex_input(&self) -> Result<Vec<lex_token::Token>, Error<LexError>> {
        Err(Error::new(LexError::Unidentified))
    }
}
