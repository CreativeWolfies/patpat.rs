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
