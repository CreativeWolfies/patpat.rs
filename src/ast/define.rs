use super::*;

#[derive(Clone)]
#[derive(Debug)]
pub enum DefineMember<'a> {
  Variable(String),
  Number(f64),
  Tuple(Box<ASTNode<'a>>)
}
