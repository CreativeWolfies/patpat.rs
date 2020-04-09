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
}

pub fn interprete_expression<'a>(
    expr: &RExpression<'a>,
    location: Location<'a>,
    contexes: &Vec<ContextRef<'a>>,
) -> VariableValue<'a> {
    let mut stack: Vec<ExprValue<'a>> = Vec::with_capacity(expr.max_depth);
    for term in &expr.terms {
        match term {
            RExprTerm::Push(node) => stack.push(ExprValue::Value(interprete_instruction(
                node,
                location.clone(),
                contexes,
            ))),
            RExprTerm::Op(op) => match op {
                Operator::Interpretation | Operator::MemberAccessor => unimplemented!(),
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
    match stack.pop() {
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
