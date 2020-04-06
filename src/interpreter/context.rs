use super::*;

use std::cell::RefCell;
use std::collections::HashMap;

pub type ContextRef<'a> = Rc<RefCell<Context<'a>>>;

pub struct Context<'a> {
    pub depth: usize,
    pub variables: HashMap<String, VariableValue<'a>>,
}

impl<'a> From<RASTRef<'a>> for Context<'a> {
    fn from(ast: RASTRef<'a>) -> Context<'a> {
        let mut variables = HashMap::with_capacity(ast.borrow().variables.len());

        for var in &ast.borrow().variables {
            variables.insert(var.borrow().name.clone(), VariableValue::Nil);
        }

        Context {
            depth: ast.borrow().depth,
            variables,
        }
    }
}
