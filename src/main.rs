use std::fs;

mod tokenizer;

use tokenizer::tokenize;

fn main() {

    let content = fs::read_to_string("../test/01noSiblings.curly")
        .unwrap();

    println!("{}", content);

    let mut tokens = Vec::new();
    tokenize(&content, &mut tokens);

    for token in tokens {
        println!("{:?}", token);
    }

}