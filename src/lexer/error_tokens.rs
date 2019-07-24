#[derive(Debug)]
pub enum LexError {
    UnexpectedCharacter(String),
    UnidentifiedError,
}

#[derive(Debug)]
pub struct Error<T> {
    error_type: T,
    line_number: u32,
    // column_number: u32
}

impl<T> Error<T> {
    pub fn new_no_line(error_type: T) -> Error<T> {
        Error {
            error_type,
            line_number: 0
        }
    }

    pub fn new(error_type: T, line_number: u32) -> Error<T> {
        Error {
            error_type,
            line_number,
        }
    }
}

use std::fmt;

impl<LexError> fmt::Display for Error<LexError>
where
    LexError: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lex Error {:?} on line {}.",
            self.error_type, self.line_number
        )
    }
}
