use super::{ASTNode, TokenTree, Token, ast};

pub fn mangle_pattern_declaration(tree: &mut TokenTree) -> Option<ASTNode> {
  if let Token::Pattern(p) = &tree.tokens[0] {
    if let Token::Define = &tree.tokens[1] {
      let pattern = p.clone();
      let mut iter = tree.tokens.clone().into_iter();
      iter.next()?;
      iter.next()?;
      let raw_fn = (iter.next()?, iter.next()?, iter.next()?);
      return match ast::Function::parse(raw_fn) {
        Some(f) => Some(ASTNode::PatternDecl(ast::Pattern {
          function: f,
          name: pattern.name
        })),
        None => None
      }
    }
  }
  None
}
