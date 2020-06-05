use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

pub trait Callable<'a> {
    fn get_name(&self) -> String;

    fn call_member(
        &self,
        args: Vec<VariableValue<'a>>,
        location: Location<'a>,
        contexes: &Vec<ContextRef<'a>>,
        closure: Vec<(String, VariableValue<'a>)>,
        parent: Option<VariableValue<'a>>,
    ) -> VariableValue<'a>;

    fn get_args_n(&self) -> Option<usize> {
        None
    }

    fn call(
        &self,
        args: Vec<VariableValue<'a>>,
        location: Location<'a>,
        contexes: &Vec<ContextRef<'a>>,
        closure: Vec<(String, VariableValue<'a>)>,
    ) -> VariableValue<'a> {
        self.call_member(args, location, contexes, closure, None)
    }
}

impl<'a> fmt::Debug for dyn Callable<'a> + 'a {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Callable({})", self.get_name())
    }
}

impl<'a> Callable<'a> for RPattern<'a> {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn call_member(
        &self,
        args: Vec<VariableValue<'a>>,
        location: Location<'a>,
        contexes: &Vec<ContextRef<'a>>,
        closure: Vec<(String, VariableValue<'a>)>,
        parent: Option<VariableValue<'a>>,
    ) -> VariableValue<'a> {
        self.function
            .borrow()
            .as_ref()
            .unwrap()
            .call_member(args, location, contexes, closure, parent)
    }
}

impl<'a> Callable<'a> for RFunction<'a> {
    fn get_name(&self) -> String {
        "<anonymous function>".to_string()
    }

    #[allow(unused_variables)]
    fn call_member(
        &self,
        args: Vec<VariableValue<'a>>,
        location: Location<'a>,
        contexes: &Vec<ContextRef<'a>>,
        closure: Vec<(String, VariableValue<'a>)>,
        parent: Option<VariableValue<'a>>,
    ) -> VariableValue<'a> {
        //! Asserts that contexes is not empty
        if let Some((depth, ulid, ref_loc)) = self.required_ctx.clone() {
            let mut ctx_found = false;
            for ctx in contexes.iter() {
                if ctx.borrow().depth == depth && ctx.borrow().ulid == ulid {
                    ctx_found = true;
                }
            }
            if !ctx_found {
                CompError::new(
                    206,
                    format!("Function fell out of scope"),
                    location.into()
                ).append(
                    format!("This is due to the following reference being made within it:"),
                    ref_loc.into()
                ).print_and_exit();
            }
        }

        let mut init_ctx = Context::from(self.body.clone());

        if args.len() != self.args.len() {
            CompError::new(
                203,
                format!(
                    "Mismatching number of arguments: expected {}, got {}.",
                    self.args.len(),
                    args.len()
                ),
                CompLocation::from(location),
            )
            .print_and_exit();
        }

        for (from, to) in args.into_iter().zip(self.args.iter()) {
            // TODO: conversion
            init_ctx.variables.insert(to.name.clone(), from);
        }

        for (name, value) in closure.into_iter() {
            init_ctx.variables.insert(name, value);
        }

        if self.has_lhs {
            init_ctx.variables.insert(
                "lhs".to_string(),
                contexes.last().unwrap().borrow().last_value.clone(),
            );
        }
        if self.has_new {
            if let Some(VariableValue::Type(type_raw)) = parent {
                let obj: InstanceRef<'_> = Rc::new(RefCell::new(HashMap::new()));
                init_ctx.variables.insert(
                    "self".to_string(),
                    VariableValue::Instance(type_raw.clone(), obj.clone()),
                );

                let mut contexes = contexes.clone();
                contexes.push(Rc::new(RefCell::new(init_ctx)));

                match self.body.borrow().instructions.last().unwrap() {
                    (RASTNode::Block(body), _) => interprete(body.clone(), contexes),
                    _ => panic!("Expected function body node to be a block"),
                };

                VariableValue::Instance(type_raw, obj)
            } else {
                unimplemented!("Non-struct #new() is not supported yet!");
            }
        } else {
            // TODO: has_self

            if self.has_self {
                init_ctx
                    .variables
                    .insert("self".to_string(), parent.unwrap());
            }

            let mut contexes = contexes.clone();
            contexes.push(Rc::new(RefCell::new(init_ctx)));

            match self.body.borrow().instructions.last().unwrap() {
                (RASTNode::Block(body), _) => interprete(body.clone(), contexes),
                _ => panic!("Expected function body node to be a block"),
            }
        }
    }
}

impl<'a> Callable<'a> for RefCell<RFunction<'a>> {
    fn get_name(&self) -> String {
        "<anonymous function>".to_string()
    }

    fn call_member(
        &self,
        args: Vec<VariableValue<'a>>,
        location: Location<'a>,
        contexes: &Vec<ContextRef<'a>>,
        closure: Vec<(String, VariableValue<'a>)>,
        parent: Option<VariableValue<'a>>,
    ) -> VariableValue<'a> {
        self.borrow()
            .call_member(args, location, contexes, closure, parent)
    }
}
