use super::*;

pub trait Callable<'a> {
  fn call(&self, args: Vec<VariableValue<'a>>, location: Location<'a>, contexes: &Vec<ContextRef<'a>>) -> VariableValue<'a>;
}

impl<'a> Callable<'a> for RPattern<'a> {
  fn call(&self, args: Vec<VariableValue<'a>>, location: Location<'a>, contexes: &Vec<ContextRef<'a>>) -> VariableValue<'a> {
    self.function.borrow().as_ref().unwrap().call(args, location, contexes)
  }
}

impl<'a> Callable<'a> for RFunction<'a> {
  #[allow(unused_variables)]
  fn call(&self, args: Vec<VariableValue<'a>>, location: Location<'a>, contexes: &Vec<ContextRef<'a>>) -> VariableValue<'a> {
    let mut init_ctx = Context::from(self.body.clone());

    if args.len() != self.args.len() {
        panic!(); // TODO: CompError
    }

    for (from, to) in args.into_iter().zip(self.args.iter()) {
        // TODO: conversion
        init_ctx.variables.insert(to.name.clone(), from);
    }

    if self.has_lhs {
        init_ctx.variables.insert("lhs".to_string(), contexes.last().unwrap().borrow().last_value.clone());
    }
    // TODO: has_self

    let mut contexes = contexes.clone();
    contexes.push(Rc::new(RefCell::new(init_ctx)));

    match self.body.borrow().instructions.last().unwrap() {
        (RASTNode::Block(body), _) => {
            interprete(body.clone(), contexes)
        }
        _ => panic!("Expected function body node to be a block"),
    }
  }
}
