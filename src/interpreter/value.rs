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

impl<'a> PartialEq for VariableValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            VariableValue::String(x) => {
                if let VariableValue::String(y) = other {
                    x == y
                } else {
                    false
                }
            }
            VariableValue::Number(x) => {
                if let VariableValue::Number(y) = other {
                    x == y
                } else {
                    false
                }
            }
            VariableValue::Boolean(x) => {
                if let VariableValue::Boolean(y) = other {
                    x == y
                } else {
                    false
                }
            }
            VariableValue::Nil => {
                if let VariableValue::Nil = other {
                    true
                } else {
                    false
                }
            }
            VariableValue::Tuple(x) => {
                if let VariableValue::Tuple(y) = other {
                    x == y
                } else {
                    false
                }
            }
            VariableValue::Instance(_, _) => false, // instance comparison is not yet supported
        }
    }
}
