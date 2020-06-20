use std::fs;

use curly::lexer;
use curly::parse;

fn main() {
    let content = fs::read_to_string("tests/06strings.curly").unwrap();
    println!("{}", content);

    let tokens = lexer::tokenize(content);
    println!("{:?}", tokens);
    parse::parse(tokens).unwrap_or_else(|e| panic!("{}", e)).to_html();
}
