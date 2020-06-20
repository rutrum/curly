use std::fs;

use curly::lexer;
use curly::parse;

fn main() {
    let content = fs::read_to_string("tests/07attributes.curly").unwrap();
    println!("{}", content);

    let tokens = lexer::tokenize(content);
    println!("{:#?}", tokens);
    let tree = parse::parse(tokens).unwrap_or_else(|e| panic!("{}", e));
    println!("{}", tree.to_html());
    println!("{}", tree.to_html_min());
}
