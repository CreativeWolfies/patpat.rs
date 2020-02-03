use super::parser::{token::{Type, Token, TokenTree}, manglers};

#[derive(Debug)]
#[derive(Clone)]
pub enum ASTNode {
  Function(Function),
  PatternDecl(Pattern),
  PatternCall(String, AST), // name, tuple
  Variable(String),
  TypedVariable(String, Type),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct AST {
  pub instructions: Vec<ASTNode>,
  pub kind: ASTKind,
}

impl AST {
  pub fn parse(raw: TokenTree, kind: ASTKind) -> AST {
    let mut raw = raw.clone();
    let mut instructions = Vec::<ASTNode>::new();
    while raw.tokens.len() > 0 {
      match manglers::mangle(&mut raw) {
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

#[derive(Debug)]
#[derive(Clone)]
pub enum ASTKind {
  Tuple,
  ArgTuple,
  Block,
  Expression,
  File,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Function {
  pub args: Vec<FunctionArg>,
  pub body: AST,
  pub has_self: bool,
  pub has_lhs: bool,
}

impl Function {
  pub fn parse(raw: (Token, Token, Token)) -> Option<Function> {
    match raw {
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
