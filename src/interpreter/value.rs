use super::*;

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum VariableValue<'a> {
    String(String),
    Number(f64),
    Boolean(bool),
    Instance(RStructRef<'a>, RefCell<HashMap<String, VariableValue<'a>>>), // TODO
    Tuple(Vec<VariableValue<'a>>),
    Nil,
}
