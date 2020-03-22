use super::{ASTNode, TokenTree, Token, construct};
use crate::{Location, error::*};
use std::rc::Rc;

pub fn construct_variable<'a>(
  tree: Rc<TokenTree<'a>>,
  offset: &mut usize
) -> Option<(ASTNode<'a>, Location<'a>)> {
  /*! Tries to match plain variables; does not run any lookup or simulation */
  if let (Token::Symbol(symbol), loc) = &tree.tokens[*offset] {
    if tree.tokens.len() > *offset + 1 {
      if let (Token::Type(t), _) = &tree.tokens[*offset + 1] {
        *offset += 2;
        return Some((ASTNode::TypedVariable(symbol.clone(), t.clone()), loc.clone()))
      }
    }
    *offset += 1;
    Some((ASTNode::Variable(symbol.clone()), loc.clone()))
  } else {
    None
  }
}

pub fn construct_variable_declaration<'a>(
  tree: Rc<TokenTree<'a>>,
  offset: &mut usize
) -> Option<(ASTNode<'a>, Location<'a>)> {
  if let (Token::Let, loc) = &tree.tokens[*offset] {
    if tree.tokens.len() == *offset + 1 {
      CompError::new(
        16,
        String::from("Incomplete variable declaration"),
        CompLocation::from(loc)
      ).print_and_exit();
    }
    if let (Token::Symbol(symbol), _) = &tree.tokens[*offset + 1] {
      if tree.tokens.len() > *offset + 2 {
        if let (Token::Define, loc3) = &tree.tokens[*offset + 2] { // declaration with value
          if tree.tokens.len() == *offset + 3 {
            CompError::new(
              16,
              String::from("Incomplete variable declaration"),
              CompLocation::from(loc3)
            ).append(
              String::from("Variable declaration starts here"),
              CompLocation::from(loc)
            ).print_and_exit();
          }

          *offset += 3;
          let expr = construct(tree.clone(), offset).unwrap_or_else(|| panic!("Unimplemented"));

          if !expr.0.is_valid_expr_term() {
            CompError::new(
              19,
              String::from("Invalid term in variable definition"),
              CompLocation::from(expr.1)
            ).print_and_exit();
          }

          return Some((
            ASTNode::VariableInit(symbol.clone(), Box::new(expr.0)),
            loc.clone()
          ));
        }
      }
      // empty declaration
      *offset += 2;
      return Some((
        ASTNode::VariableDecl(symbol.clone()),
        loc.clone()
      ));
    } else {
      CompError::new(
        17,
        String::from("Invalid term in variable declaration"),
        CompLocation::from(&tree.tokens[*offset + 1].1)
      ).print_and_exit();
    }
  }
  None
}

pub fn construct_variable_definition<'a>(
  tree: Rc<TokenTree<'a>>,
  offset: &mut usize
) -> Option<(ASTNode<'a>, Location<'a>)> {
  if tree.tokens.len() > *offset + 1 {
    if let (Token::Define, define_loc) = &tree.tokens[*offset + 1] {
      if let (Token::Symbol(symbol), sym_loc) = &tree.tokens[*offset] {
        if tree.tokens.len() == *offset + 2 {
          CompError::new(
            19,
            String::from("Incomplete variable definition: expected expression or value"),
            CompLocation::from(define_loc)
          ).print_and_exit()
        }

        *offset += 2;
        let expr = construct(tree.clone(), offset).unwrap_or_else(|| panic!("Unimplemented"));

        if !expr.0.is_valid_expr_term() {
          CompError::new(
            19,
            String::from("Invalid term in variable definition"),
            CompLocation::from(expr.1)
          ).print_and_exit();
        }

        return Some((
          ASTNode::VariableDef(symbol.clone(), Box::new(expr.0)),
          sym_loc.clone()
        ));
      }
    }
  }
  None
}
