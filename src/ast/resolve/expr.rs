use super::*;
use token::Operator;

#[derive(Clone, Debug)]
pub struct RExpression<'a> {
    pub terms: Vec<RExprTerm<'a>>,
    pub max_depth: usize,
}

#[derive(Clone, Debug)]
pub enum RExprTerm<'a> {
    Push(RASTNode<'a>),
    Op(Operator),
}
