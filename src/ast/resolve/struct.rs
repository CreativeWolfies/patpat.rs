use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct RStruct<'a> {
  pub name: TypeName,
  pub context: Option<RASTRef<'a>>,
}

impl<'a> RStruct<'a> {
  pub fn new(name: TypeName) -> RStruct<'a> {
    RStruct {
      name: name,
      context: None,
    }
  }
}
