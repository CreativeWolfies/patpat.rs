use super::{ASTNode, TokenTree, Token, ast};

pub fn construct_pattern_declaration(tree: &mut TokenTree) -> Option<ASTNode> {
  /*
    Pattern declarations require the following tokens:
    - Pattern
    - Define
    - Tuple
    - Arrow
    - Block
  */
  if let Token::Pattern(p) = &tree.tokens[0] {
    if let Token::Define = &tree.tokens[1] {
      let pattern = p.clone();
      let mut iter = tree.tokens.clone().into_iter();
      iter.next()?;
      iter.next()?;
      let raw_fn = (iter.next()?, iter.next()?, iter.next()?);
      return match ast::Function::parse(raw_fn) {
        Some(f) => {
          // TODO: consider swapping the order
          for _ in 0..5 {
            tree.tokens.remove(0);
          }
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
