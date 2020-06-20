use std::fs;

use curly::lexer;
use curly::parse;

fn main() {
    let content = fs::read_to_string("tests/05styles.curly").unwrap();
    println!("{}", content);

    let tokens = lexer::tokenize(content);
    let tree = parse::parse(tokens).unwrap_or_else(|e| panic!("{}", e));
    println!("{}", tree.to_html());
    println!("{}", tree.to_html_min());
}
