#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
  Illegal,
  End,
  EOF,

  Number,
  Identifier,

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
