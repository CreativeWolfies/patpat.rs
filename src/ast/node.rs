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
  Expression(Expression<'a>)
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
      _ => false,
    }
  }
}
