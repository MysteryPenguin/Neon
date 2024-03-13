use std::fs;
use std::str;
use crate::lexer::lexer;

pub fn parse(file: &'static String) -> Vec<&'static str> {
    let binding = fs::read_to_string(file).expect("Unable to read the file! Maybe the file does not exist?");
    let contents: &str = binding.as_str().copy();
    let tokens = lexer::lexer(contents);
    return tokens;
}