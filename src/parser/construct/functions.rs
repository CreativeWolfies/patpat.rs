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
  if let Token::Pattern(name) = &tree.tokens[*offset] {
    if tree.tokens.len() == *offset {return None}
    if let Token::Define = &tree.tokens[*offset + 1] {
      let mut iter = tree.tokens.clone().into_iter().skip(*offset + 2);
      let raw_fn = (iter.next()?, iter.next()?, iter.next()?);
      return match ast::Function::parse(raw_fn) {
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
  if let Token::Pattern(name) = &tree.tokens[*offset] {
    if tree.tokens.len() == *offset {return None}
    if let Token::Tuple(t) = &tree.tokens[*offset + 1] {
      let args = AST::parse(t.clone(), ASTKind::ArgTuple);
      *offset += 2;
      return Some(ASTNode::PatternCall(name.to_string(), args));
    }
  }
  None
}
