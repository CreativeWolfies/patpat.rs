// Internal patterns and constants

pub mod pattern;

pub use super::*;
pub use crate::interpreter::*;
use std::rc::Weak;
use std::cell::RefCell;

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
                &args.iter()
                    .map(|arg| format!("{:?}", arg))
                    .collect::<Vec<String>>()
                    .join(", ")[..]
            );
            s.borrow_mut().push_str("\n");
        });
        VariableValue::Nil
    });

    res
}

fn add_pattern<'a, F: 'static>(rast: &mut RAST<'a>, name: &str, fun: F)
where F: Fn(Vec<VariableValue<'a>>, Location<'a>, &Vec<ContextRef<'a>>) -> VariableValue<'a> {
    rast.patterns.push(Rc::new(IntPattern::new(
        name.to_string(),
        Box::new(fun)
    )));
}
