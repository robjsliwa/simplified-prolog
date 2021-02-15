use crate::enum_to_str;

enum_to_str! {
  #[derive(Clone, PartialEq)]
  pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // Two-character tokens
    COLONMINUS,

    // Literals.
    ATOM,
    VARIABLE,
    NUMBER,

    EOF,
  }
}
