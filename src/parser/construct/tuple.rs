use super::{ASTKind, ASTNode, Token, TokenTree, AST};
use crate::Location;
use std::rc::Rc;

/** Assembles tuples together
    @param tree - The TokenTree containing the Tuple node
    @param offset - The offset to the Tuple node; will be incremented on success
    @returns The node and its location on success, None otherwise
**/
pub fn construct_tuple<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    if let (Token::Tuple(tree), loc) = &tree.tokens[*offset] {
        let ast = AST::parse(tree.clone(), ASTKind::Tuple);

        *offset += 1;
        if ast.instructions.len() == 0 {
            Some((ASTNode::Nil, loc.clone()))
        } else {
            Some((ASTNode::Tuple(ast, false), loc.clone()))
        }
    } else {
        None
    }
}
