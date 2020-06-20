use crate::lexer::{Token, TokenType, Location};
use crate::tag::{Tag, Tree, Node};
use std::vec::IntoIter;
use std::fmt;

type Result = std::result::Result<Tree, Error>;

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

#[derive(Copy, Clone, Debug)]
pub enum ErrorType {
    EOF,
    UnexpectedClose,
    StartWithoutTag,
    ExpectedIdName,
    ExpectedString,
    ExpectedCloseString,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorType::*;
        let s = match self {
            EOF => "Unexpected end of file",
            UnexpectedClose => "Unexpected close tag",
            StartWithoutTag => "Cannot start tag without name",
            ExpectedIdName => "Expected id name",
            ExpectedString => "Expected string",
            ExpectedCloseString => "Expected quotes to close string",
        };
        write!(f, "{}", s)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result {
    let root = Node::new(Tag::new("html".to_string()));
    let mut token_iter = tokens.into_iter();

    let tree = parse_children(root, &mut token_iter)?;

    Ok(tree)
}

fn parse_children(
    mut node: Node,
    mut token_iter: &mut IntoIter<Token>,
) -> Result {
    while let Some(token) = token_iter.next() {
        use TokenType::*;

        match &token.val {
            CloseCurly => {
                break;
            }
            Literal(s) => {
                let child = parse_tag(s, &mut token_iter)?;
                node.add_child(child);
            }
            DoubleQuote => {
                let child = parse_literal(&token, token_iter)?;
                node.add_child(child);
                let next_token = token_iter.next();
                match next_token {
                    Some(Token { val: TokenType::DoubleQuote, .. }) => {}
                    Some(a) => return Err(Error::new(a.loc, ErrorType::ExpectedCloseString)),
                    None => return Err(Error::new(token.loc, ErrorType::EOF))
                }
            }
            _ => unimplemented!(),
        }
    }

    Ok(Tree::Node(node))
}

fn eat(expected: Token, token_iter: &mut IntoIter<Token>) {
    
}

fn parse_literal(last_token: &Token, token_iter: &mut IntoIter<Token>) -> Result {
    if let Some(token) = token_iter.next() {
        match token.val {
            TokenType::Literal(s) => Ok(Tree::Literal(s)),
            _ => Err(Error::new(token.loc, ErrorType::ExpectedString)),
        }
    } else {
        Err(Error::new(last_token.loc, ErrorType::EOF))
    }
}

fn parse_tag(tag_name: &str, token_iter: &mut IntoIter<Token>) -> Result {
    let mut tag = Tag::new(tag_name.to_string());

    while let Some(token) = token_iter.next() {
        use TokenType::*;

        match &token.val {
            OpenCurly => {
                let node = Node::new(tag);
                let tree = parse_children(node, token_iter)?;
                return Ok(tree);
            }
            Hash => {
                match token_iter.next() {
                    Some(Token { val: Literal(id_name), .. }) => {
                        tag.add_id(id_name);
                    },
                    Some(token) => return Err(Error::new(token.loc, ErrorType::ExpectedIdName)),
                    None => return Err(Error::new(token.loc, ErrorType::EOF)),
                }
            }
            _ => unimplemented!(),
        }
    }
    unreachable!();
}
