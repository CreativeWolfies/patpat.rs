use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use token::Operator;

pub type InstanceRef<'a> = Rc<RefCell<HashMap<String, VariableValue<'a>>>>;

#[derive(Debug, Clone)]
pub enum VariableValue<'a> {
    String(String),
    Number(f64),
    Boolean(bool),
    Instance(RStructRef<'a>, InstanceRef<'a>), // TODO
    Type(RStructRef<'a>),
    Tuple(Vec<VariableValue<'a>>),
    Function(Rc<(dyn Callable<'a> + 'a)>),
    Nil,
    Bail,
}

impl<'a> fmt::Display for VariableValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableValue::String(x) => write!(f, "{}", x),
            VariableValue::Number(x) => write!(f, "{}", x),
            VariableValue::Boolean(x) => write!(f, "{}", x),
            VariableValue::Instance(x, _) => write!(f, "[{} instance]", x.borrow().name),
            VariableValue::Type(x) => write!(f, "[{} type]", x.borrow().name),
            VariableValue::Tuple(x) => write!(
                f,
                "({})",
                x.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            VariableValue::Nil => write!(f, "nil"),
            VariableValue::Bail => write!(f, "bail"),
            VariableValue::Function(fun) => write!(f, "[function {}]", fun.get_name()),
        }
    }
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
            VariableValue::Bail => {
                if let VariableValue::Bail = other {
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
            VariableValue::Function(_) => false,    // function comparison is not yet supported
            VariableValue::Type(_) => false,        // type comparison is not yet supported
        }
    }
}

impl<'a> BinaryOp<'a, Self> for VariableValue<'a> {
    fn binary_op(self, b: Self, op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        if let Operator::Eq = op {
            VariableValue::Boolean(b == self)
        } else if let Operator::Neq = op {
            VariableValue::Boolean(b != self)
        } else {
            match self {
                VariableValue::String(x) => match b {
                    VariableValue::String(y) => x.binary_op(y, op, loc),
                    VariableValue::Number(y) => x.binary_op(y, op, loc),
                    VariableValue::Boolean(y) => x.binary_op(y, op, loc),
                    VariableValue::Nil => x.binary_op("nil".to_string(), op, loc),
                    VariableValue::Bail => x.binary_op("bail".to_string(), op, loc),
                    VariableValue::Function(y) => {
                        x.binary_op(format!("[function {}]", y.get_name()), op, loc)
                    }
                    VariableValue::Type(y) => {
                        x.binary_op(format!("[{} type]", y.borrow().name), op, loc)
                    }
                    VariableValue::Instance(y, _) => {
                        x.binary_op(format!("[{} instance]", y.borrow().name), op, loc)
                    }
                    _ => err_mixed_types(loc),
                },
                VariableValue::Number(x) => match b {
                    VariableValue::Number(y) => x.binary_op(y, op, loc),
                    VariableValue::String(y) => x.binary_op(y, op, loc),
                    _ => err_mixed_types(loc),
                },
                VariableValue::Boolean(x) => match b {
                    VariableValue::Boolean(y) => x.binary_op(y, op, loc),
                    VariableValue::String(y) => x.binary_op(y, op, loc),
                    _ => err_mixed_types(loc),
                },
                VariableValue::Nil => match b {
                    VariableValue::String(y) => "nil".to_string().binary_op(y, op, loc),
                    _ => err_mixed_types(loc),
                },
                VariableValue::Bail => match b {
                    VariableValue::String(y) => "bail".to_string().binary_op(y, op, loc),
                    _ => err_mixed_types(loc),
                },
                VariableValue::Tuple(vec) => match b {
                    VariableValue::Tuple(b_vec) => {
                        let mut res: Vec<VariableValue<'a>> = Vec::with_capacity(vec.len());
                        for (a, b) in vec.into_iter().zip(b_vec.into_iter()) {
                            res.push(a.binary_op(b, op, loc.clone()));
                        }
                        VariableValue::Tuple(res)
                    }
                    _ => err_mixed_types(loc),
                },
                VariableValue::Function(x) => {
                    if let VariableValue::String(y) = b {
                        format!("[function {}]", x.get_name()).binary_op(y, op, loc)
                    } else {
                        err_invalid_op(loc)
                    }
                },
                VariableValue::Type(x) => {
                    if let VariableValue::String(y) = b {
                        format!("[{} type]", x.borrow().name).binary_op(y, op, loc)
                    } else {
                        err_invalid_op(loc)
                    }
                },
                VariableValue::Instance(x, _) => {
                    if let VariableValue::String(y) = b {
                        format!("[{} instance]", x.borrow().name).binary_op(y, op, loc)
                    } else {
                        err_invalid_op(loc)
                    }
                },
                // _ => unimplemented!(),
            }
        }
    }
}

impl<'a> UnaryOp<'a> for VariableValue<'a> {
    fn unary_op(self, op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        match self {
            VariableValue::String(x) => x.unary_op(op, loc),
            VariableValue::Number(x) => x.unary_op(op, loc),
            VariableValue::Boolean(x) => x.unary_op(op, loc),
            VariableValue::Nil => err_invalid_op(loc),
            VariableValue::Tuple(vec) => {
                let mut res: Vec<VariableValue<'a>> = Vec::with_capacity(vec.len());
                for a in vec.into_iter() {
                    res.push(a.unary_op(op, loc.clone()));
                }
                VariableValue::Tuple(res)
            }
            _ => unimplemented!(),
        }
    }
}

// Implementations of BinaryOp and UnaryOp for the different primitives

impl<'a> BinaryOp<'a, Self> for String {
    fn binary_op(self, b: Self, op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        if let Operator::Add = op {
            VariableValue::String(b + &self)
        } else {
            err_invalid_op(loc);
        }
    }
}

impl<'a> UnaryOp<'a> for String {
    fn unary_op(self, _op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        err_invalid_op(loc);
    }
}

impl<'a> BinaryOp<'a, bool> for String {
    fn binary_op(self, b: bool, op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        if let Operator::Add = op {
            VariableValue::String(b.to_string() + &self)
        } else {
            err_invalid_op(loc);
        }
    }
}

impl<'a> BinaryOp<'a, f64> for String {
    fn binary_op(self, b: f64, op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        if let Operator::Add = op {
            VariableValue::String(b.to_string() + &self)
        } else {
            err_invalid_op(loc);
        }
    }
}

impl<'a> BinaryOp<'a, Self> for f64 {
    fn binary_op(self, b: Self, op: &Operator, _loc: Location<'a>) -> VariableValue<'a> {
        match op {
            Operator::Add => VariableValue::Number(b + self),
            Operator::Sub => VariableValue::Number(b - self),
            Operator::Mul => VariableValue::Number(b * self),
            Operator::Div => VariableValue::Number(b / self),
            Operator::Mod => VariableValue::Number(b % self),
            Operator::Gt => VariableValue::Boolean(b > self),
            Operator::Gte => VariableValue::Boolean(b >= self),
            Operator::Lt => VariableValue::Boolean(b < self),
            Operator::Lte => VariableValue::Boolean(b <= self),
            Operator::And => VariableValue::Number(((b as u32) & (self as u32)) as f64),
            Operator::Or => VariableValue::Number(((b as u32) | (self as u32)) as f64),
            _ => unimplemented!(),
        }
    }
}

impl<'a> UnaryOp<'a> for f64 {
    fn unary_op(self, op: &Operator, _loc: Location<'a>) -> VariableValue<'a> {
        match op {
            Operator::Not => VariableValue::Number((!(self as u32)) as f64),
            _ => unimplemented!(),
        }
    }
}

impl<'a> BinaryOp<'a, String> for f64 {
    fn binary_op(self, b: String, op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        VariableValue::String(match op {
            Operator::Add => b + &self.to_string(),
            _ => err_mixed_types(loc),
        })
    }
}

impl<'a> BinaryOp<'a, Self> for bool {
    fn binary_op(self, b: Self, op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        VariableValue::Boolean(match op {
            Operator::Add | Operator::Or => b || self,
            Operator::Mul | Operator::And => b && self,
            Operator::Gt => b && !self,
            Operator::Gte => b || !self,
            Operator::Lt => !b && self,
            Operator::Lte => !b || self,
            _ => err_invalid_op(loc),
        })
    }
}

impl<'a> UnaryOp<'a> for bool {
    fn unary_op(self, op: &Operator, _loc: Location<'a>) -> VariableValue<'a> {
        VariableValue::Boolean(match op {
            Operator::Not => !self,
            _ => unimplemented!(),
        })
    }
}

impl<'a> BinaryOp<'a, String> for bool {
    fn binary_op(self, b: String, op: &Operator, loc: Location<'a>) -> VariableValue<'a> {
        VariableValue::String(match op {
            Operator::Add => b + &self.to_string(),
            _ => err_mixed_types(loc),
        })
    }
}

fn err_mixed_types<'a>(loc: Location<'a>) -> ! {
    CompError::new(
        201,
        String::from("Invalid mixed types in expression"),
        CompLocation::from(loc),
    )
    .print_and_exit();
}

fn err_invalid_op<'a>(loc: Location<'a>) -> ! {
    CompError::new(
        202,
        String::from("Invalid operator in expression"),
        CompLocation::from(loc),
    )
    .print_and_exit();
}
