use super::*;
use std::fmt;

#[derive(Clone)]
pub struct RFunction<'a> {
  pub args: Vec<FunctionArg>, // TODO: RFunctionArg
  pub body: Rc<RefCell<RAST<'a>>>,
  pub has_self: bool,
  pub has_lhs: bool,
}

impl<'a> From<(Function<'a>, RASTWeak<'a>, Location<'a>)> for RFunction<'a> {
  /**
    Creates an RFunction off a Function and a parent RAST.
  */
  fn from(input: (Function<'a>, RASTWeak<'a>, Location<'a>)) -> RFunction<'a> {
    let function = input.0;
    let parent = input.1;
    let loc = input.2;

    let init = Rc::new(RefCell::new(RAST::new(parent)));

    for arg in function.args.iter() {
      init.borrow_mut().variables.push(Rc::new(RefCell::new(RSymbol::new(arg.name.clone()))));
    }
    if function.has_lhs {
      init.borrow_mut().variables.push(Rc::new(RefCell::new(RSymbol::new(String::from("lhs")))));
    }
    if function.has_self {
      init.borrow_mut().variables.push(Rc::new(RefCell::new(RSymbol::new(String::from("self")))));
    }

    let body = RAST::resolve(function.body, Rc::downgrade(&init));

    init.borrow_mut().instructions.push(
      (RASTNode::Block(body), loc)
    );

    RFunction {
      args: function.args,
      body: init,
      has_lhs: function.has_lhs,
      has_self: function.has_self,
    }
  }
}

// NOTE: this assumes that the block is the last instruction of the `init` RAST
impl<'a> fmt::Debug for RFunction<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut builder = f.debug_struct("RFunction");
    builder.field("args", &self.args);
    let instructions = &self.body.borrow().instructions;
    match &instructions[instructions.len() - 1].0 {
      RASTNode::Block(expr) => {
        builder.field("body", &expr.borrow());
      },
      _ => {
        builder.field("body", &String::from("<error>"));
      }
    }
    builder.field("has_lhs", &self.has_lhs);
    builder.field("has_self", &self.has_self);
    builder.finish()
  }
}
