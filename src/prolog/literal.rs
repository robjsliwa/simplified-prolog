#[derive(Clone, Debug)]
pub enum Literal {
  Variable(String),
  Atom(String),
  Number(f64),
}
