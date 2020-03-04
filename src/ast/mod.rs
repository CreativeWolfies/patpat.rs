pub mod node;
pub mod function;
pub mod pattern;
pub mod expr;

pub use super::parser::{token::{Type, Token, TokenTree}, construct, token};
pub use std::rc::Rc;
pub use super::error::*;
pub use crate::Location;
pub use node::ASTNode;
pub use function::*;
pub use pattern::*;
pub use expr::*;

/** Asyntactical tree: a more tree-like representation of instructions and expressions
* Contains a set of ASTNodes, which may contain nested ASTs
*/
#[derive(Debug)]
#[derive(Clone)]
pub struct AST<'a> {
  pub instructions: Vec<(ASTNode<'a>, Location<'a>)>,
  pub kind: ASTKind,
}

impl<'a> AST<'a> {
  pub fn new(kind: ASTKind) -> AST<'a> {
    //! Outputs a blank AST
    AST {
      instructions: Vec::new(),
      kind
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
          instructions.push(node);
        },
        None => {
          // ERROR: invalid token
          panic!("Unimplemented")
        }
      }
    }
    AST {
      instructions,
      kind
    }
  }
}

/// Denotes what kind of AST an AST represents, used to determine wether an input expression is valid or not
#[derive(Debug)]
#[derive(Clone)]
pub enum ASTKind {
  Tuple,
  ArgTuple,
  Block,
  Expression,
  File,
}
