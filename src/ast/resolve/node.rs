use super::*;
use std::fmt;

#[derive(Clone)]
pub enum RASTNode<'a> {
    // resolved AST node
    PatternCall(RPatRef<'a>, RASTRef<'a>),
    MethodCall(String, RASTRef<'a>),
    Member(String),
    VariableDef(RSymRef, Box<RASTNode<'a>>),
    ComplexDef(RExpression<'a>, DefineMember<'a>, Box<RASTNode<'a>>),
    Function(RFunRef<'a>),
    Pattern(RPatRef<'a>),
    Variable(RSymRef),
    Expression(RExpression<'a>),
    Block(RASTRef<'a>),
    Tuple(Vec<(RASTNode<'a>, Location<'a>)>),
    Boolean(bool),
    Number(f64),
    String(String),
    TypeName(RStructRef<'a>),
    Nil,
}

impl<'a> fmt::Debug for RASTNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RASTNode::PatternCall(pat, args) => f
                .debug_tuple("PatternCall")
                .field(&pat.name)
                .field(&args.borrow())
                .finish(),
            RASTNode::MethodCall(name, args) => f
                .debug_tuple("MethodCall")
                .field(&name)
                .field(&args.borrow())
                .finish(),
            RASTNode::VariableDef(var, value) => f
                .debug_tuple("VariableDef")
                .field(&var.name)
                .field(&value)
                .finish(),
            RASTNode::ComplexDef(expr, member, val) => f
                .debug_tuple("ComplexDef")
                .field(&expr)
                .field(&member)
                .field(val)
                .finish(),
            RASTNode::Function(rfn) => f.debug_tuple("Function").field(&rfn.borrow()).finish(),
            RASTNode::Pattern(rfn) => f.debug_tuple("Pattern").field(&rfn.name).finish(),
            RASTNode::Variable(var) => f.debug_tuple("Variable").field(&var.name).finish(),
            RASTNode::Member(name) => f.debug_tuple("Member").field(&name).finish(),
            RASTNode::Expression(expr) => f.debug_tuple("Expression").field(&expr).finish(),
            RASTNode::Block(rast) => f.debug_tuple("Block").field(&rast).finish(),
            RASTNode::Tuple(vec) => f.debug_tuple("Tuple").field(&vec).finish(),
            RASTNode::TypeName(rstruct) => write!(f, "{:?}", rstruct.borrow().name),
            RASTNode::Boolean(b) => write!(f, "Boolean({})", b),
            RASTNode::Number(x) => write!(f, "Number({})", x),
            RASTNode::String(string) => write!(f, "String({})", string),
            RASTNode::Nil => write!(f, "Nil"),
        }
    }
}
