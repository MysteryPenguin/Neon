use std::str;

pub fn lexer(contents: String) -> Vec<&'static str> {
    contents
        .lines()
        .filter(|&line| !line.trim().is_empty())
        .collect()
}
