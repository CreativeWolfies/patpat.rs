use super::{ASTNode, AST, ASTKind, TokenTree, Token, ast};

pub fn construct_pattern_declaration(tree: &TokenTree, offset: &mut usize) -> Option<ASTNode> {
  /*! Tries to match pattern declarations:
  *
  * ```patpat
  * (args) => {
  *   // body
  * }
  * ```
  * Pattern declarations require the following tokens:
  * - Pattern
  * - Define
  * - Tuple
  * - Arrow
  * - Block
  */
  // PATTERN_DEFINITION = PATTERN, {whitespace}, DEFINE, {whitespace}, FUNCTION;
  if let Token::Pattern(name) = &tree.tokens[*offset].0 {
    if tree.tokens.len() == *offset + 1 {return None}
    if let Token::Define = &tree.tokens[*offset + 1].0 {
      let mut iter = tree.tokens.clone().into_iter().skip(*offset + 2);
      return match ast::Function::parse(iter.next()?, iter.next()?, iter.next()?) {
        Some(f) => {
          *offset += 5;
          Some(ASTNode::PatternDecl(ast::Pattern {
            function: f,
            name: name.to_string()
          }))
        },
        None => None
      }
    }
  }
  None
}

pub fn construct_pattern_call(tree: &TokenTree, offset: &mut usize) -> Option<ASTNode> {
  // PATTERN_CALL = PATTERN, {whitespace}, TUPLE;
  if let Token::Pattern(name) = &tree.tokens[*offset].0 {
    if tree.tokens.len() == *offset + 1 {return None}
    if let Token::Tuple(t) = &tree.tokens[*offset + 1].0 {
      let args = AST::parse(t.clone(), ASTKind::ArgTuple);
      *offset += 2;
      return Some(ASTNode::PatternCall(name.to_string(), args));
    }
  }
  None
}

pub fn construct_standalone_function(tree: &TokenTree, offset: &mut usize) -> Option<ASTNode> {
  if tree.tokens.len() <= *offset + 2 {
    None
  } else {
    let res = ast::Function::parse(
      tree.tokens[*offset].clone(),
      tree.tokens[*offset + 1].clone(),
      tree.tokens[*offset + 2].clone()
    )?;
    *offset += 3;
    Some(ASTNode::Function(res))
  }
}
