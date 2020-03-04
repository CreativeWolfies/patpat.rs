use super::{ASTNode, AST, ASTKind, TokenTree, Token, ast, ast::Expression, token::{Operator}, construct_non_expression};
use crate::{Location, error::{CompError, CompInfo, CompLocation}};
use std::rc::Rc;

// Constructs expressions (yay!)

pub fn construct_expression<'a>(
  tree: Rc<TokenTree<'a>>,
  offset: &mut usize
) -> Option<(ASTNode<'a>, Location<'a>)> {
  /*! Constructs expressions out of the token tree.
    This function is ran before any of the other non-expression functions.
    Non-expression functions are handled by a second function in parser/construct/mod.rs, as to let out expression constructor query it to parse the terms making up the expression.

    An expression looks like this:
    `<term> <op> <term> {<op> <term>}`

    Where `<term>` can be preceded by an arbitrary number of unary operators.

    The returned object looks like this:
    ```
    Expression {
      terms: [term1, term2, term3, ...],
      ops: [op1, op2, op3]
    }
    ```
    This object may have different operators in the `ops` array: this is due to the fact that nested expression (`a <op> (b <op> c)` for instance) are squashed into the topmost one.
    This way, the interpreter does not have to traverse the expression tree.
  */

  // TODO: handle interpretation assignements
  // TODO: handle unary operators
  if tree.tokens.len() > *offset + 1 { // check if we're not at the end of the token list
    if let (Token::Operator(main_op), main_loc) = tree.tokens[*offset + 1].clone() {
      let mut terms: Vec<(ASTNode<'a>, Location<'a>)> = Vec::new();
      let mut ops: Vec<Operator> = Vec::new();

      // Append the first term
      append_term(&mut terms, &mut ops, construct_non_expression(tree.clone(), &mut offset.clone()));

      let mut offset2 = *offset;
      while tree.tokens.len() > offset2 + 1 {
        if let (Token::Operator(op), loc) = tree.tokens[offset2 + 1].clone() { // for each operator following the operator suite
          if main_op != op { // mixed operators
            CompError::new(
              107, vec![
                CompInfo::new("PatPat does not support operator precedence", CompLocation::from(loc.clone())),
                CompInfo::new("Main operator is defined here", CompLocation::from(main_loc.clone())),
                CompInfo::new("Consider using parentheses", CompLocation::from(loc.clone()))
              ]
            ).print_and_exit();
          } else if tree.tokens.len() == offset2 + 2 { // operator missing next term
            CompError::new(
              8, vec![
                CompInfo::new("Expected term following operator", CompLocation::from(loc.clone()))
              ]
            ).print_and_exit();
          }

          append_term(&mut terms, &mut ops, construct_non_expression(tree.clone(), &mut (offset2 + 2)));
          ops.push(main_op);

          offset2 += 2;
        } else {
          break
        }
      }
      println!("{:?}", terms);
      println!("{:?}", ops);
      let initial_loc = tree.tokens[*offset].1.clone();
      *offset = offset2 + 1;
      return Some((ASTNode::Expression(Expression {
        terms,
        ops
      }), initial_loc));
    }
  }
  None
}

fn append_term<'a, 'b>(
  terms: &'b mut Vec<(ASTNode<'a>, Location<'a>)>,
  ops: &'b mut Vec<Operator>,
  term: Option<(ASTNode<'a>, Location<'a>)>
) {
  /*! Appends a term to the terms array of an expression. If `term` is an expression, it gets squashed, otherwise, `term` is simply added to `terms`. */
  // TODO: tuples!
  match term {
    Some((ASTNode::Expression(mut subexpr), _loc)) => {
      terms.append(&mut subexpr.terms);
      ops.append(&mut subexpr.ops);
    },
    Some((x, loc)) => {
      terms.push((x, loc));
    },
    None => {
      panic!("Unimplemented");
    }
  }
}
