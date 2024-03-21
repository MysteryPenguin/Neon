use std::fs;

pub fn lexer(contents: String) -> Vec<Vec<String>> {
    let mut counter = 0;
    let lines: Vec<Vec<String>> = contents
        .split('\n')
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut result = Vec::new();
            let mut in_quotes = false;
            let mut current_word = String::new();
            let mut escaped = false;
            for char in line.chars() {
                if char == '"' && !escaped {
                    counter += 1;
                    if in_quotes {
                        in_quotes = false;
                        result.push(current_word.clone());
                        current_word.clear();
                    } else {
                        in_quotes = true;
                        current_word.push('"');
                    }
                    if counter % 2 == 0 {
                      break;
                    }
                } else if char == '\\' && !escaped {
                    escaped = true;
                } else if char == ' ' && !in_quotes {
                    if !current_word.is_empty() {
                        result.push(current_word.clone());
                        current_word.clear();
                    }
                } else {
                    current_word.push(char);
                    escaped = false;
                }
            }
            if !current_word.is_empty() {
                result.push(current_word);
            }
            if in_quotes && result.is_empty() && result[0].starts_with("\"") {
                result[0] = format!("\"{}", result[0]);
            }
            if !in_quotes && !result.is_empty() && result[0].starts_with("\"") {
                result[0] = format!("{}\"", result[0]);
            }
            result
        })
        .collect();
    for char in &lines {}
    lines
}

pub fn parse(file: String) -> Vec<Vec<String>> {
    let contents: String = fs::read_to_string(file).expect("Error to read or open the file.");
    let tokens = lexer(contents);
    tokens
}
