use super::*;
use token::Operator;
use std::collections::HashMap;

pub trait BinaryOp<'a, T> {
    fn binary_op(self: Self, b: T, op: &Operator, loc: Location<'a>) -> VariableValue<'a>;
}
pub trait UnaryOp<'a> {
    fn unary_op(self: Self, op: &Operator, loc: Location<'a>) -> VariableValue<'a>;
}

#[derive(Debug)]
pub enum ExprValue<'a> {
    Value(VariableValue<'a>),
    Member(String),
    MethodCall(String, RASTRef<'a>),
    Access(RefCell<HashMap<String, VariableValue<'a>>>, String),
}

/// Executes the expression and returns the remaining ExprValue stack
pub fn interprete_expression_int<'a>(
    expr: &RExpression<'a>,
    location: Location<'a>,
    contexes: &Vec<ContextRef<'a>>,
) -> Vec<ExprValue<'a>> {
    let mut stack: Vec<ExprValue<'a>> = Vec::with_capacity(expr.max_depth);
    for term in &expr.terms {
        match term {
            RExprTerm::Push(node) => stack.push(match node {
                RASTNode::MethodCall(name, body) => ExprValue::MethodCall(name.clone(), body.clone()),
                RASTNode::Member(name) => ExprValue::Member(name.clone()),
                x => ExprValue::Value(interprete_instruction(
                    x,
                    location.clone(),
                    contexes,
                )
            )}),
            RExprTerm::Op(op) => match op {
                Operator::Interpretation => unimplemented!(),
                Operator::MemberAccessor => {
                    let right = stack.pop().unwrap();
                    let left = resolve_access(stack.pop().unwrap());
                    match left {
                        ExprValue::Value(VariableValue::Function(fun)) => {
                            let args = match right {
                                ExprValue::Value(VariableValue::Tuple(vec)) => vec,
                                ExprValue::Value(x) => vec![x],
                                _ => unimplemented!()
                            };
                            stack.push(ExprValue::Value(fun.call(
                                args,
                                location.clone(),
                                contexes,
                            )));
                        },
                        ExprValue::Value(VariableValue::Type(t)) => {
                            match right {
                                ExprValue::Member(_name) => unimplemented!(),
                                ExprValue::MethodCall(name, args) => {
                                    if let Some(fun) = t.borrow().get_method(name) {
                                        stack.push(ExprValue::Value(
                                            fun.call_member(
                                                match interprete(args, contexes.clone()) {
                                                    VariableValue::Tuple(list) => list,
                                                    x => vec![x],
                                                },
                                                location.clone(),
                                                contexes,
                                                Some(VariableValue::Type(t.clone())),
                                            )
                                        ));
                                    } else {
                                        unimplemented!(); // TODO: error
                                    }
                                },
                                _ => unimplemented!()
                            }
                        },
                        ExprValue::Value(VariableValue::Instance(t, vars)) => {
                            match right {
                                ExprValue::Member(name) => stack.push(ExprValue::Access(vars, name)),
                                ExprValue::MethodCall(name, args) => {
                                    if let Some(fun) = t.borrow().get_method(name) {
                                        stack.push(ExprValue::Value(
                                            fun.call_member(
                                                match interprete(args, contexes.clone()) {
                                                    VariableValue::Tuple(list) => list,
                                                    x => vec![x],
                                                },
                                                location.clone(),
                                                contexes,
                                                Some(VariableValue::Instance(t.clone(), vars.clone())),
                                            )
                                        ));
                                    } else {
                                        unimplemented!(); // TODO: error
                                    }
                                },
                                _ => unimplemented!(),
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
                Operator::Not => {
                    let res = execute_unary_op(stack.pop().unwrap(), &op, location.clone());
                    stack.push(res);
                }
                _ => {
                    let res = execute_bin_op(
                        stack.pop().unwrap(),
                        stack.pop().unwrap(),
                        &op,
                        location.clone(),
                    );
                    stack.push(res);
                }
            },
        }
    }

    stack
}

pub fn interprete_expression<'a>(
    expr: &RExpression<'a>,
    location: Location<'a>,
    contexes: &Vec<ContextRef<'a>>,
) -> VariableValue<'a> {
    match interprete_expression_int(expr, location, contexes).pop().map(|x| resolve_access(x)) {
        Some(ExprValue::Value(val)) => val,
        _ => VariableValue::Nil,
    }
}

#[allow(unreachable_patterns)]
pub fn execute_bin_op<'a>(
    a: ExprValue<'a>,
    b: ExprValue<'a>,
    op: &Operator,
    location: Location<'a>,
) -> ExprValue<'a> {
    match resolve_access(a) {
        ExprValue::Value(a_val) => match b {
            ExprValue::Value(b_val) => ExprValue::Value(a_val.binary_op(b_val, op, location)),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

#[allow(unreachable_patterns)]
pub fn execute_unary_op<'a>(
    a: ExprValue<'a>,
    op: &Operator,
    location: Location<'a>,
) -> ExprValue<'a> {
    match a {
        ExprValue::Value(a_val) => ExprValue::Value(a_val.unary_op(op, location)),
        _ => unimplemented!(),
    }
}

fn resolve_access<'a>(from: ExprValue<'a>) -> ExprValue<'a> {
    match from {
        ExprValue::Access(vars, name) => ExprValue::Value(vars.borrow().get(&name).map(|x| x.clone()).unwrap_or(VariableValue::Nil)),
        x => x,
    }
}
