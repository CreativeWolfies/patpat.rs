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

/** Executes the expression and returns the remaining ExprValue stack
    @param expr - The resolved expression
    @param location - The expression's location
    @param contexes - The local contexes stack

    Consider the following expression: (a + b) - 2
    The expression constructer and the resolver should output something similar to:

    {
        terms: [
            Push(Variable(a)),
            Push(Variable(b)),
            Op(Add),
            Push(Number(2)),
            Op(Sub)
        ]
    }

    This will allocate a 2-long value stack (the length being calculated earlier).
    It will sequentially push and pop elements of this stack in order to execute the expression:

    - Push(a) -> [a]
    - Push(b) -> [a, b]
    - Op(Add) -> [(a+b)]
    - Push(2) -> [(a+b), 2]
    - Op(Sub) -> [(a+b-2)]

    In this case, this function will return [(a+b-2)]. The return array should have a length of one if the terms list was generated by the constructer.
**/
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
                            CompError::new(
                                1,
                                "Casting non-struct-instances to other objects is not yet supported!".to_string(),
                                CompLocation::from(location)
                            ).print_and_exit();
                        }
                    } else {
                        CompError::new(
                            204,
                            "Trying to cast to a non-type".to_string(),
                            CompLocation::from(location)
                        ).print_and_exit();
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
                                _ => CompError::new(
                                    1,
                                    "Accessing a member of a function is not yet supported".to_string(),
                                    CompLocation::from(location)
                                ).print_and_exit(),
                            };
                            stack.push(ExprValue::Value(fun.call(
                                args,
                                location.clone(),
                                contexes,
                            )));
                        }
                        ExprValue::Value(VariableValue::Type(t)) => {
                            match right {
                                ExprValue::Member(_name) => CompError::new(
                                        1,
                                        "Accessing a member of a type is not yet supported".to_string(),
                                        CompLocation::from(location)
                                    ).print_and_exit(),
                                ExprValue::MethodCall(name, args) => {
                                    if let Some(fun) = t.borrow().get_method(name.clone()) {
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
                                        CompError::new(
                                            152,
                                            format!("Couldn't find method {} in object", name),
                                            CompLocation::from(location)
                                        ).print_and_exit()
                                    }
                                }
                                _ => CompError::new(
                                    1,
                                    format!("Complex accessors are not yet supported!"),
                                    CompLocation::from(location)
                                ).print_and_exit(),
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
                                        CompError::new(
                                            152,
                                            format!("Cannot find method {} in object of type {}.", name, t.borrow().name.clone()),
                                            CompLocation::from(location)
                                        ).print_and_exit();
                                    }
                                }
                                _ => CompError::new(
                                    1,
                                    format!("Complex accessors are not yet supported!"),
                                    CompLocation::from(location)
                                ).print_and_exit(),
                            }
                        }
                        _ => CompError::new(
                            1,
                            format!("Accessing members of this data type is not yet supported!"),
                            CompLocation::from(location)
                        ).print_and_exit(),
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

/**
    Calls interprete_expression_int and returns the last element of its remaining stack.
**/
pub fn interprete_expression<'a>(
    expr: &RExpression<'a>,
    location: Location<'a>,
    contexes: &Vec<ContextRef<'a>>,
) -> VariableValue<'a> {
    match interprete_expression_int(expr, location, contexes).pop() {
        Some(ExprValue::Value(val)) => val,
        _ => panic!("interprete_expression_int(...) returned an empty array"),
    }
}

// TODO: move these back to interprete_expr_int?

/** Executes the binary operator `op` on `a` and `b` **/
pub fn execute_bin_op<'a>(
    a: ExprValue<'a>,
    b: ExprValue<'a>,
    op: &Operator,
    location: Location<'a>,
) -> ExprValue<'a> {
    match a {
        ExprValue::Value(a_val) => match b {
            ExprValue::Value(b_val) => ExprValue::Value(a_val.binary_op(b_val, op, location)),
            _ => panic!("Expected `b` to be ExprValue::Value in execute_bin_op"),
        },
        _ => panic!("Expected `a` to be ExprValue::Value in execute_bin_op"),
    }
}

/** Executes the unary operator `op` on `a` **/
pub fn execute_unary_op<'a>(
    a: ExprValue<'a>,
    op: &Operator,
    location: Location<'a>,
) -> ExprValue<'a> {
    match a {
        ExprValue::Value(a_val) => ExprValue::Value(a_val.unary_op(op, location)),
        _ => panic!("Expected `a` to be ExprValue::Value in execute_unary_op"),
    }
}

/** Looks up `name` in `obj` as part of a member access expression **/
fn resolve_access<'a>(obj: InstanceRef<'a>, name: String) -> VariableValue<'a> {
    obj.borrow()
        .get(&name)
        .map(|x| x.clone())
        .unwrap_or(VariableValue::Nil)
}
