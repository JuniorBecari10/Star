use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
  Illegal,
  End,
  EOF,

  Number,
  Identifier,

  FnKw,
  VarKw,
  ConstKw,

  Plus,
  Minus,
  Star,
  Slash,

  Assign,

  LParen,
  RParen,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
  pub content: Vec<char>,
  pub kind: TokenKind,
  pub pos: usize
}

pub static keywords: HashMap<String, TokenKind> = HashMap::from([
  ("fn".to_string(), TokenKind::FnKw),
  ("var".to_string(), TokenKind::VarKw),
  ("const".to_string(), TokenKind::ConstKw),
]);
