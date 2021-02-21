use super::{errors::PrologError, expr::*, literal::*, token::*, token_types::*};
use std::cell::Cell;

type ParserExpr = Box<dyn Expr>;

pub struct Parser {
  tokens: Vec<Token>,
  current: Cell<usize>,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser {
      tokens,
      current: Cell::new(0),
    }
  }

  fn token_match(&self, token_types: Vec<TokenType>) -> bool {
    for token_type in token_types {
      if self.check(token_type) {
        self.advance();
        return true;
      }
    }

    false
  }

  fn check(&self, token_type: TokenType) -> bool {
    if self.is_at_end() {
      return false;
    }
    self.peek().token_type == token_type
  }

  fn advance(&self) -> Token {
    if !self.is_at_end() {
      self.current.set(self.current.get() + 1);
    }
    self.previous()
  }

  fn is_at_end(&self) -> bool {
    self.peek().token_type == TokenType::EOF
  }

  fn peek(&self) -> Token {
    self.tokens[self.current.get()].clone()
  }

  fn previous(&self) -> Token {
    self.tokens[self.current.get() - 1].clone()
  }

  fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, PrologError> {
    if self.check(token_type) {
      return Ok(self.advance());
    }

    Err(PrologError::ParserError(
      self.tokens_line_number(),
      message.to_string(),
    ))
  }

  fn tokens_line_number(&self) -> usize {
    let token = self.peek();
    token.line
  }

  pub fn parse(&self) -> Result<Vec<Rule>, PrologError> {
    let mut rules = Vec::new();
    while !self.is_at_end() {
      rules.push(self.rule()?);
    }

    Ok(rules)
  }

  fn rule(&self) -> Result<Rule, PrologError> {
    if self.token_match(vec![TokenType::ATOM]) {
      if self.peek().token_type == TokenType::DOT {
        let unit_clause = self.unit_clause()?;
        return Ok(Rule::new(unit_clause));
      }

      // return self.clause();
    }

    Err(PrologError::ParserError(
      self.tokens_line_number(),
      "Expected clause.".to_string(),
    ))
  }

  fn unit_clause(&self) -> Result<UnitClause, PrologError> {
    let atom = self.atom()?;
    self.advance();

    Ok(UnitClause::new(atom))
  }

  fn atom(&self) -> Result<LiteralObj, PrologError> {
    let atom = self.previous();
    let literal = match atom.literal {
      Some(l) => l,
      None => {
        return Err(PrologError::ParserError(
          atom.line,
          "Invalid atom.".to_string(),
        ))
      }
    };
    match literal {
      Literal::Atom(a) => Ok(LiteralObj::new(Literal::Atom(a))),
      _ => Err(PrologError::ParserError(
        self.tokens_line_number(),
        "Expected atom.".to_string(),
      )),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prolog::scanner::Scanner;

  #[test]
  fn test_unit_clauses() -> Result<(), PrologError> {
    let text = String::from("d.\nf. \ne.");
    let source = text.chars().collect();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    let parser = Parser::new(tokens);
    let rules = parser.parse()?;

    let atom_lexemes = vec!["d", "f", "e"];

    for (i, rule) in rules.iter().enumerate() {
      match &rule.value.literal.value {
        Literal::Atom(a) => assert_eq!(a, atom_lexemes[i]),
        _ => {
          return Err(PrologError::ParserError(
            0,
            "Invalid atom type.".to_string(),
          ));
        }
      }
    }

    Ok(())
  }
}
