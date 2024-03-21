use std::fs;

pub fn lexer(contents: String) -> Vec<String> {
    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    lines
}

pub fn parse(file: String) -> Vec<String> {
    let contents: String = fs::read_to_string(file).expect("Error to read or open the file.");
    let tokens = lexer(contents);
    tokens
}
