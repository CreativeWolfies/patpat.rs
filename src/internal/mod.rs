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
        if is_truthy(iter.next().unwrap()) {
            match iter.next().unwrap() {
                VariableValue::Function(fun, closure) => {
                    // TODO: give Callable a n_args() method
                    fun.call(vec![], loc, contexes, closure)
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
                VariableValue::Function(fun, closure) => fun.call(vec![], loc, contexes, closure),
                x => x,
            },
            x => x,
        }
    });

    add_pattern(&mut res, "#elseif", |args, loc, contexes| {
        let last_value = contexes.last().unwrap().borrow().last_value.clone();
        if args.len() < 2 {
            return VariableValue::Nil;
        }
        let mut iter = args.into_iter();

        match last_value {
            VariableValue::Bail => {
                if is_truthy(iter.next().unwrap()) { // 1st argument: condition
                    match iter.next().unwrap() { // 2nd argument
                        VariableValue::Function(fun, closure) => fun.call(vec![], loc, contexes, closure),
                        x => x,
                    }
                } else {
                    last_value
                }
            },
            x => x,
        }
    });

    add_pattern(&mut res, "#bail", |_, _, _| VariableValue::Bail);

    add_pattern(&mut res, "#for", |args, loc, contexes| {
        if args.len() < 3 {
            // TODO: error out
            return VariableValue::Nil;
        }
        let mut iter = args.into_iter();
        let from_raw = iter.next().unwrap();
        let to_raw = iter.next().unwrap();
        let callback_raw = iter.next().unwrap();
        if let VariableValue::Function(callback, closure) = callback_raw {
            if let (VariableValue::Number(from), VariableValue::Number(to)) = (from_raw, to_raw) {
                let mut last_value = VariableValue::Nil;
                for x in (from as usize)..((to + 1f64) as usize) {
                    match callback.call(vec![VariableValue::Number(x as f64)], loc.clone(), contexes, closure.clone()) {
                        VariableValue::Bail => {
                            return VariableValue::Bail;
                        },
                        x => last_value = x,
                    }
                }
                return last_value;
            }
        }
        return VariableValue::Nil;
    });

    add_pattern(&mut res, "#loop", |args, loc, contexes| {
        if args.len() < 1 {
            // TODO: error out
            return VariableValue::Nil;
        }
        let callback_raw = args.into_iter().next().unwrap();
        if let VariableValue::Function(callback, closure) = callback_raw {
            loop {
                match callback.call(vec![], loc.clone(), contexes, closure.clone()) {
                    VariableValue::Bail => {
                        return VariableValue::Bail;
                    },
                    _ => {}
                }
            }
        }
        VariableValue::Nil
    });

    res
}

fn add_pattern<'a, F: 'static>(rast: &mut RAST<'a>, name: &str, fun: F)
where
    F: Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a>,
{
    rast.patterns
        .push(Rc::new(IntPattern::new(name.to_string(), fun)));
}

fn is_truthy(value: VariableValue) -> bool {
    match value {
        VariableValue::Number(x) => x != 0.0,
        VariableValue::Boolean(x) => x,
        VariableValue::String(x) => x.len() > 0,
        VariableValue::Nil => false,
        _ => true,
    }
}
