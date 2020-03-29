use super::{ASTNode, AST, ASTKind, TokenTree, Token};
use crate::Location;
use std::rc::Rc;
use crate::error::*;

// TODO: make it TypeName(String)

pub fn construct_struct<'a>(
  tree: Rc<TokenTree<'a>>,
  offset: &mut usize
) -> Option<(ASTNode<'a>, Location<'a>)> {
  /*! Constructs Struct definitions
  */
  if tree.tokens.len() > *offset + 1 {
    if let (Token::TypeName(tn), tn_loc) = &tree.tokens[*offset] {
      if let (Token::Define, define_loc) = &tree.tokens[*offset + 1] {
        if tree.tokens.len() <= *offset + 3 {
          CompError::new(
            20,
            String::from("Incomplete struct definition: expected struct and block"),
            CompLocation::from(define_loc)
          ).print_and_exit();
        }
        if let (Token::Struct, _) = &tree.tokens[*offset + 2] {
          if let (Token::Block(tree), _) = &tree.tokens[*offset + 3] {
            *offset += 4;
            return Some((
              ASTNode::Struct(tn.clone(), AST::parse(tree.clone(), ASTKind::Struct)),
              tn_loc.clone()
            ))
          } else {
            CompError::new(
              20,
              String::from("Unexpected token in struct definition: expected struct block"),
              CompLocation::from(&tree.tokens[*offset + 2].1)
            ).print_and_exit();
          }
        } else {
          CompError::new(
            20,
            String::from("Unexpected token in struct definition: expected `struct`"),
            CompLocation::from(&tree.tokens[*offset + 2].1)
          ).print_and_exit();
        }
      }
    }
  }
  None
}
