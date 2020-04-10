// Internal patterns and constants

pub mod pattern;

pub use super::*;
pub use crate::interpreter::*;
use std::rc::Weak;

pub use pattern::*;

pub fn std_rast<'a>() -> RAST<'a> {
    let mut res = RAST::new(Weak::new(), ASTKind::Block);

    res.patterns.push(Rc::new(IntPattern::new(
        "#println".to_string(),
        Box::new(|args, _, _| {
            println!(
                "{}",
                args.iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            VariableValue::Nil
        }),
    )));

    res
}
