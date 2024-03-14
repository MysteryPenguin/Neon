use std::env;
use crate::lexer::parse;

pub mod lexer;

fn main() {
    let file = env::args().nth(1).expect("No file specified");
    let content = parse::parse(&file);
    println!("{:?}", content);
}