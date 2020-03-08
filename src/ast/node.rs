use super::*;

/// A node in an AST
#[derive(Debug)]
#[derive(Clone)]
pub enum ASTNode<'a> {
  Function(Function<'a>),
  PatternDecl(Pattern<'a>),
  PatternCall(String, AST<'a>), // name, tuple
  Variable(String),
  TypedVariable(String, Type),
  Boolean(bool),
  Number(f64),
  String(String),
  Expression(Expression<'a>),
  Tuple(AST<'a>),
  Nil,
}

impl<'a> ASTNode<'a> {
  pub fn is_valid_expr_term(&self) -> bool {
    match self {
      ASTNode::Function(_) => true,
      ASTNode::PatternCall(_, _) => true,
      ASTNode::Variable(_) => true,
      ASTNode::Boolean(_) => true,
      ASTNode::Number(_) => true,
      ASTNode::String(_) => true,
      ASTNode::Tuple(_) => true,
      ASTNode::Nil => true,
      ASTNode::Expression(_) => true,
      _ => false,
    }
  }

  pub fn is_valid_tuple_term(&self) -> bool {
    return self.is_valid_expr_term();
  }

  pub fn is_valid_argtuple_term(&self) -> bool {
    match self {
      ASTNode::Variable(_) => true,
      ASTNode::TypedVariable(_, _) => true,
      ASTNode::PatternCall(_, _) => true,
      _ => false,
    }
  }

  pub fn is_valid_block_term(&self) -> bool {
    if self.is_valid_expr_term() {
      return true;
    }
    match self {
      ASTNode::PatternDecl(_) => true,
      _ => false,
    }
  }

  pub fn is_valid_file_term(&self) -> bool {
    if self.is_valid_block_term() {
      return true;
    }
    return false;
  }
}
