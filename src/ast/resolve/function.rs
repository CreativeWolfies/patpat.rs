use super::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct RFunction<'a> {
  pub args: Vec<FunctionArg>, // TODO: RFunctionArg
  pub body: Rc<RefCell<RAST<'a>>>,
  pub has_self: bool,
  pub has_lhs: bool,
}

impl<'a> From<(Function<'a>, Weak<RefCell<RAST<'a>>>)> for RFunction<'a> {
  /**
    Creates an RFunction off a Function and a parent RAST.
  */
  fn from(input: (Function<'a>, Weak<RefCell<RAST<'a>>>)) -> RFunction<'a> {
    let function = input.0;
    let parent = input.1;
    RFunction {
      args: function.args,
      body: RAST::resolve(function.body, parent),
      has_lhs: function.has_lhs,
      has_self: function.has_self,
    }
  }
}
