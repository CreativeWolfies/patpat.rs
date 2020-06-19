use super::*;
use token::Operator;
use std::rc::Rc;

pub struct CompositeFunction<'a> {
    pub left: (Rc<dyn Callable<'a> + 'a>, Vec<(String, VariableValue<'a>)>),
    pub right: (Rc<dyn Callable<'a> + 'a>, Vec<(String, VariableValue<'a>)>),
    pub op: Operator,
}

impl<'a> CompositeFunction<'a> {
    pub fn new(
        left: Rc<dyn Callable<'a> + 'a>,
        left_closure: Vec<(String, VariableValue<'a>)>,
        right: Rc<dyn Callable<'a> + 'a>,
        right_closure: Vec<(String, VariableValue<'a>)>,
        op: Operator,
    ) -> Rc<CompositeFunction<'a>> {
        Rc::new(CompositeFunction {
            left: (left, left_closure),
            right: (right, right_closure),
            op
        })
    }
}

impl<'a> Callable<'a> for CompositeFunction<'a> {
    fn get_name(&self) -> String {
        format!("[{} {} {}]", self.left.0.get_name(), self.op, self.right.0.get_name())
    }

    fn call_member(
        &self,
        args: Vec<VariableValue<'a>>,
        location: Location<'a>,
        contexes: &Vec<ContextRef<'a>>,
        _closure: Vec<(String, VariableValue<'a>)>,
        parent: Option<VariableValue<'a>>,
    ) -> VariableValue<'a> {
        match self.op {
            Operator::And => {
                let left = self.left.0.call_member(args.clone(), location.clone(), contexes, self.left.1.clone(), parent.clone());
                if is_truthy(&left) {
                    let right = self.right.0.call_member(args, location.clone(), contexes, self.right.1.clone(), parent);
                    left.binary_op(right, &self.op, location)
                } else {
                    left
                }
            }
            Operator::Or => {
                let left = self.left.0.call_member(args.clone(), location.clone(), contexes, self.left.1.clone(), parent.clone());
                if !is_truthy(&left) {
                    let right = self.right.0.call_member(args, location.clone(), contexes, self.right.1.clone(), parent);
                    left.binary_op(right, &self.op, location)
                } else {
                    left
                }
            }
            _ => {
                let left = self.left.0.call_member(args.clone(), location.clone(), contexes, self.left.1.clone(), parent.clone());
                let right = self.right.0.call_member(args, location.clone(), contexes, self.right.1.clone(), parent);
                left.binary_op(right, &self.op, location)
            }
        }
    }
}
