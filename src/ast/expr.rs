use super::*;
use token::Operator;

#[derive(Debug)]
#[derive(Clone)]
pub struct Expression<'a> {
  pub terms: Vec<ExprTerm<'a>>,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum ExprTerm<'a> {
  Push(ASTNode<'a>, Location<'a>),
  Op(Operator),
}
