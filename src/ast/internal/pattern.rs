use super::*;
use std::fmt;

pub struct IntPattern<T>
where
    T: for<'a> Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>,
{
    pub name: String,
    pub fun: T,
}

impl<T> IntPattern<T>
where
    T: for<'a> Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>,
{
    pub fn new(name: String, fun: T) -> IntPattern<T> {
        IntPattern { name, fun }
    }
}

impl<'a, T> Callable<'a> for IntPattern<T>
where
    T: for<'b> Fn(Vec<VariableValue<'b>>, Location<'b>, &Vec<ContextRef<'b>>) -> VariableValue<'b>,
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

impl<T> fmt::Debug for IntPattern<T>
where
    T: for<'a> Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntPattern({})", self.name)
    }
}
