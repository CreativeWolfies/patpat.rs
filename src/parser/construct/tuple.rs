use super::{ASTNode, AST, ASTKind, TokenTree, Token};
use crate::Location;
use std::rc::Rc;

pub fn construct_tuple<'a>(
  tree: Rc<TokenTree<'a>>,
  offset: &mut usize
) -> Option<(ASTNode<'a>, Location<'a>)> {
  if let (Token::Tuple(tree), loc) = &tree.tokens[*offset] {
    let mut ast = AST::parse(tree.clone(), ASTKind::Tuple);

    *offset += 1;
    if ast.instructions.len() == 0 {
      Some((ASTNode::Nil, loc.clone()))
    } else if ast.instructions.len() == 1 {
      Some(ast.instructions.pop()?)
    } else {
      Some((ASTNode::Tuple(ast), loc.clone()))
    }
  } else {
    None
  }
}
