mod ast;

use crate::lexer::token::*;

struct Parser {
  input: Vec<Token>,
  cursor: usize,
}

impl Parser {
  fn new(input: Vec<Token>) -> Self {
    Self { input: input, cursor: 0 }
  }

  fn advance(&mut self) {
    self.cursor += 1;
  }
}
