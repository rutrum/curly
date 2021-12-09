pub mod token;
pub use token::{TokenType, Location, Token};

#[derive(Debug)]
enum CharType {
    Whitespace,
    Alphabetic,
    Punctuation,
    StartParens,
    EndParens,
    Quotes,
}

impl From<char> for CharType {
    fn from(c: char) -> Self {
        use CharType::*;
        if c.is_ascii_whitespace() {
            Whitespace
        } else if let TokenType::Literal(_) = TokenType::from(c.to_string()) {
            Alphabetic
        } else {
            match c {
                '(' => StartParens,
                ')' => EndParens,
                '\"' => Quotes,
                _ => Punctuation,
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum LiteralBlock {
    ParensBlock,
    QuotesBlock,
    Not,
}

struct TokenBuilder {
    tokens: Vec<Token>,
    buf: String,
    col: usize,
}

impl TokenBuilder {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            buf: String::new(),
            col: 0,
        }
    }

    pub fn push_buf(&mut self, c: char, col: usize) {
        if self.buf.is_empty() {
            self.col = col;
        }
        self.buf.push(c);
    }

    pub fn flush_buf(&mut self, line: usize) {
        self.col = 0;
        let b = std::mem::replace(&mut self.buf, String::new());
        if !b.is_empty() {
            self.tokens.push(Token::new(b, line, self.col));
        }
    }

    pub fn add_token(&mut self, c: char, line: usize, col: usize) {
        if !self.buf.is_empty() {
            self.flush_buf(line)
        }
        let t = Token::new(c.to_string(), line, col);
        self.tokens.push(t);
    }

    pub fn into_vec(self) -> Vec<Token> {
        self.tokens
    }
}

pub fn tokenize(content: String) -> Vec<Token> {
    let mut builder = TokenBuilder::new();
    let mut literal = LiteralBlock::Not;

    for (line, line_raw) in content.split('\n').enumerate().map(|(l, r)| (l + 1, r)) {
        for (col, c) in line_raw.chars().enumerate().map(|(col, c)| (col + 1, c)) {
            use CharType::*;
            use LiteralBlock::*;
            match (c.into(), literal) {
                (Whitespace, Not) => {
                    // whitespace not in literal block
                    builder.flush_buf(line);
                }
                (StartParens, Not) => {
                    // Start of parens block
                    literal = ParensBlock;
                    builder.add_token(c, line, col);
                }
                (Quotes, Not) => {
                    // Start of quotes block
                    literal = QuotesBlock;
                    builder.add_token(c, line, col);
                }
                (Punctuation, Not) => {
                    // Not blocking punc not in a block
                    builder.add_token(c, line, col);
                }
                (EndParens, ParensBlock) | (Quotes, QuotesBlock) => {
                    // Ending a literal block
                    literal = Not;
                    builder.add_token(c, line, col);
                }
                (_, _) => {
                    // Some symbol, letter, or whitespace in literal block
                    builder.push_buf(c, col);
                }
            }
        }
    }
    builder.into_vec()
}
