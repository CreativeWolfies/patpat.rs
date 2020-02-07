use super::parser::{token::{Type, Token, TokenTree}, construct};
use std::rc::Rc;
use super::error::*;
use crate::Location;

/** Asyntactical tree: a more tree-like representation of instructions and expressions
* Contains a set of ASTNodes, which may contain nested ASTs
*/
#[derive(Debug)]
#[derive(Clone)]
pub struct AST<'a> {
  pub instructions: Vec<(ASTNode<'a>, Location<'a>)>,
  pub kind: ASTKind,
}

/// A node in an AST
#[derive(Debug)]
#[derive(Clone)]
pub enum ASTNode<'a> {
  Function(Function<'a>),
  PatternDecl(Pattern<'a>),
  PatternCall(String, AST<'a>), // name, tuple
  Variable(String),
  TypedVariable(String, Type),
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

/// The Function type, corresponds to `TUPLE ARROW BLOCK`
#[derive(Debug)]
#[derive(Clone)]
pub struct Function<'a> {
  pub args: Vec<FunctionArg>,
  pub body: AST<'a>,
  pub has_self: bool,
  pub has_lhs: bool,
}

impl<'a> Function<'a> {
  pub fn parse(
    one: (Token<'a>, Location<'a>),
    two: (Token<'a>, Location<'a>),
    three: (Token<'a>, Location<'a>),
    is_pattern: bool
  ) -> Option<Function<'a>> {
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
        let mut visited = Vec::<(ASTNode, Location)>::new();
        for (raw_arg, location) in tuple.instructions {
          match &raw_arg {
            ASTNode::Variable(name) => args.push(FunctionArg {
              argtype: None,
              name: name.to_string()
            }),
            ASTNode::TypedVariable(name, argtype) => args.push(FunctionArg {
              argtype: Some(argtype.clone()),
              name: name.to_string()
            }),
            ASTNode::PatternCall(name, _args) => { // TODO: handle _args
              if name == "#self" {
                if has_self {
                  let mut err = CompError::from(104, CompInfo::new(
                    "Duplicate #self() in pattern declaration",
                    CompLocation::from(&location)
                  ));
                  for (r, l) in visited.into_iter() {
                    if let ASTNode::PatternCall(n, _) = r {
                      if n == "#self" {
                        err.add_info(CompInfo::new(
                          "#self() is used here",
                          CompLocation::from(l)
                        ));
                        break;
                      }
                    }
                  }
                  err.print_and_exit();
                  break;
                } else if !is_pattern {
                  CompError::from(105, CompInfo::new(
                    "#self() can only be used as a pattern's argument",
                    CompLocation::from(&location)
                  )).print_and_exit();
                } else {
                  has_self = true;
                }
              } else if name == "#lhs" {
                if has_lhs {
                  let mut err = CompError::from(106, CompInfo::new(
                    "Duplicate #lhs() in function declaration",
                    CompLocation::from(&location)
                  ));
                  for (r, l) in visited.into_iter() {
                    if let ASTNode::PatternCall(n, _) = r {
                      if n == "#lhs" {
                        err.add_info(CompInfo::new(
                          "#lhs() is used here",
                          CompLocation::from(l)
                        ));
                        break;
                      }
                    }
                  }
                  err.print_and_exit();
                  break;
                } else {
                  has_lhs = true;
                }
              } // else ERROR
            },
            _ => {
              // ERROR: invalid element in array tuple (should be handled by AST::parse)
              return None
            }
          }
          visited.push((raw_arg, location));
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
pub struct Pattern<'a> {
  pub function: Function<'a>,
  pub name: String,
}
