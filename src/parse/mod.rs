use crate::lexer::{Token, TokenType};
use crate::tag::{Node, Tag, Tree};

pub mod error;
use error::{Error, ErrorType, Result};

type TokenIter = std::vec::IntoIter<Token>;

pub fn parse(tokens: Vec<Token>) -> Result<Tree> {
    let root = Node::new(Tag::new("html".to_string()));
    let mut token_iter = tokens.into_iter();

    parse_children(root, &mut token_iter)
}

fn parse_children(mut node: Node, mut token_iter: &mut TokenIter) -> Result<Tree> {
    while let Some(token) = token_iter.next() {
        use TokenType::*;
        println!("{:?}", token);

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
                    Some(Token {
                        val: TokenType::DoubleQuote,
                        ..
                    }) => {}
                    Some(a) => return Err(Error::new(a.loc, ErrorType::ExpectedCloseString)),
                    None => return Err(Error::new(token.loc, ErrorType::EOF)),
                }
            }
            _ => unimplemented!(),
        }
    }

    Ok(Tree::Node(node))
}

fn parse_literal(last_token: &Token, token_iter: &mut TokenIter) -> Result<Tree> {
    if let Some(token) = token_iter.next() {
        match token.val {
            TokenType::Literal(s) => Ok(Tree::Literal(s)),
            _ => Err(Error::new(token.loc, ErrorType::ExpectedString)),
        }
    } else {
        Err(Error::new(last_token.loc, ErrorType::EOF))
    }
}

fn parse_tag(tag_name: &str, token_iter: &mut TokenIter) -> Result<Tree> {
    let mut tag = Tag::new(tag_name.to_string());

    while let Some(token) = token_iter.next() {
        use TokenType::*;

        println!("{:?}", token);

        match &token.val {
            OpenCurly => {
                let node = Node::new(tag);
                let tree = parse_children(node, token_iter)?;
                return Ok(tree);
            }
            Hash => tag.add_id(get_next_literal(&token, token_iter)?),
            Dot => tag.add_class(get_next_literal(&token, token_iter)?),
            Caret => {
                let prop = get_next_literal(&token, token_iter)?;
                eat(TokenType::OpenParens, &token, token_iter)?;
                let val = get_next_literal(&token, token_iter)?;
                eat(TokenType::CloseParens, &token, token_iter)?;
                tag.add_style(prop, val);
            }
            Literal(prop) => {
                // For now, I assume it is an attribute
                eat(TokenType::OpenParens, &token, token_iter)?;
                let optv = match token_iter.next() {
                    Some(Token {
                        val: TokenType::Literal(value),
                        ..
                    }) => {
                        eat(TokenType::CloseParens, &token, token_iter)?;
                        Some(value)
                    }
                    Some(Token {
                        val: TokenType::CloseParens,
                        ..
                    }) => None,
                    Some(token) => return Err(Error::new(token.loc, ErrorType::ExpectedLiteral)),
                    None => return Err(Error::new(token.loc, ErrorType::EOF)),
                };
                tag.add_attribute(prop.to_string(), optv);
            }
            _ => unimplemented!(),
        }
    }
    unreachable!();
}

fn get_next_literal(token: &Token, token_iter: &mut TokenIter) -> Result<String> {
    match token_iter.next() {
        Some(Token {
            val: TokenType::Literal(class_name),
            ..
        }) => Ok(class_name),
        Some(token) => Err(Error::new(token.loc, ErrorType::ExpectedLiteral)),
        None => Err(Error::new(token.loc, ErrorType::EOF)),
    }
}

fn eat(
    expected: TokenType,
    last_token: &Token,
    token_iter: &mut TokenIter,
) -> std::result::Result<(), Error> {
    if let Some(token) = token_iter.next() {
        if expected == token.val {
            Ok(())
        } else {
            Err(Error::new(token.loc, ErrorType::ExpectedToken(expected)))
        }
    } else {
        Err(Error::new(last_token.loc, ErrorType::EOF))
    }
}
