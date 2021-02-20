use super::{errors::report, literal::Literal, token::Token, token_types::TokenType};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Scanner {
  source: Vec<char>,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  pub fn new(source: Vec<char>) -> Scanner {
    Scanner {
      source,
      tokens: Vec::<Token>::new(),
      start: 0,
      current: 0,
      line: 1,
    }
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.source[self.current - 1]
  }

  fn add_token(&mut self, token_type: TokenType) {
    self.add_token_with_literal(token_type, None);
  }

  fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
    let v = &self.source[self.start..self.current];
    let text: String = v.into_iter().collect();
    self
      .tokens
      .push(Token::new(token_type, &text, literal, self.line));
  }

  fn is_next_match(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      return false;
    }

    if self.source[self.current] != expected {
      return false;
    }

    self.current += 1;
    true
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      return '\0';
    }
    self.source[self.current]
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() {
      return '\0';
    }
    return self.source[self.current + 1];
  }

  fn is_special_char(&self, letter: &char) -> bool {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"[^0-9a-zA-Z()._]+").unwrap();
    }

    RE.is_match(&letter.to_string())
  }

  fn is_alphanumeric(&self, c: char) -> bool {
    match c {
      '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => true,
      _ => false,
    }
  }

  fn is_digit(&self, c: char) -> bool {
    match c {
      '0'..='9' => true,
      _ => false,
    }
  }

  fn process_string_atom(&mut self) {
    while self.peek() != '\'' && !self.is_at_end() {
      if self.peek() == '\n' {
        self.line += 1;
      }
      self.advance();
    }

    // unterminated string
    if self.is_at_end() {
      report(self.line, "Unterminated string.");
      return;
    }

    // the closing "
    self.advance();

    // trim the surrounding quotes
    let l = &self.source[self.start + 1..self.current - 1];
    let literal_string: Literal = Literal::Atom(l.into_iter().collect());
    self.add_token_with_literal(TokenType::ATOM, Some(literal_string));
  }

  fn process_atom(&mut self) {
    while self.is_alphanumeric(self.peek()) {
      self.advance();
    }

    let text_slice = &self.source[self.start..self.current];
    let text: String = text_slice.into_iter().collect();
    self.add_token_with_literal(TokenType::ATOM, Some(Literal::Atom(text)));
  }

  fn process_special_atom(&mut self) {
    while self.is_special_char(&self.peek()) {
      self.advance();
    }

    let text_slice = &self.source[self.start..self.current];
    let text: String = text_slice.into_iter().collect();
    self.add_token_with_literal(TokenType::ATOM, Some(Literal::Atom(text)));
  }

  fn process_variable(&mut self) {
    while self.is_alphanumeric(self.peek()) {
      self.advance();
    }

    let text_slice = &self.source[self.start..self.current];
    let text: String = text_slice.into_iter().collect();
    self.add_token_with_literal(TokenType::ATOM, Some(Literal::Variable(text)));
  }

  fn process_number(&mut self) {
    while self.is_digit(self.peek()) {
      self.advance();
    }

    if self.peek() == '.' && self.is_digit(self.peek_next()) {
      // consume the "."
      self.advance();

      while self.is_digit(self.peek()) {
        self.advance();
      }
    }
  }

  fn scan_token(&mut self) {
    let c = self.advance();
    match c {
      ' ' | '\r' | '\t' => {
        // ignore whitespace
      }
      '\n' => self.line += 1,
      '.' => self.add_token(TokenType::DOT),
      ',' => self.add_token(TokenType::COMMA),
      '\'' => self.process_string_atom(),
      t if self.is_special_char(&c) => {
        if t == ':' && self.peek() == '-' {
          self.advance();
          self.add_token(TokenType::COLONMINUS);
        } else {
          self.process_special_atom();
        }
      }
      'a'..='z' => self.process_atom(),
      'A'..='Z' | '_' => self.process_variable(),
      '0'..='9' => self.process_number(),
      _ => report(self.line, "Unexpected character."),
    }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }

    self
      .tokens
      .push(Token::new(TokenType::EOF, "", None, self.line));

    self.tokens.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn scan_sample_tokens() {
    let text = String::from("a :- b, c, d.\nb :-f. \ne.");
    let source = text.chars().collect();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let assert_tokens = vec![
      TokenType::ATOM,
      TokenType::COLONMINUS,
      TokenType::ATOM,
      TokenType::COMMA,
      TokenType::ATOM,
      TokenType::COMMA,
      TokenType::ATOM,
      TokenType::DOT,
      TokenType::ATOM,
      TokenType::COLONMINUS,
      TokenType::ATOM,
      TokenType::DOT,
      TokenType::ATOM,
      TokenType::DOT,
      TokenType::EOF,
    ];

    let assert_lexeme = vec![
      "a", ":-", "b", ",", "c", ",", "d", ".", "b", ":-", "f", ".", "e", ".", "",
    ];

    for (i, t) in tokens.iter().enumerate() {
      assert_eq!(assert_tokens[i].name(), t.token_type.name());
      assert_eq!(assert_lexeme[i], t.lexeme);
    }
  }
}
