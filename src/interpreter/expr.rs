use super::*;
use token::Operator;

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
                RASTNode::MethodCall(name, body) => {
                    ExprValue::MethodCall(name.clone(), body.clone())
                }
                RASTNode::Member(name) => ExprValue::Member(name.clone()),
                x => ExprValue::Value(interprete_instruction(x, location.clone(), contexes)),
            }),
            RExprTerm::Op(op) => match op {
                Operator::Interpretation => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    if let ExprValue::Value(VariableValue::Type(into)) = right {
                        if let ExprValue::Value(VariableValue::Instance(of, values)) = left {
                            let interpretation = of
                                .borrow()
                                .interpretations
                                .iter()
                                .find(|x| x.0.upgrade().map(|y| *y == *into).unwrap_or(false))
                                .unwrap_or_else(|| panic!("Couldn't find interpretation"))
                                .clone();

                            stack.push(ExprValue::Value(
                                interpretation::interprete_interpretation(
                                    VariableValue::Instance(of.clone(), values),
                                    interpretation.clone(),
                                ),
                            ));
                        } else {
                            unimplemented!("Casting non-struct-instances to other objects is not yet supported");
                        }
                    } else {
                        unimplemented!();
                    }
                }
                Operator::MemberAccessor => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    match left {
                        ExprValue::Value(VariableValue::Function(fun)) => {
                            let args = match right {
                                ExprValue::Value(VariableValue::Tuple(vec)) => vec,
                                ExprValue::Value(x) => vec![x],
                                _ => unimplemented!(),
                            };
                            stack.push(ExprValue::Value(fun.call(
                                args,
                                location.clone(),
                                contexes,
                            )));
                        }
                        ExprValue::Value(VariableValue::Type(t)) => {
                            match right {
                                ExprValue::Member(_name) => unimplemented!(),
                                ExprValue::MethodCall(name, args) => {
                                    if let Some(fun) = t.borrow().get_method(name) {
                                        stack.push(ExprValue::Value(fun.call_member(
                                            match interprete(args, contexes.clone()) {
                                                VariableValue::Tuple(list) => list,
                                                x => vec![x],
                                            },
                                            location.clone(),
                                            contexes,
                                            Some(VariableValue::Type(t.clone())),
                                        )));
                                    } else {
                                        unimplemented!(); // TODO: error
                                    }
                                }
                                _ => unimplemented!(),
                            }
                        }
                        ExprValue::Value(VariableValue::Instance(t, vars)) => {
                            match right {
                                ExprValue::Member(name) => {
                                    stack.push(ExprValue::Value(resolve_access(vars, name)))
                                }
                                ExprValue::MethodCall(name, args) => {
                                    if let Some(fun) = t.borrow().get_method(name.clone()) {
                                        stack.push(ExprValue::Value(fun.call_member(
                                            match interprete(args, contexes.clone()) {
                                                VariableValue::Tuple(list) => list,
                                                x => vec![x],
                                            },
                                            location.clone(),
                                            contexes,
                                            Some(VariableValue::Instance(t.clone(), vars.clone())),
                                        )));
                                    } else {
                                        unimplemented!(
                                            "Cannot find method {} in {}",
                                            name,
                                            t.borrow().name.clone()
                                        ); // TODO: error
                                    }
                                }
                                _ => unimplemented!(),
                            }
                        }
                        _ => unimplemented!("Cannot access member of {:?}", left),
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
    match interprete_expression_int(expr, location, contexes).pop() {
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
    match a {
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

fn resolve_access<'a>(obj: InstanceRef<'a>, name: String) -> VariableValue<'a> {
    obj.borrow()
        .get(&name)
        .map(|x| x.clone())
        .unwrap_or(VariableValue::Nil)
}
