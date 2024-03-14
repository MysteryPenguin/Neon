use std::{fs, iter};

pub fn lexer(contents: String) -> Vec<Vec<String>> {
    let lines: Vec<Vec<String>> = contents
        .split('\n')
        .map(|line| line.split(' ').map(String::from).collect())
        .collect();
    lines
}

pub fn parse(file: String) -> Vec<Vec<String>> {
    let contents: String = fs::read_to_string(file).expect("Error to read or open the file.");
    let tokens = lexer(contents);
    tokens
}
