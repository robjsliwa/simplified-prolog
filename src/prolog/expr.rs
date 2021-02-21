use super::{errors::*, literal::Literal};

// expression     → assignment ;
// assignment     → IDENTIFIER "=" assignment
//                | logic_or ;
// logic_or       → logic_and ( "or" logic_and )* ;
// logic_and      → equality ( "and" equality )* ;
// expression     → equality ;
// equality       → comparison ( ( "l!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary | call ;
// call           → primary ( "(" arguments? ")" )* ;
// arguments      → expression ( "," expression )* ;
// primary        → "true" | "false" | "nil" | "this"
//                | NUMBER | STRING | IDENTIFIER | "(" expression ")"
//                | "super" "." IDENTIFIER ;

// rule → clause . | unit_clause .
// clause → head ":-" tail
// head → literal
// tail → literal ( "," literal )*
// unit_clause → literal
// literal → ATOM | VARIABLE | NUMBER

pub trait Expr {
  fn accept(&self, visitor: Box<dyn Visitor>) -> Result<Literal, PrologError>;
  fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Visitor {
  fn visit_literal_expr(&self, expr: &LiteralObj) -> Result<Literal, PrologError>;
  fn visit_unit_clause_expr(&self, expr: &UnitClause) -> Result<Literal, PrologError>;
  fn visit_rule_expr(&self, expr: &Rule) -> Result<Literal, PrologError>;
}

pub struct LiteralObj {
  pub value: Literal,
}

impl LiteralObj {
  pub fn new(value: Literal) -> LiteralObj {
    LiteralObj { value }
  }
}

impl Expr for LiteralObj {
  fn accept(&self, visitor: Box<dyn Visitor>) -> Result<Literal, PrologError> {
    visitor.visit_literal_expr(self)
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}

pub struct UnitClause {
  pub literal: LiteralObj,
}

impl UnitClause {
  pub fn new(literal: LiteralObj) -> UnitClause {
    UnitClause { literal }
  }
}

impl Expr for UnitClause {
  fn accept(&self, visitor: Box<dyn Visitor>) -> Result<Literal, PrologError> {
    visitor.visit_unit_clause_expr(self)
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}

pub struct Rule {
  pub value: UnitClause,
}

impl Rule {
  pub fn new(value: UnitClause) -> Rule {
    Rule { value }
  }
}

impl Expr for Rule {
  fn accept(&self, visitor: Box<dyn Visitor>) -> Result<Literal, PrologError> {
    visitor.visit_rule_expr(self)
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
}

////////////////////////////////////////////////////////////////////////////////

// pub trait Expr<T> {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error>;
// }

// pub trait Visitor<T> {
//   fn visit_binary_expr(&self, expr: &Binary<T>) -> Result<T, Error>;
//   fn visit_grouping_expr(&self, expr: &Grouping<T>) -> Result<T, Error>;
//   fn visit_literal_expr(&self, expr: &LiteralObj) -> Result<T, Error>;
//   fn visit_unary_expr(&self, expr: &Unary<T>) -> Result<T, Error>;
// }

// pub struct Binary<T> {
//   pub left: Rc<RefCell<dyn Expr<T>>>,
//   pub operator: Token,
//   pub right: Rc<RefCell<dyn Expr<T>>>,
// }

// impl<T> Binary<T> {
//   pub fn new(
//     left: Rc<RefCell<dyn Expr<T>>>,
//     operator: Token,
//     right: Rc<RefCell<dyn Expr<T>>>,
//   ) -> Binary<T> {
//     Binary {
//       left,
//       operator,
//       right,
//     }
//   }
// }

// impl<T> Expr<T> for Binary<T> {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
//     visitor.borrow().visit_binary_expr(self)
//   }
// }

// pub struct Grouping<T> {
//   pub expression: Rc<RefCell<dyn Expr<T>>>,
// }

// impl<T> Grouping<T> {
//   pub fn new(expression: Rc<RefCell<dyn Expr<T>>>) -> Grouping<T> {
//     Grouping { expression }
//   }
// }

// impl<T> Expr<T> for Grouping<T> {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
//     visitor.borrow().visit_grouping_expr(self)
//   }
// }

// pub struct LiteralObj {
//   pub value: Option<Literal>,
// }

// impl LiteralObj {
//   pub fn new(value: Option<Literal>) -> LiteralObj {
//     LiteralObj { value }
//   }
// }

// impl<T> Expr<T> for LiteralObj {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
//     visitor.borrow().visit_literal_expr(self)
//   }
// }

// pub struct Unary<T> {
//   pub operator: Token,
//   pub right: Rc<RefCell<dyn Expr<T>>>,
// }

// impl<T> Unary<T> {
//   pub fn new(operator: Token, right: Rc<RefCell<dyn Expr<T>>>) -> Unary<T> {
//     Unary { operator, right }
//   }
// }

// impl<T> Expr<T> for Unary<T> {
//   fn accept(&self, visitor: Rc<RefCell<dyn Visitor<T>>>) -> Result<T, Error> {
//     visitor.borrow().visit_unary_expr(self)
//   }
// }
