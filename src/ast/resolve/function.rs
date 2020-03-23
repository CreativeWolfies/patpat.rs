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
  fn from(input: (Function<'a>, Weak<RefCell<RAST<'a>>>)) -> RFunction<'a> {
    let function = input.0;
    let ast = input.1;
    RFunction {
      args: function.args,
      body: RAST::resolve(function.body, ast),
      has_lhs: function.has_lhs,
      has_self: function.has_self,
    }
  }
}
