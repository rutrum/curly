use std::fs;
use std::collections::HashMap;

fn main() {

    let content = fs::read_to_string("./test/01noSiblings.curly")
        .unwrap();

    println!("{}", content);

    let tokens = tokenizer::tokenize(&content);

    for token in tokens {
        println!("{:?}", token);
    }

}

#[derive(Debug)]
pub enum Token<'a> {
    Name(String),
    Symbol(&'a Symbol),
}

#[derive(Debug)]
pub enum Symbol {
    LeftCurly,
    RightCurly,
}

#[macro_use]
pub mod tokenizer {

    #[macro_use]
    use super::*;

    fn to_symbol(sym: &char) -> Result<&Symbol, ()> {
        let mut map = HashMap::new();
        map.insert('{', &Symbol::LeftCurly);
        map.insert('}', &Symbol::RightCurly);
        
        match map.get(sym) {
            Some(a) => Ok(a),
            None => Err(()),
        }
    }

    pub fn tokenize(file: &String, tokens: mut Vec<Token>) {

        for c in file.chars() {
            
            // Remove whitespace characters
            if c.is_whitespace() {continue}

            // Try to parse as symbol
            // Otherwise 
            match to_symbol(&c) {
                Ok(a) => tokens.push(Token::Symbol(a)),
                Err(_) => {
                    continue
                }
            }
        }
    }
    
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}
