// Internal patterns and constants

pub mod pattern;

pub use super::*;
pub use crate::interpreter::*;
use std::cell::RefCell;
use std::rc::Weak;

pub use pattern::*;

thread_local!(pub static TEST_LOG: RefCell<String> = RefCell::new(String::new()));

pub fn std_rast<'a>() -> RAST<'a> {
    let mut res = RAST::new(Weak::new(), ASTKind::Block);

    add_pattern(&mut res, "#println", |args, _, _| {
        println!(
            "{}",
            args.iter()
                .map(|arg| format!("{}", arg))
                .collect::<Vec<String>>()
                .join(", ")
        );
        VariableValue::Nil
    });

    add_pattern(&mut res, "#test_log", |args, _, _| {
        TEST_LOG.with(|s| {
            s.borrow_mut().push_str(
                &args
                    .iter()
                    .map(|arg| format!("{:?}", arg))
                    .collect::<Vec<String>>()
                    .join(", ")[..],
            );
            s.borrow_mut().push_str("\n");
        });
        VariableValue::Nil
    });

    add_pattern(&mut res, "#if", |args, loc, contexes| {
        if args.len() < 2 {
            return VariableValue::Nil;
        }
        let mut iter = args.into_iter();
        if match iter.next().unwrap() {
            VariableValue::Number(x) => x != 0.0,
            VariableValue::Boolean(x) => x,
            VariableValue::String(x) => x.len() > 0,
            VariableValue::Nil => false,
            _ => true,
        } {
            match iter.next().unwrap() {
                VariableValue::Function(fun) => {
                    // TODO: give Callable a n_args() method
                    fun.call(vec![], loc, contexes)
                }
                x => x,
            }
        } else {
            VariableValue::Bail
        }
    });

    add_pattern(&mut res, "#else", |args, loc, contexes| {
        let last_value = contexes.last().unwrap().borrow().last_value.clone();
        if args.len() < 1 {
            return VariableValue::Nil;
        }
        match last_value {
            VariableValue::Bail => match args.into_iter().next().unwrap() {
                VariableValue::Function(fun) => fun.call(vec![], loc, contexes),
                x => x,
            },
            x => x,
        }
    });

    add_pattern(&mut res, "#bail", |_, _, _| VariableValue::Bail);

    res
}

fn add_pattern<'a, F: 'static>(rast: &mut RAST<'a>, name: &str, fun: F)
where
    F: Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>,
{
    rast.patterns
        .push(Rc::new(IntPattern::new(name.to_string(), fun)));
}
