use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct Pattern<'a> {
  pub function: Function<'a>,
  pub name: String,
}
