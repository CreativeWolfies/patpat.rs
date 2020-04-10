use super::*;
use std::fmt;

type CallableFn<'a> =
    dyn Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>;

pub struct IntPattern<'a> {
    pub name: String,
    pub fun: Box<CallableFn<'a>>,
}

impl<'a> IntPattern<'a> {
    pub fn new(name: String, fun: Box<CallableFn<'a>>) -> IntPattern<'a> {
        IntPattern { name, fun }
    }
}

impl<'a> Callable<'a> for IntPattern<'a> {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn call(
        &self,
        args: Vec<VariableValue<'a>>,
        location: Location<'a>,
        contexes: &Vec<ContextRef<'a>>,
    ) -> VariableValue<'a> {
        (self.fun)(args, location, contexes)
    }
}

impl<'a> fmt::Debug for IntPattern<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntPattern({})", self.name)
    }
}
