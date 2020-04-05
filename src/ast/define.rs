use super::*;

#[derive(Clone, Debug)]
pub enum DefineMember<'a> {
    Member(String),
    Number(f64),
    Tuple(Box<ASTNode<'a>>),
}
