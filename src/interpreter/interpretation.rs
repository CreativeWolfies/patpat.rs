use super::*;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn interprete_interpretation<'a>(value: VariableValue<'a>, (into, body): (RStructWeak<'a>, RASTRef<'a>)) -> VariableValue<'a> {
  let mut init_ctx = Context::from(body.clone());
  init_ctx.variables.insert(String::from("from"), value);
  let res = VariableValue::Instance(into.upgrade().unwrap(), Rc::new(RefCell::new(HashMap::new())));
  init_ctx.variables.insert(String::from("to"), res.clone());

  match body.borrow().instructions.last() {
    Some((RASTNode::Block(body), _)) => interprete(body.clone(), vec![Rc::new(RefCell::new(init_ctx))]),
    _ => panic!("Expected interpretation body to end with a block")
  };

  res
}
