use std::collections::HashMap;

mod symbol;
use symbol::Symbol;
use symbol::Token;

fn to_symbol<'a>(sym: &char) -> Result<Symbol, ()> {
    let mut map = HashMap::new();
    map.insert('{', Symbol::LeftCurly);
    map.insert('}', Symbol::RightCurly);
    
    match map.get(sym) {
        Some(&a) => Ok(a),
        None => Err(()),
    }
}

pub fn tokenize(file: &String, tokens: &mut Vec<Token>) {

    let chars = file.chars();

    for c in chars {
        
        // Remove whitespace characters
        if c.is_whitespace() {continue}

        // Try to parse as symbol
        // Otherwise 
        match to_symbol(&c) {
            Ok(a) => tokens.push(Token::Symbol(a)),
            Err(_) => {
                chars.next();
            }
        }
    }
}