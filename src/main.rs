mod lexer;
mod parser;
mod util;

use std::env;
use std::fs;
use std::process::exit;

fn main() {
    //util::print_error("main.st", 5, 11, "No value in constant declaration", "This error happens when you declare a constant variable and don't bind a value to it.\nConstants are declared at compile time, so they must have an initial value.", "const int myconst", lexer::token::Token { content: "myconst".chars().collect(), kind: lexer::token::TokenKind::Identifier, pos: 11 }, "this variable doesn't have a value", vec![], "Consider binding a value to this variable.", "const int myconst = 0")
    // star <file>

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: star <file>");
    }

    if let Some(file) = args.get(1) {
        let contents = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(_) => {
                println!("File '{}' doesn't exist. Check if you typed the name correctly.", file);
                exit(1);
            }
        };

        let tokens = lexer::lex(contents);
    }
}
