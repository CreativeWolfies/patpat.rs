use super::{TokenTree, Token};
use super::super::{ast, ast::{ASTNode}};

pub mod functions;

pub fn construct(tree: &mut TokenTree, offset: &mut usize) -> Option<ASTNode> {
  if let Some(x) = functions::construct_pattern_declaration(tree, offset) {Some(x)}
  else {None}
}
