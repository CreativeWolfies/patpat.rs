use super::*;
use std::fmt;

#[derive(Clone)]
pub enum RASTNode<'a> {
    // resolved AST node
    PatternCall(RPatRef<'a>, RASTRef<'a>),
    VariableDef(RSymRef, Box<RASTNode<'a>>),
    Function(RFunRef<'a>),
    Pattern(RPatRef<'a>),
    Variable(RSymRef),
    Expression(RExpression<'a>),
    Block(RASTRef<'a>),
    Tuple(Vec<(RASTNode<'a>, Location<'a>)>),
    Boolean(bool),
    Number(f64),
    Nil,
}

impl<'a> fmt::Debug for RASTNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RASTNode::PatternCall(pat, args) => f
                .debug_tuple("PatternCall")
                .field(&pat.borrow().name)
                .field(&args.borrow())
                .finish(),
            RASTNode::VariableDef(var, value) => f
                .debug_tuple("VariableDef")
                .field(&var.name)
                .field(&value)
                .finish(),
            RASTNode::Function(rfn) => f.debug_tuple("Function").field(&rfn.borrow()).finish(),
            RASTNode::Pattern(rfn) => f.debug_tuple("Pattern").field(&rfn.borrow().name).finish(),
            RASTNode::Variable(var) => f.debug_tuple("Variable").field(&var.name).finish(),
            RASTNode::Expression(expr) => f.debug_tuple("Expression").field(&expr).finish(),
            RASTNode::Block(rast) => f.debug_tuple("Block").field(&rast).finish(),
            RASTNode::Tuple(vec) => f.debug_tuple("Tuple").field(&vec).finish(),
            RASTNode::Boolean(b) => write!(f, "Boolean({})", b),
            RASTNode::Number(x) => write!(f, "Number({})", x),
            RASTNode::Nil => write!(f, "Nil"),
        }
    }
}
