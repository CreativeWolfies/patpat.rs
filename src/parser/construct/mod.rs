use super::super::{
    ast,
    ast::{ASTKind, ASTNode, AST},
};
use super::{token, Token, TokenTree};
use crate::Location;
use std::rc::Rc;

pub mod block;
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

    expr::construct_expression(tree.clone(), offset)
        .or_else(|| construct_non_expression(tree, offset))
}

pub fn construct_non_expression<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    /*! Same as construct, it is separated to allow `construct_expression` to parse its terms */

    functions::construct_pattern_declaration(tree.clone(), offset)
        .or_else(|| functions::construct_pattern_call(tree.clone(), offset))
        .or_else(|| functions::construct_standalone_function(tree.clone(), offset))
        .or_else(|| functions::construct_standalone_pattern(tree.clone(), offset))
        .or_else(|| r#struct::construct_struct(tree.clone(), offset))
        .or_else(|| variables::construct_variable_definition(tree.clone(), offset))
        .or_else(|| variables::construct_variable(tree.clone(), offset))
        .or_else(|| variables::construct_variable_declaration(tree.clone(), offset))
        .or_else(|| tuple::construct_tuple(tree.clone(), offset))
        .or_else(|| ident::construct_ident(tree.clone(), offset))
        .or_else(|| block::construct_block(tree.clone(), offset))
}
