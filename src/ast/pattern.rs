use super::*;

#[derive(Debug, Clone)]
pub struct Pattern<'a> {
    pub function: Function<'a>,
    pub name: String,
}
