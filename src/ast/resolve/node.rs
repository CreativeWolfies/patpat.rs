use super::*;
use std::fmt;

#[derive(Clone)]
pub enum RASTNode<'a> { // resolved AST node
  PatternCall(Rc<RefCell<RPattern<'a>>>, Rc<RefCell<RAST<'a>>>),
  VariableDef(Rc<RefCell<RSymbol<'a>>>, Rc<RefCell<RAST<'a>>>),
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
          .field(&value.borrow())
          .finish()
      }
    }
  }
}
