// tokens that will end up in the AST

#[derive(Debug)]
pub enum Token {
    Boolean(Boolean),
    Symbol(Symbol),
    Define,
    Let,
    Pattern(Pattern),

}

impl Token {
  pub fn from_match(caps: &super::Captures, matcher: &Kind) -> Token {
      match matcher {
          Kind::Boolean => Token::Boolean(Boolean {
              state: caps.get(1).unwrap().as_str() == "true"
          }),
          Kind::Let => Token::Let,
          Kind::Symbol => Token::Symbol(Symbol {
              name: String::from(caps.get(0).unwrap().as_str())
          }),
          Kind::Define => Token::Define,
          Kind::Pattern => Token::Pattern(Pattern {
              name: String::from(caps.get(0).unwrap().as_str())
          }),
          _ => {
              eprintln!("Unknown token kind: {:?}", matcher);
              std::process::exit(4);
          },
      }
  }
}

// value-less tokens
#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub enum Kind {
    Boolean,
    Symbol,
    Define,
    Space,
    Let,
    Comment,
    Pattern
}

#[derive(Debug)]
pub struct Boolean {
    pub state: bool
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String
}

#[derive(Debug)]
pub struct Pattern {
    pub name: String
}
