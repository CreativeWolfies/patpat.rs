use super::*;
use std::fmt;

#[derive(Clone)]
pub enum RASTNode<'a> { // resolved AST node
  PatternCall(RPatRef<'a>, RASTRef<'a>),
  VariableDef(RSymRef<'a>, Box<RASTNode<'a>>),
  Function(RFunRef<'a>),
  Pattern(RPatRef<'a>),
  Variable(RSymRef<'a>),
  Boolean(bool),
  Number(f64),
  Nil,
}

impl<'a> fmt::Debug for RASTNode<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      RASTNode::PatternCall(pat, args) => {
        f.debug_tuple("PatternCall")
          .field(&pat.borrow().name)
          .field(&args.borrow())
          .finish()
      },
      RASTNode::VariableDef(var, value) => {
        f.debug_tuple("VariableDef")
          .field(&var.borrow().name)
          .field(&value)
          .finish()
      },
      RASTNode::Function(rfn) => {
        f.debug_tuple("Function")
          .field(&rfn.borrow())
          .finish()
      },
      RASTNode::Pattern(rfn) => {
        f.debug_tuple("Pattern")
          .field(&rfn.borrow().name)
          .finish()
      },
      RASTNode::Variable(var) => {
        f.debug_tuple("Variable")
          .field(&var.borrow().name)
          .finish()
      },
      RASTNode::Boolean(b) => {
        write!(f, "Boolean({})", b)
      },
      RASTNode::Number(x) => {
        write!(f, "Number({})", x)
      },
      RASTNode::Nil => write!(f, "Nil"),
    }
  }
}
