use std::fs;

pub fn lexer(contents: String) -> Vec<Vec<String>> {
    let lines: Vec<Vec<String>> = contents
        .split('\n')
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            if line.contains('"') {
                line.split("\"").filter(|s| !s.is_empty()).map(String::from).collect()
            } else {
                line.split(' ').map(String::from).collect()
            }
        })
        .collect();
    lines
}

pub fn parse(file: String) -> Vec<Vec<String>> {
    let contents: String = fs::read_to_string(file).expect("Error to read or open the file.");
    let tokens = lexer(contents);
    tokens
}
