use super::*;
use token::Operator;

#[derive(Clone)]
#[derive(Debug)]
pub struct RExpression<'a> {
  pub terms: Vec<RExprTerm<'a>>,
  pub max_depth: usize,
}

#[derive(Clone)]
#[derive(Debug)]
pub enum RExprTerm<'a> {
  Push(RASTNode<'a>),
  Op(Operator),
}
