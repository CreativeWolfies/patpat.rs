use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct Symbol<'a> { // Only used during interpretation
  pub value: SymbolValue<'a>,
  pub name: String,
}

impl<'a> Symbol<'a> {
  pub fn new(name: String) -> Symbol<'a> {
    Symbol {
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
