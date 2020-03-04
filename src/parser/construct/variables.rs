use super::{ASTNode, AST, ASTKind, TokenTree, Token, ast};
use crate::Location;
use std::rc::Rc;

pub fn construct_variable<'a>(tree: Rc<TokenTree<'a>>, offset: &mut usize) -> Option<(ASTNode<'a>, Location<'a>)> {
  /*! Tries to match plain variables; does not run any lookup or simulation */
  if let (Token::Symbol(s), loc) = &tree.tokens[*offset] {
    if tree.tokens.len() > *offset + 1 {
      if let (Token::Type(t), _) = &tree.tokens[*offset + 1] {
        *offset += 2;
        return Some((ASTNode::TypedVariable(s.name.clone(), t.clone()), loc.clone()))
      }
    }
    *offset += 1;
    Some((ASTNode::Variable(s.name.clone()), loc.clone()))
  } else {
    None
  }
}
