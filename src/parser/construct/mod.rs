use super::{TokenTree, Token};
use super::super::{ast, ast::{ASTNode, ASTKind, AST}};

pub mod functions;

pub fn construct(tree: &TokenTree, offset: &mut usize) -> Option<ASTNode> {
  /*!
  * Constructs an ASTNode from the TokenTree. It does this by trying every method in order.
  * No AST building magic library is used, as to provide better granularity.
  */
  if let Some(x) = functions::construct_pattern_declaration(tree, offset) {Some(x)}
  else if let Some(x) = functions::construct_pattern_call(tree, offset) {Some(x)}
  else {None}
}
