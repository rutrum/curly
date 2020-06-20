use crate::lexer::{Token, TokenType};
use crate::tag::{Tag, TagTree};
use std::vec::IntoIter;
use std::fmt;

type Result = std::result::Result<TagTree, Error>;

#[derive(Debug)]
pub struct Error {
    token: Token,
    error: ErrorType,
}

impl Error {
    pub fn new(token: Token, error: ErrorType) -> Self {
        Self { token, error }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} at line {}, col {}",
            self.error, self.token.line, self.token.col
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ErrorType {
    EOF,
    UnexpectedClose,
    StartWithoutTag,
    ExpectedIdName
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorType::*;
        match self {
            EOF => write!(f, "Unexpected end of file"),
            UnexpectedClose => write!(f, "Unexpected close tag"),
            StartWithoutTag => write!(f, "Cannot start tag without name"),
            ExpectedIdName => write!(f, "Expected id name"),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result {
    let root = TagTree::new(Tag::new("html".to_string()));
    let mut token_iter = tokens.into_iter();

    let tree = parse_children(root, &mut token_iter)?;

    Ok(tree)
}

fn parse_children(
    mut root: TagTree,
    mut token_iter: &mut IntoIter<Token>,
) -> Result {
    while let Some(token) = token_iter.next() {
        use TokenType::*;

        match &token.val {
            OpenCurly => {}
            CloseCurly => {
                return Ok(root);
            }
            Literal(s) => {
                let child = parse_tag(s, &mut token_iter)?;
                root.add_child(child);
            }
            _ => unimplemented!(),
        }
    }

    Ok(root)
}

fn parse_tag(tag_name: &str, token_iter: &mut IntoIter<Token>) -> Result {
    let mut tag = Tag::new(tag_name.to_string());

    while let Some(token) = token_iter.next() {
        use TokenType::*;

        match &token.val {
            OpenCurly => {
                let tree = TagTree::new(tag);
                return parse_children(tree, token_iter);
            }
            Hash => {
                match token_iter.next() {
                    Some(Token { val: Literal(id_name), .. }) => {
                        tag.add_id(id_name);
                    },
                    Some(token) => return Err(Error::new(token, ErrorType::ExpectedIdName)),
                    None => return Err(Error::new(token, ErrorType::EOF)),
                }
            }
            _ => unimplemented!(),
        }
    }
    unreachable!();
}
