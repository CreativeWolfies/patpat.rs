use super::{TokenTree, Token, token};
use super::super::{ast, ast::{ASTNode, ASTKind, AST}};
use crate::{Location};
use std::rc::Rc;

pub mod functions;
pub mod ident;
pub mod variables;
pub mod expr;

pub fn construct<'a>(tree: Rc<TokenTree<'a>>, offset: &mut usize) -> Option<(ASTNode<'a>, Location<'a>)> {
  /*!
  * Constructs an ASTNode from the TokenTree. It does this by trying every method in order.
  * No AST building magic library is used, as to provide better granularity and more headache.
  */
  if let Some(x) = expr::construct_expression(tree.clone(), offset) {
    Some(x)
  } else {
    construct_non_expression(tree, offset)
  }
}

pub fn construct_non_expression<'a>(tree: Rc<TokenTree<'a>>, offset: &mut usize) -> Option<(ASTNode<'a>, Location<'a>)> {
  if let Some(x) = functions::construct_pattern_declaration(tree.clone(), offset) {Some(x)}
  else if let Some(x) = functions::construct_pattern_call(tree.clone(), offset) {Some(x)}
  else if let Some(x) = functions::construct_standalone_function(tree.clone(), offset) {Some(x)}
  else if let Some(x) = ident::construct_ident(tree.clone(), offset) {Some(x)}
  else if let Some(x) = variables::construct_variable(tree.clone(), offset) {Some(x)}
  else {None}
}
