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

  fn token(&self) -> Token {
    if self.cursor >= self.input.len() {
      return Token { content: "".chars().collect(), kind: TokenKind::End, pos: self.cursor };
    }

    self.input[self.cursor].clone()
  }

  fn peek(&self) -> Token {
    if self.cursor >= self.input.len() {
      return Token { content: "".chars().collect(), kind: TokenKind::End, pos: self.cursor };
    }

    self.input[self.cursor]
  }

  fn next_statement(&mut self) -> ast::Stmt {

  }
}
