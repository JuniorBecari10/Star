#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_lexer() {
    let input = "123 hello +-*/()=".into();
    let tokens = lex(input);

    let expect: Vec<token::Token> = vec![
      token::Token {
        content: "123".chars().collect(),
        kind: token::TokenKind::Number,
        pos: 0
      },
      token::Token {
        content: "hello".chars().collect(),
        kind: token::TokenKind::Identifier,
        pos: 4
      },
      token::Token {
        content: "+".chars().collect(),
        kind: token::TokenKind::Plus,
        pos: 10
      },
      token::Token {
        content: "-".chars().collect(),
        kind: token::TokenKind::Minus,
        pos: 11
      },
      token::Token {
        content: "*".chars().collect(),
        kind: token::TokenKind::Star,
        pos: 12
      },
      token::Token {
        content: "/".chars().collect(),
        kind: token::TokenKind::Slash,
        pos: 13
      },
      token::Token {
        content: "(".chars().collect(),
        kind: token::TokenKind::LParen,
        pos: 14
      },
      token::Token {
        content: ")".chars().collect(),
        kind: token::TokenKind::RParen,
        pos: 15
      },
      token::Token {
        content: "=".chars().collect(),
        kind: token::TokenKind::Assign,
        pos: 16
      },
    ];

    assert_eq!(tokens, expect);
  }
}

pub mod token;

struct Lexer {
  input: Vec<char>,
  cursor: usize,
}

impl Lexer {
  fn new(input: Vec<char>) -> Self {
    Self { input: input, cursor: 0 }
  }

  fn advance(&mut self) {
    self.cursor += 1;
  }

  fn char(&mut self) -> char {
    if self.cursor >= self.input.len() {
      return '\0';
    }

    self.input[self.cursor]
  }

  fn skip_whitespace(&mut self) {
    while self.char().is_whitespace() {
      self.advance();
    }
  }

  fn next_token(&mut self) -> token::Token {
    self.skip_whitespace();

    let mut buf: Vec<char> = vec![self.char()];
    let pos: usize = self.cursor;

    let is_ident = |ch: char| ch.is_alphabetic() || ch == '_';
    let is_num = |ch: char| ch.is_numeric() || ch == '.';

    let kind = match self.char() {
      '\0' => token::TokenKind::EOF,

      '+' => token::TokenKind::Plus,
      '-' => token::TokenKind::Minus,
      '*' => token::TokenKind::Star,
      '/' => token::TokenKind::Slash,

      '(' => token::TokenKind::LParen,
      ')' => token::TokenKind::RParen,

      '=' => token::TokenKind::Assign,

      _ if is_num(self.char()) => {
        self.advance();

        while is_num(self.char()) {
          buf.push(self.char());
          self.advance();
        }

        let point_count = buf.iter().filter(|&n| *n == '.').count();

        if point_count > 1 {
          // TODO: throw error
        }

        token::TokenKind::Number
      }

      _ if is_ident(self.char()) => {
        self.advance();

        while is_ident(self.char()) {
          buf.push(self.char());
          self.advance();
        }

        token::TokenKind::Identifier
      }

      _ => {
        token::TokenKind::Illegal
      }
    };

    self.advance();
    token::Token {
      content: buf.clone(),
      kind: kind,
      pos: pos
    }
  }
}

fn lex(input: String) -> Vec<token::Token> {
  let mut l: Lexer = Lexer::new(input.chars().collect());
  let mut tokens: Vec<token::Token> = Vec::new();
  let mut token: token::Token = l.next_token();

  while token.kind != token::TokenKind::EOF {
    tokens.push(token);
    token = l.next_token();
  }

  tokens
}
