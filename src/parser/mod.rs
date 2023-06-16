mod ast;

use crate::lexer::token::*;

struct Parser {
  input: Vec<Token>,
  cursor: usize,
  line: i32
}

impl Parser {
  fn new(input: Vec<Token>, line: i32) -> Self {
    Self { input: input, cursor: 0, line: line }
  }

  fn expect_kind(&self, expect: TokenKind) -> bool {
    self.peek().kind == expect
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
    if self.token().kind == TokenKind::VarKw {
      return self.parse_var_stat();
    }

    self.parse_exp_stmt()
  }

  // ---

  fn parse_var_stat(&mut self) -> ast::Stmt {
    let ident: String;
    let is_const = self.token().kind == TokenKind::ConstKw;
    let exp: ast::Exp;

    self.advance();

    if !self.expect_kind(TokenKind::Identifier) {
      return ast::Stmt::ErrorStmt { line: self.line, msg: format!("Expected Identifier, but got {:?} instead.", self.peek().kind) }
    }
    
    self.advance();
    ident = self.token().content.iter().cloned().collect::<String>();

    if !self.expect_kind(TokenKind::Assign) {
      return ast::Stmt::ErrorStmt { line: self.line, msg: format!("Expected Assign, but got {:?} instead.", self.peek().kind) }
    }

    self.advance();

    exp = self.parse_exp();

    ast::Stmt::VarStmt { line: 0, ident: ident, exp: exp, is_const: is_const }
  }

  fn parse_exp_stmt(mut self) -> ast::Stmt {

  }

  // ---

  fn parse_exp(&mut self) -> ast::Exp {

  }
}
