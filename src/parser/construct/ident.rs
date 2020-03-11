use super::{ASTNode, TokenTree, Token};
use crate::Location;
use std::rc::Rc;

// Identity (e.g. number) constructs

pub fn construct_ident<'a>(tree: Rc<TokenTree<'a>>, offset: &mut usize) -> Option<(ASTNode<'a>, Location<'a>)> {
  /*!
  Tries to construct simple terms:
  - numbers
  - strings
  - booleans
  */
  let res = match &tree.tokens[*offset] {
    (Token::Boolean(b), loc) => Some((ASTNode::Boolean(*b), loc.clone())),
    (Token::Number(n), loc) => Some((ASTNode::Number(*n), loc.clone())),
    (Token::String(s), loc) => Some((ASTNode::String(s.clone()), loc.clone())),
    (Token::TypeName(n), loc) => Some((ASTNode::TypeName(n.clone()), loc.clone())),
    _ => None
  };
  if let None = res {}
  else {
    *offset += 1;
  }
  res
}
