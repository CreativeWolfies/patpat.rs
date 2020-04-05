use super::{ASTKind, ASTNode, Token, TokenTree, AST};
use crate::Location;
use std::rc::Rc;

pub fn construct_block<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    /*! Constructs blocks (`{...}`)

        **Example:**

        ```patpat
        {
            x: x * 2
            x - 1
        }
        ```

        Becomes (simplified):

        ```rust
        ASTNode::Block(
            AST {
                instructions: [
                    "x: x * 2",
                    "x - 1"
                ]
            }
        )
        ```
    */
    if let (Token::Block(tree), loc) = &tree.tokens[*offset] {
        let ast = AST::parse(tree.clone(), ASTKind::Block);
        *offset += 1;
        Some((ASTNode::Block(ast), loc.clone()))
    } else {
        None
    }
}
