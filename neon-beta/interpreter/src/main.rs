use std::env;
use crate::lexer::parse;
use std::str;
pub mod lexer;

fn main() {
  let args: Vec<String> = env::args().collect();
  let content: Vec<&str> = parse::parse(&args[0].to_string());
  println!("{:?}", content);
}
