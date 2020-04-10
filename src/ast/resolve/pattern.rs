use super::*;

#[derive(Debug, Clone)]
pub struct RPattern<'a> {
    pub name: String,
    pub function: RefCell<Option<RFunction<'a>>>,
}

impl<'a> RPattern<'a> {
    pub fn new(name: String) -> RPattern<'a> {
        RPattern {
            name: name,
            function: RefCell::new(None),
        }
    }

    pub fn set_function(&self, function: RFunction<'a>) {
        *self.function.borrow_mut() = Some(function);
    }
}
