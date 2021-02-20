use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrologError {
  #[error("Error parsing")]
  ParserError(String),

  #[error("Interpreter error")]
  InterpreterError(String),

  #[error("Resolver error")]
  ResolverError(String),

  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

pub fn report(line: usize, message: &str) {
  println!("Line[{}] Error: {}", line, message);
}
