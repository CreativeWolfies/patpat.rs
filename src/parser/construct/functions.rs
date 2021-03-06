use super::{ast, ASTKind, ASTNode, Token, TokenTree, AST};
use crate::Location;
use crate::error::CompError;
use std::rc::Rc;

pub fn construct_pattern_declaration<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    /*! Tries to match pattern declarations:
     *
     * ```patpat
     * (args) => {
     *   // body
     * }
     * ```
     *
     * Pattern declarations require the following tokens:
     * - Pattern
     * - Define
     * - Tuple
     * - Arrow
     * - Block
     */
    // PATTERN_DEFINITION = PATTERN, {whitespace}, DEFINE, {whitespace}, FUNCTION;
    if let Token::Pattern(name) = &tree.tokens[*offset].0 {
        if tree.tokens.len() == *offset + 1 {
            return None;
        }
        if let Token::Define = &tree.tokens[*offset + 1].0 {
            let mut iter = tree.tokens.clone().into_iter().skip(*offset + 2);
            return match ast::Function::parse(iter.next()?, iter.next()?, iter.next()?, true) {
                Some(f) => {
                    let location = tree.tokens[*offset].1.clone();
                    *offset += 5;
                    Some((
                        ASTNode::PatternDecl(ast::Pattern {
                            function: f,
                            name: name.to_string(),
                        }),
                        location,
                    ))
                }
                None => None,
            };
        }
    }
    None
}

pub fn construct_pattern_call<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    /*! Handles construct calls, ie `'pattern(...)` */
    // PATTERN_CALL = PATTERN, {whitespace}, TUPLE;
    if let Token::Pattern(name) = &tree.tokens[*offset].0 {
        if tree.tokens.len() == *offset + 1 {
            return None;
        }
        if let Token::Tuple(t) = &tree.tokens[*offset + 1].0 {
            let args = AST::parse(t.clone(), ASTKind::Tuple);
            let location = tree.tokens[*offset].1.clone();
            *offset += 2;
            return Some((ASTNode::PatternCall(name.to_string(), args), location));
        }
    }
    None
}

pub fn construct_standalone_function<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    /*! Handles standalone functions, that is, functions that are not introduced as patterns.
     * These may occur in expressions, parameter or as a return value.
     */
    if tree.tokens.len() <= *offset + 2 {
        None
    } else {
        let res = ast::Function::parse(
            tree.tokens[*offset].clone(),
            tree.tokens[*offset + 1].clone(),
            tree.tokens[*offset + 2].clone(),
            false,
        )?;
        let location = tree.tokens[*offset].1.clone();
        *offset += 3;
        Some((ASTNode::Function(res), location))
    }
}

pub fn construct_standalone_pattern<'a>(
    tree: Rc<TokenTree<'a>>,
    offset: &mut usize,
) -> Option<(ASTNode<'a>, Location<'a>)> {
    if let (Token::Pattern(p), loc) = &tree.tokens[*offset] {
        if tree.tokens.len() > *offset + 1 {
            if let (Token::Separator, _) = &tree.tokens[*offset + 1] {}
            if let (Token::Operator(_), _) = &tree.tokens[*offset + 1] {}
            else {
                CompError::new(
                    22,
                    String::from("Invalid standalone pattern: the next term may be wrangled with it"),
                    loc.into()
                ).append(
                    String::from("Consider using partial application, encapsulating the pattern in a tuple or putting a separator (,) before this term"),
                    (&tree.tokens[*offset + 1].1).into()
                ).print_and_exit();
            }
        }
        *offset += 1;
        Some((ASTNode::Pattern(p.clone()), loc.clone()))
    } else {
        None
    }
}
