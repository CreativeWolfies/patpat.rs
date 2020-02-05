use super::{TokenTree, Token};
use super::super::{ast, ast::{ASTNode}};

pub mod functions;

pub fn construct(tree: &mut TokenTree) -> Option<ASTNode> {
  if let Some(x) = functions::construct_pattern_declaration(tree) {Some(x)}
  else {None}
}
