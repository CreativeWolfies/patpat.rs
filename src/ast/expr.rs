use super::*;
use token::Operator;

#[derive(Debug)]
#[derive(Clone)]
pub struct Expression<'a> {
  pub terms: Vec<(ASTNode<'a>, Vec<Operator>, Location<'a>)>, // term, unary ops, location
  pub ops: Vec<Operator>
}
