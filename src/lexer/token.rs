use std::fmt;

#[derive(Debug)]
pub struct Token {
    pub val: TokenType,
    pub loc: Location,
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(val: String, line: usize, col: usize) -> Self {
        Token {
            val: val.into(),
            loc: Location { line, col },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenType {
    Literal(String),
    OpenCurly,
    CloseCurly,
    OpenParens,
    CloseParens,
    DoubleQuote,
    Caret,
    Hash,
    Dot,
    Slash,
}

impl From<String> for TokenType {
    fn from(s: String) -> Self {
        use TokenType::*;
        match s.as_str() {
            "{" => OpenCurly,
            "}" => CloseCurly,
            "(" => OpenParens,
            ")" => CloseParens,
            "\"" => DoubleQuote,
            "^" => Caret,
            "#" => Hash,
            "." => Dot,
            "/" => Slash,
            _ => Literal(s),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenType::*;
        let s = match self {
            Literal(s) => s.as_str(),
            OpenCurly => "{",
            CloseCurly => "}",
            OpenParens => "(",
            CloseParens => ")",
            DoubleQuote => "\"",
            Caret => "^",
            Hash => "#",
            Dot => ".",
            Slash => "/",
        };
        write!(f, "{}", s)
    }
}
