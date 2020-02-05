use super::{ASTNode, TokenTree, Token, ast};

pub fn construct_pattern_declaration(tree: &mut TokenTree, offset: &mut usize) -> Option<ASTNode> {
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
  if let Token::Pattern(p) = &tree.tokens[*offset] {
    if tree.tokens.len() == *offset {return None}
    if let Token::Define = &tree.tokens[*offset + 1] {
      let pattern = p.clone();
      let mut iter = tree.tokens.clone().into_iter().skip(*offset + 2);
      let raw_fn = (iter.next()?, iter.next()?, iter.next()?);
      return match ast::Function::parse(raw_fn) {
        Some(f) => {
          *offset += 5;
          Some(ASTNode::PatternDecl(ast::Pattern {
            function: f,
            name: pattern.name
          }))
        },
        None => None
      }
    }
  }
  None
}
