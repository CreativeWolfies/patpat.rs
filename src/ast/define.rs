use super::*;

#[derive(Clone, Debug)]
pub enum DefineMember<'a> {
    Variable(String),
    Number(f64),
    Tuple(Box<ASTNode<'a>>),
}
