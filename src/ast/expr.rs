use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct Expression<'a> {
  pub terms: Vec<(ASTNode<'a>, Location<'a>)>,
  pub ops: Vec<token::Operator>
}
