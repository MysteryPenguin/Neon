use std::fs;
use std::str;
use crate::lexer::lexer;

pub fn parse(file: &str) -> Vec<&'static str> {
    let contents = fs::read_to_string(file)
        .expect("Unable to read the file! Maybe the file does not exist?");
    let tokens = lexer::lexer(contents);
    tokens
}