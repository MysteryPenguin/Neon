use std::env;
use crate::lexer::lexer as lexers;

pub mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = lexers::parse(args[1].clone());
    for line_content in content {
        println!("{:?}", line_content);
    }
}