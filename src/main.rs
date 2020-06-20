use std::fs;

use curly::lexer;
use curly::parse;

fn main() {
    let content = fs::read_to_string("tests/03id.curly").unwrap();
    println!("{}", content);

    let tokens = lexer::tokenize(content);
    parse::parse(tokens).unwrap().to_html();
}
