use super::{
    ast::{DefineMember, ExprTerm, Expression},
    construct, construct_non_expression,
    token::Operator,
    tuple, ASTKind, ASTNode, Token, TokenTree, AST,
};
use crate::{
    error::{CompError, CompLocation},
    Location,
};
use std::rc::Rc;

// Constructs expressions (yay!)
// TODO: handle <define> at start of instruction

pub fn construct_expression<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
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
        terms: [Push(term1), Push(term2), Op(op1), Op(op2), Push(term3), ...],
      }
      ```
      `opX` may be of any kind.
      This way, the interpreter does not have to traverse the expression tree.

      Operator precedence is not supported: this function and its childs will panic if the user types, eg. `a + b - c`; they'd have to type `(a + b) - c`.
      This is due to the fact that these operators can easily be redefined in structs and precedence assumptions should thus not be made.

      Modifies `offset`.
    */

    let first_term_ops: Vec<Operator> = handle_unary_operators(tree.clone(), offset);
    let mut offset2 = *offset;
    let first_term = construct_non_expression(tree.clone(), &mut offset2);

    if tree.tokens.len() > offset2 {
        // check if we're not at the end of the token list
        if let (Token::Operator(main_op), main_loc) = tree.tokens[offset2].clone() {
            if let Operator::Interpretation = main_op {
                if tree.tokens.len() > offset2 + 2 {
                    if let (Token::Define, define_loc) = &tree.tokens[offset2 + 2] {
                        if first_term_ops.len() > 0 {
                            CompError::new(
                18,
                String::from("Invalid term in interpretation definition: unexpected unary operator"),
                CompLocation::from(&tree.tokens[*offset].1)
              ).print_and_exit();
                        }
                        let res = handle_interpretation_definition(
                            tree.clone(),
                            &mut offset2,
                            first_term.unwrap_or_else(|| panic!("Unimplemented")).0,
                            main_loc,
                            define_loc.clone(),
                        );
                        *offset = offset2;
                        return res;
                    }
                }
            }
            let mut terms: Vec<ExprTerm<'a>> = Vec::new();

            // Append the first term
            append_term(&mut terms, first_term, first_term_ops);

            while tree.tokens.len() > offset2 {
                if let (Token::Operator(op), loc) = tree.tokens[offset2].clone() {
                    // for each operator following the operator suite
                    if main_op != op {
                        // mixed operators
                        if op.is_unary() {
                            break;
                        }

                        err_op_precedence(loc, main_loc);
                    } else if tree.tokens.len() == offset2 + 1 {
                        // operator missing next term
                        CompError::new(
                            8,
                            String::from("Expected term following operator"),
                            CompLocation::from(loc.clone()),
                        )
                        .print_and_exit();
                    }

                    offset2 += 1;

                    let term_ops: Vec<Operator> =
                        handle_unary_operators(tree.clone(), &mut offset2);

                    // Handle <expresssion> <define> <expression>
                    if tree.tokens.len() > offset2 + 1 {
                        if let (Token::Define, _) = &tree.tokens[offset2 + 1] {
                            if term_ops.len() > 0 {
                                CompError::new(
                                    108,
                                    String::from("Unexpected unary operator(s) preceding define"),
                                    CompLocation::from(&tree.tokens[offset2 - 1].1),
                                )
                                .print_and_exit();
                            }
                            *offset = offset2; // term hasn't been parsed yet
                            return handle_definition(tree, offset, terms, main_loc, op);
                        }
                    }

                    let mut res = construct_non_expression(tree.clone(), &mut offset2)
                        .unwrap_or_else(|| panic!("Unimplemented"));

                    // Operator-specific rules
                    if let Operator::Interpretation = op {
                        if term_ops.len() > 0 {
                            CompError::new(
                                18,
                                String::from(
                                    "Invalid term in casting expression: unexpected unary operator",
                                ),
                                CompLocation::from(&res.1),
                            )
                            .print_and_exit();
                        }
                        if let (ASTNode::TypeName(_), _) = &res {
                        } else {
                            CompError::new(
                                18,
                                String::from("Expected TypeName in casting expression"),
                                CompLocation::from(&res.1),
                            )
                            .print_and_exit();
                        }
                    } else if let Operator::MemberAccessor = op {
                        if let (ASTNode::PatternCall(name, body), call_loc) = res {
                            if term_ops.len() == 0 {
                                if let (Token::Pattern(_), _) = &tree.tokens[offset2 - 2] {
                                    res = (ASTNode::MethodCall(name.clone(), body), call_loc);
                                } else {
                                    // restitude `res`
                                    res = (ASTNode::PatternCall(name, body), call_loc);
                                }
                            } else {
                                res = (ASTNode::PatternCall(name, body), call_loc);
                            }
                        }
                    }

                    append_term(&mut terms, Some(res), term_ops);
                    terms.push(ExprTerm::Op(main_op));
                } else {
                    break;
                } // not a binary operator; don't look further
            }

            let initial_loc = tree.tokens[*offset].1.clone();
            *offset = offset2;
            return Some((ASTNode::Expression(Expression { terms }), initial_loc));
        }
    }

    // if the expression consists only of unary operators
    if first_term_ops.len() > 0 {
        let mut terms: Vec<ExprTerm<'a>> = Vec::new();
        let (node, initial_loc) = first_term.unwrap_or_else(|| panic!("Unimplemented"));

        // Append the first term
        append_term(
            &mut terms,
            Some((node, initial_loc.clone())),
            first_term_ops,
        );

        *offset = offset2;

        return Some((
            ASTNode::Expression(Expression { terms }),
            initial_loc.clone(),
        ));
    }
    None
}

fn append_term<'a, 'b>(
    terms: &'b mut Vec<ExprTerm<'a>>,
    term: Option<(ASTNode<'a>, Location<'a>)>,
    termops: Vec<Operator>,
) {
    /*! Appends a term to the terms array of an expression. If `term` is an expression, it gets squashed, otherwise, `term` is simply added to `terms`. */
    match term {
        Some((ASTNode::Expression(mut subexpr), _loc)) => {
            terms.append(&mut subexpr.terms);
        }
        Some((x, loc)) => {
            if !x.is_valid_expr_term() {
                CompError::new(
                    10,
                    String::from("Invalid expression term"),
                    CompLocation::from(&loc),
                )
                .print_and_exit();
            }
            terms.push(ExprTerm::Push(x, loc));
        }
        None => {
            panic!("Unimplemented");
        }
    }
    for op in termops.iter().rev() {
        terms.push(ExprTerm::Op(op.clone()));
    }
}

fn handle_unary_operators<'a>(tree: Rc<TokenTree<'a>>, offset: &mut usize) -> Vec<Operator> {
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
                    CompLocation::from(loc.clone()),
                )
                .print_and_exit();
            }
        } else {
            CompError::new(
                9,
                String::from("Unexpected binary operator"),
                CompLocation::from(loc),
            )
            .print_and_exit();
        }
    }

    term_ops
}

fn handle_interpretation_definition<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
    from: ASTNode<'a>,
    op_loc: Location<'a>,
    define_loc: Location<'a>,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    if let ASTNode::TypeName(from2) = from {
        if let (Token::TypeName(to), _) = tree.tokens[*offset + 1].clone() {
            if tree.tokens.len() <= *offset + 3 {
                CompError::new(
                    18,
                    String::from("Invalid EOF in interpretation definition: expected body"),
                    CompLocation::from(define_loc),
                )
                .print_and_exit();
            }
            if let (Token::Block(tree2), _) = tree.tokens[*offset + 3].clone() {
                let body = AST::parse(tree2, ASTKind::Block);
                *offset += 4;
                return Some((ASTNode::Interpretation(from2, to, body), op_loc));
            } else {
                CompError::new(
                    18,
                    String::from("Invalid term in interpretation definition: expected body"),
                    CompLocation::from(&tree.tokens[*offset + 3].1),
                )
                .print_and_exit();
            }
        } else {
            CompError::new(
                18,
                String::from("Invalid term in interpretation definition: expected TypeName"),
                CompLocation::from(&tree.tokens[*offset + 1].1),
            )
            .print_and_exit();
        }
    } else {
        CompError::new(
            18,
            String::from("Invalid term in interpretation definition: expected TypeName"),
            CompLocation::from(&tree.tokens[*offset - 1].1),
        )
        .print_and_exit();
    }
}

fn handle_definition<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
    terms: Vec<ExprTerm<'a>>,
    loc: Location<'a>,
    op: Operator,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    /*! Handles complex definitions, ie. <expr> <define> <expr>
    Called from `construct_expression`.
    Checks that the operator is a valid operator (MemberAccessor), that the member assigned is a valid member (variable, number or tuple) and returns a ComplexDef.
    */
    if let Operator::MemberAccessor = op {
        let arg: DefineMember;
        match &tree.tokens[*offset] {
            (Token::Symbol(name), _) => {
                *offset += 1;
                arg = DefineMember::Variable(name.clone());
            }
            (Token::Number(num), _) => {
                *offset += 1;
                arg = DefineMember::Number(*num);
            }
            (Token::Tuple(_), _) => {
                arg = DefineMember::Tuple(Box::new(
                    tuple::construct_tuple(tree.clone(), offset)
                        .unwrap_or_else(|| panic!("Error while parsing tuple"))
                        .0,
                ));
            }
            (_, loc) => {
                CompError::new(
                    108,
                    String::from(
                        "Invalid expression term to define: expected string, number or tuple",
                    ),
                    CompLocation::from(loc),
                )
                .print_and_exit();
            }
        }

        *offset += 1; // <define>

        let value = construct(tree, offset).unwrap_or_else(|| unimplemented!());

        Some((
            ASTNode::ComplexDef(Expression { terms }, arg, Box::new(value.0)),
            loc,
        ))
    } else {
        CompError::new(
            108,
            String::from("Cannot define this kind of expression"),
            CompLocation::from(&tree.tokens[*offset - 1].1),
        )
        .append(
            String::from("Can only define expression with member accessors (.)"),
            CompLocation::from(&tree.tokens[*offset - 1].1),
        )
        .print_and_exit();
    }
}

fn err_op_precedence(loc: Location<'_>, main_loc: Location<'_>) -> ! {
    CompError::new(
        107,
        String::from("PatPat does not support operator precedence"),
        CompLocation::from(loc.clone()),
    )
    .append(
        String::from("the main operator is defined here"),
        CompLocation::from(main_loc.clone()),
    )
    .append(
        String::from("consider using parentheses to separate both operators"),
        CompLocation::None,
    )
    .print_and_exit();
}
