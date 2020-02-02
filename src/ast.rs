use super::parser::{token::{Type, Token, TokenTree}};

pub enum ASTNode {
  Function(Function),
  PatternDecl(Pattern),
  PatternCall(String, AST), // name, tuple
  Variable(String),
  TypedVariable(String, Type),
}

pub struct AST {
  pub instructions: Vec<ASTNode>,
  pub kind: ASTKind,
}

impl AST {
  pub fn parse(raw: TokenTree, kind: ASTKind) -> Option<AST> {
    None // TODO
  }
}

pub enum ASTKind {
  Tuple,
  ArgTuple,
  Block,
  Expression,
}

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
        let tuple = AST::parse(raw_tuple, ASTKind::ArgTuple)?;
        let body = AST::parse(raw_body, ASTKind::Block)?;
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

pub struct FunctionArg {
  pub argtype: Option<Type>,
  pub name: String,
}

pub struct Pattern {
  pub function: Function,
  pub name: String,
}
