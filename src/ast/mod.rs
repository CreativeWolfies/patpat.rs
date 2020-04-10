pub mod define;
pub mod expr;
pub mod function;
pub mod internal;
pub mod node;
pub mod pattern;
pub mod resolve;

pub use super::error::*;
pub use super::parser::{
    construct, token,
    token::{Token, TokenTree, Type, TypeName},
};
pub use crate::Location;
pub use define::*;
pub use expr::*;
pub use function::*;
pub use node::ASTNode;
pub use pattern::*;
pub use resolve::*;
pub use std::rc::Rc;

/** Asyntactical tree: a more tree-like representation of instructions and expressions
* Contains a set of ASTNodes, which may contain nested ASTs
*/
#[derive(Debug, Clone)]
pub struct AST<'a> {
    pub instructions: Vec<(ASTNode<'a>, Location<'a>)>,
    pub kind: ASTKind,
}

impl<'a> AST<'a> {
    pub fn new(kind: ASTKind) -> AST<'a> {
        //! Outputs a blank AST
        AST {
            instructions: Vec::new(),
            kind,
        }
    }

    pub fn parse(raw: TokenTree<'a>, kind: ASTKind) -> AST<'a> {
        //! Parses a TokenTree (node) down into an AST
        let len = raw.tokens.len();
        let raw = Rc::new(raw);
        // let raw_c = raw.clone();
        let mut instructions = Vec::<(ASTNode<'a>, Location<'a>)>::new();
        let mut offset = 0usize;
        while offset < len {
            match construct::construct(raw.clone(), &mut offset) {
                Some(node) => {
                    kind.verify_term(&node);
                    instructions.push(node);
                    expect_next_instruction(raw.clone(), &mut offset);
                }
                None => {
                    // ERROR: invalid token
                    panic!("Unimplemented")
                }
            }
        }
        AST { instructions, kind }
    }
}

/// Denotes what kind of AST an AST represents, used to determine wether an input expression is valid or not
#[derive(Debug, Clone)]
pub enum ASTKind {
    Tuple,
    ArgTuple,
    Block,
    File,
    Struct,
}

impl ASTKind {
    pub fn verify_term<'a>(&self, node: &'a (ASTNode<'a>, Location<'a>)) {
        match self {
            ASTKind::Tuple => {
                if !node.0.is_valid_tuple_term() {
                    CompError::new(
                        11,
                        String::from("Invalid tuple term"),
                        CompLocation::from(node.1.clone()),
                    )
                    .print_and_exit();
                }
            }
            ASTKind::ArgTuple => {
                if !node.0.is_valid_argtuple_term() {
                    CompError::new(
                        12,
                        String::from("Invalid argument term"),
                        CompLocation::from(node.1.clone()),
                    )
                    .print_and_exit();
                }
            }
            ASTKind::Block => {
                if !node.0.is_valid_block_term() {
                    CompError::new(
                        13,
                        String::from("Invalid block instruction"),
                        CompLocation::from(node.1.clone()),
                    )
                    .print_and_exit();
                }
            }
            ASTKind::File => {
                if !node.0.is_valid_file_term() {
                    CompError::new(
                        14,
                        String::from("Invalid source file instruction"),
                        CompLocation::from(node.1.clone()),
                    )
                    .print_and_exit();
                }
            }
            ASTKind::Struct => {
                if !node.0.is_valid_struct_term() {
                    CompError::new(
                        14,
                        String::from("Invalid struct instruction"),
                        CompLocation::from(node.1.clone()),
                    )
                    .print_and_exit();
                }
            }
        }
    }
}

fn expect_next_instruction<'a>(tree: Rc<TokenTree<'a>>, offset: &mut usize) {
    if *offset == 0 || tree.tokens.len() == *offset {
        return;
    }
    let old_loc = &tree.tokens[*offset - 1].1;
    let new_loc = &tree.tokens[*offset].1;

    if old_loc.line < new_loc.line {
        return;
    }

    if let (Token::Separator, _) = tree.tokens[*offset] {
        *offset += 1;
        return;
    }

    CompError::new(
        15,
        String::from("Missing separator or newline"),
        CompLocation::from(new_loc),
    )
    .append(
        String::from("Consider adding a separator (',') after this term"),
        CompLocation::from(old_loc),
    )
    .print_and_exit();
}
