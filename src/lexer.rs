#[derive(Debug)]
pub struct Token {
    pub val: TokenType,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(val: String, line: usize, col: usize) -> Self {
        Token {
            val: val.into(),
            line,
            col,
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    Literal(String),
    OpenCurly,
    CloseCurly,
    OpenParens,
    CloseParens,
    Caret,
    Hash,
    Dot,
}

impl From<String> for TokenType {
    fn from(s: String) -> Self {
        use TokenType::*;
        match s.as_str() {
            "{" => OpenCurly,
            "}" => CloseCurly,
            "(" => OpenParens,
            ")" => CloseParens,
            "^" => Caret,
            "#" => Hash,
            "." => Dot,
            _ => Literal(s),
        }
    }
}

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
        } else if c.is_ascii_punctuation() && c != '-' {
            Punctuation
        } else {
            //c.is_ascii_alphabetic() {
            Alphabetic
        }
    }
}

pub fn tokenize(content: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    let mut buf_col = 0;
    let mut literal = false;

    for (line, line_raw) in content.split("\n").enumerate().map(|(l, r)| (l + 1, r)) {
        for (col, c) in line_raw.chars().enumerate().map(|(col, c)| (col + 1, c)) {
            use CharType::*;
            match c.into() {
                Whitespace => {
                    if literal {
                        if buf.is_empty() {
                            buf_col = col;
                        }
                        buf.push(c);
                    }
                }
                Alphabetic => {
                    if buf.is_empty() {
                        buf_col = col;
                    }
                    buf.push(c);
                }
                Punctuation => {
                    if !buf.is_empty() {
                        tokens.push(Token::new(buf.to_string(), line, buf_col));
                        buf = String::new();
                    }

                    let t = Token::new(c.to_string(), line, col);
                    tokens.push(t);

                    if c == '(' {
                        literal = true;
                    } else if c == ')' {
                        literal = false;
                    }
                }
            }
        }
    }
    tokens
}
