use super::*;
use std::fmt;

pub struct IntPattern<T> {
    pub name: String,
    pub fun: T,
}

impl<'a, T> IntPattern<T>
where
    T: Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>,
{
    pub fn new(name: String, fun: T) -> IntPattern<T> {
        IntPattern { name, fun }
    }
}

impl<'a, T> Callable<'a> for IntPattern<T>
where
    T: Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>,
{
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

impl<'a, T> fmt::Debug for IntPattern<T>
where
    T: Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntPattern({})", self.name)
    }
}
