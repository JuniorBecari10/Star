mod lexer;
mod util;

fn main() {
    util::print_error("main.st", 5, 11, "No value in constant declaration", "This error happens when you declare a constant variable and don't bind a value to it.\nConstants are declared at compile time, so they must have an initial value.", "const int myconst", lexer::token::Token { content: "myconst".chars().collect(), kind: lexer::token::TokenKind::Identifier, pos: 11 }, "this variable doesn't have a value", vec![], "Consider binding a value to this variable.", "const int myconst = 0")
}
