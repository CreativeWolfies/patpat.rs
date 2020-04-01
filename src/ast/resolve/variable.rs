use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct RSymbol<'a> { // Only used during interpretation
  pub value: SymbolValue<'a>, // TODO: remove
  pub name: String,
}

impl<'a> RSymbol<'a> {
  pub fn new(name: String) -> RSymbol<'a> {
    RSymbol {
      value: SymbolValue::None,
      name
    }
  }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum SymbolValue<'a> {
  Number(f64),
  String(String),
  Function(Function<'a>),
  None,
  // TODO: Instance
}
