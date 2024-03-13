use std::str;

pub fn lexer(contents: &'static str) -> Vec<&'static str> {
    let lines: Vec<&str> = contents.split('\n').collect();
    return lines;
}

