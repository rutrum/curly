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

#[derive(Debug)]
enum CharType {
    Whitespace,
    Alphabetic,
    Punctuation,
}

impl From<char> for CharType {
    fn from(c: char) -> Self {
        use CharType::*;
        if c.is_ascii_whitespace() {
            Whitespace
        } else {
            if let TokenType::Literal(_) = TokenType::from(c.to_string()) {
                Alphabetic
            } else {
                Punctuation
            }
        } 
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum LiteralBlock {
    Parens,
    Quotes,
    Not,
}

pub fn tokenize(content: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut buf_col = 0;
    let mut literal = LiteralBlock::Not;

    for (line, line_raw) in content.split('\n').enumerate().map(|(l, r)| (l + 1, r)) {
        for (col, c) in line_raw.chars().enumerate().map(|(col, c)| (col + 1, c)) {
            use CharType::*;
            use LiteralBlock::*;
            match (c.into(), literal) {
                (Whitespace, Not) => {
                    // whitespace not in literal block
                    if !buf.is_empty() {
                        tokens.push(Token::new(buf.to_string(), line, buf_col));
                        buf = String::new();
                    }
                }
                (Whitespace, _) => {
                    // whitespace in literal block
                    if buf.is_empty() {
                        buf_col = col;
                    }
                    buf.push(c);
                }
                (Alphabetic, _) => {
                    // alphabetic in any block
                    if buf.is_empty() {
                        buf_col = col;
                    }
                    buf.push(c);
                }
                (Punctuation, Not) => {
                    // symbol not in a literal block, is it quote or parens?
                    if c == '(' {
                        literal = Parens;
                    } else if c == '"'{
                        literal = Quotes;
                    }

                    if !buf.is_empty() {
                        tokens.push(Token::new(buf.to_string(), line, buf_col));
                        buf = String::new();
                    }
                    let t = Token::new(c.to_string(), line, col);
                    tokens.push(t);
                }
                (Punctuation, Parens) => {
                    // symbol in a parens block, is it parens?
                    if c == ')' {
                        literal = Not;
                        if !buf.is_empty() {
                            tokens.push(Token::new(buf.to_string(), line, buf_col));
                            buf = String::new();
                        }
                        let t = Token::new(c.to_string(), line, col);
                        tokens.push(t);
                    } else {
                        if buf.is_empty() {
                            buf_col = col;
                        }
                        buf.push(c);
                    }
                }
                (Punctuation, Quotes) => {
                    // symbol in a quotes block, is it also quotes?
                    if c == '"' {
                        literal = Not;
                        if !buf.is_empty() {
                            tokens.push(Token::new(buf.to_string(), line, buf_col));
                            buf = String::new();
                        }
                        let t = Token::new(c.to_string(), line, col);
                        tokens.push(t);
                    } else {
                        if buf.is_empty() {
                            buf_col = col;
                        }
                        buf.push(c);
                    }
                }
            }
        }
    }
    tokens
}
