use super::super::{
    ast,
    ast::{ASTKind, ASTNode, AST},
};
use super::{token, Token, TokenTree};
use crate::Location;
use std::rc::Rc;

pub mod expr;
pub mod functions;
pub mod ident;
pub mod r#struct;
pub mod tuple;
pub mod variables;

pub fn construct<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    /*!
     * Constructs an ASTNode from the TokenTree. It does this by trying every method in order.
     * No AST building magic library is used, as to provide better granularity and more headache.
     *
     * Modifies `offset`
     */
    if let Some(x) = expr::construct_expression(tree.clone(), offset) {
        Some(x)
    } else {
        construct_non_expression(tree, offset)
    }
}

pub fn construct_non_expression<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    /*! Same as construct, it is separated to allow `construct_expression` to parse its terms */
    if let Some(x) = functions::construct_pattern_declaration(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = functions::construct_pattern_call(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = functions::construct_standalone_function(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = functions::construct_standalone_pattern(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = r#struct::construct_struct(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = variables::construct_variable_definition(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = variables::construct_variable(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = variables::construct_variable_declaration(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = tuple::construct_tuple(tree.clone(), offset) {
        Some(x)
    } else if let Some(x) = ident::construct_ident(tree.clone(), offset) {
        Some(x)
    } else {
        None
    }
}
