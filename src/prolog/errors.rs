use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrologError {
  #[error("Error parsing")]
  ParserError(usize, String),

  #[error("Interpreter error")]
  InterpreterError(usize, String),

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

pub fn report(line: usize, message: &str) {
  println!("Line[{}] Error: {}", line, message);
}
