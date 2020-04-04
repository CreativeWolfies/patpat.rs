use super::*;

#[derive(Debug, Clone)]
pub struct RPattern<'a> {
    pub name: String,
    pub function: Option<RFunction<'a>>,
}

impl<'a> RPattern<'a> {
    pub fn new(name: String) -> RPattern<'a> {
        RPattern {
            name: name,
            function: None,
        }
    }
}
