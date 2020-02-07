use super::parser::{token::{Type, Token, TokenTree, TokenLocation}, construct};
use super::error::*;

/** Asyntactical tree: a more tree-like representation of instructions and expressions
* Contains a set of ASTNodes, which may contain nested ASTs
*/
#[derive(Debug)]
#[derive(Clone)]
pub struct AST {
  pub instructions: Vec<ASTNode>,
  pub kind: ASTKind,
}

/// A node in an AST
#[derive(Debug)]
#[derive(Clone)]
pub enum ASTNode {
  Function(Function),
  PatternDecl(Pattern),
  PatternCall(String, AST), // name, tuple
  Variable(String),
  TypedVariable(String, Type),
}

impl AST {
  pub fn new(kind: ASTKind) -> AST {
    //! Outputs a blank AST
    AST {
      instructions: Vec::new(),
      kind
    }
  }

  pub fn parse(raw: TokenTree, kind: ASTKind) -> AST {
    //! Parses a TokenTree (node) down into an AST
    let mut raw = raw.clone();
    let mut instructions = Vec::<ASTNode>::new();
    let mut offset = 0usize;
    while offset < raw.tokens.len() {
      match construct::construct(&mut raw, &mut offset) {
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

/// The Function type, corresponds to `TUPLE ARROW BLOCK`
#[derive(Debug)]
#[derive(Clone)]
pub struct Function {
  pub args: Vec<FunctionArg>,
  pub body: AST,
  pub has_self: bool,
  pub has_lhs: bool,
}

impl Function {
  pub fn parse<'a>(
    one: (Token<'a>, TokenLocation<'a>),
    two: (Token<'a>, TokenLocation<'a>),
    three: (Token<'a>, TokenLocation<'a>)
  ) -> Option<Function> {
    /*! Takes as input three tokens and tries to parse them into a function
    * If these three tokens happen to be a Tuple, an Arrow and a Block, then this function yields a Function.
    * Otherwise it will return None
    */
    match (one.0, two.0, three.0) {
      (Token::Tuple(raw_tuple), Token::Arrow, Token::Block(raw_body)) => {
        let tuple = AST::parse(raw_tuple, ASTKind::ArgTuple);
        let body = AST::parse(raw_body, ASTKind::Block);
        let mut has_self = false;
        let mut has_lhs = false;
        let mut args = Vec::<FunctionArg>::new();
        for raw_arg in tuple.instructions {
          match raw_arg {
            ASTNode::Variable(name) => args.push(FunctionArg {
              argtype: None,
              name: name.to_string()
            }),
            ASTNode::TypedVariable(name, argtype) => args.push(FunctionArg {
              argtype: Some(argtype),
              name: name.to_string()
            }),
            ASTNode::PatternCall(name, _args) => { // TODO: handle _args
              if name == "#self" {
                if has_self {} // ERROR: duplicate #self()
                else {
                  has_self = true;
                }
              } else if name == "#lhs" {
                if has_lhs {} // ERROR: duplicate #lhs()
                else {
                  has_lhs = true;
                }
              } // else ERROR
            },
            _ => {
              // ERROR: invalid element in array tuple (should be handled by AST::parse)
              return None
            }
          }
        }
        Some(Function {
          args,
          body,
          has_self,
          has_lhs,
        })
      },
      _ => None,
    }
  }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct FunctionArg {
  pub argtype: Option<Type>,
  pub name: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Pattern {
  pub function: Function,
  pub name: String,
}
