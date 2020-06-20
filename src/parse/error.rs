use crate::lexer::{Location, TokenType};
use crate::tag::Tree;

use std::fmt;

pub type Result = std::result::Result<Tree, Error>;

#[derive(Debug)]
pub struct Error {
    loc: Location,
    error: ErrorType,
}

impl Error {
    pub fn new(loc: Location, error: ErrorType) -> Self {
        Self { loc, error }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} at line {}, col {}",
            self.error, self.loc.line, self.loc.col
        )
    }
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    EOF,
    UnexpectedClose,
    StartWithoutTag,
    ExpectedIdName,
    ExpectedClassName,
    ExpectedString,
    ExpectedLiteral,
    ExpectedCloseString,
    ExpectedToken(TokenType),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorType::*;
        let s = match self {
            EOF => "Unexpected end of file".to_string(),
            UnexpectedClose => "Unexpected close tag".to_string(),
            StartWithoutTag => "Cannot start tag without name".to_string(),
            ExpectedIdName => "Expected id name".to_string(),
            ExpectedClassName => "Expected class name".to_string(),
            ExpectedString => "Expected string".to_string(),
            ExpectedCloseString => "Expected quotes to close string".to_string(),
            ExpectedLiteral => "Expected literal".to_string(),
            ExpectedToken(token) => format!("Expected {}", token),
        };
        write!(f, "{}", s)
    }
}
