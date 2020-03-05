use super::{ASTNode, TokenTree, Token, ast::Expression, token::{Operator}, construct_non_expression};
use crate::{Location, error::{CompError, CompLocation}};
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

    Modifies `offset`.
  */

  // TODO: handle interpretation assignements
  // TODO: handle unary operators
  let first_term_ops: Vec<Operator> = handle_unary_operators(tree.clone(), offset);

  if tree.tokens.len() > *offset + 1 { // check if we're not at the end of the token list
    if let (Token::Operator(main_op), main_loc) = tree.tokens[*offset + 1].clone() {
      let mut terms: Vec<(ASTNode<'a>, Vec<Operator>, Location<'a>)> = Vec::new();
      let mut ops: Vec<Operator> = Vec::new();
      let mut offset2 = *offset;

      // Append the first term
      append_term(&mut terms, &mut ops, construct_non_expression(tree.clone(), &mut offset.clone()), first_term_ops);

      while tree.tokens.len() > offset2 + 1 {
        if let (Token::Operator(op), loc) = tree.tokens[offset2 + 1].clone() { // for each operator following the operator suite
          if main_op != op { // mixed operators
            if op.is_unary() {break}

            CompError::new(
              107,
              String::from("PatPat does not support operator precedence"),
              CompLocation::from(loc.clone())
            ).append(
              String::from("Main operator is defined here"),
              CompLocation::from(main_loc.clone())
            ).append(
              String::from("Consider using parentheses"),
              CompLocation::from(loc.clone())
            ).print_and_exit();
          } else if tree.tokens.len() == offset2 + 2 { // operator missing next term
            CompError::new(
              8,
              String::from("Expected term following operator"),
              CompLocation::from(loc.clone())
            ).print_and_exit();
          }

          offset2 += 2;

          let termops: Vec<Operator> = handle_unary_operators(tree.clone(), &mut offset2);

          append_term(&mut terms, &mut ops, construct_non_expression(tree.clone(), &mut offset2.clone()), termops);
          ops.push(main_op);
        } else { break } // not a binary operator; don't look further
      }

      let initial_loc = tree.tokens[*offset].1.clone();
      *offset = offset2 + 1;
      return Some((ASTNode::Expression(Expression {
        terms,
        ops
      }), initial_loc));
    }
  }

  // if the expression consists only of unary operators
  if first_term_ops.len() > 0 {
    let mut terms: Vec<(ASTNode<'a>, Vec<Operator>, Location<'a>)> = Vec::new();
    let mut ops: Vec<Operator> = Vec::new();
    let (node, initial_loc) = construct_non_expression(tree.clone(), &mut offset.clone()).unwrap_or_else(|| panic!("Unimplemented"));

    // Append the first term
    append_term(&mut terms, &mut ops, Some((node, initial_loc.clone())), first_term_ops);

    *offset += 1;

    return Some((ASTNode::Expression(Expression {
      terms,
      ops
    }), initial_loc.clone()));
  }
  None
}

fn append_term<'a, 'b>(
  terms: &'b mut Vec<(ASTNode<'a>, Vec<Operator>, Location<'a>)>,
  ops: &'b mut Vec<Operator>,
  term: Option<(ASTNode<'a>, Location<'a>)>,
  termops: Vec<Operator>
) {
  /*! Appends a term to the terms array of an expression. If `term` is an expression, it gets squashed, otherwise, `term` is simply added to `terms`. */
  // TODO: tuples!
  match term {
    Some((ASTNode::Expression(mut subexpr), _loc)) => {
      terms.append(&mut subexpr.terms);
      ops.append(&mut subexpr.ops);
    },
    Some((x, loc)) => {
      terms.push((x, termops, loc));
    },
    None => {
      panic!("Unimplemented");
    }
  }
}

fn handle_unary_operators<'a>(
  tree: Rc<TokenTree<'a>>,
  offset: &mut usize
) -> Vec<Operator> {
  /*!
  Handles unary operators; returns an array of unary operators preceding a term.
  Modifies `offset`.
  */
  let mut term_ops: Vec<Operator> = Vec::new();

  while let (Token::Operator(operator), loc) = &tree.tokens[*offset] {
    if operator.is_unary() {
      term_ops.push(operator.clone());
      *offset += 1;
      if tree.tokens.len() <= *offset {
        CompError::new(
          8,
          String::from("Expected term following operator"),
          CompLocation::from(loc.clone())
        ).print_and_exit();
      }
    } else {
      CompError::new(
        9,
        String::from("Unexpected binary operator"),
        CompLocation::from(loc)
      ).print_and_exit();
    }
  }

  term_ops
}
