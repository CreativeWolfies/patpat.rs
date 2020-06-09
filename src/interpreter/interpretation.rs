use super::*;
use std::cell::RefCell;
use std::collections::HashMap;

/** Casts or interpretes `value` into an instance of `into`.
    @param value - The value to cast
    @param into - The type to turn `value` into
    @param body - The (constructed and resolved) interpretation's body
**/
pub fn interprete_interpretation<'a>(
    value: VariableValue<'a>,
    (into, body): (RStructWeak<'a>, RASTRef<'a>),
) -> VariableValue<'a> {
    let mut init_ctx = Context::from(body.clone());
    init_ctx.variables.insert(String::from("from"), value);
    let res = VariableValue::Instance(
        into.upgrade().unwrap(),
        Rc::new(RefCell::new(HashMap::new())),
    );
    init_ctx.variables.insert(String::from("to"), res.clone());

    match body.borrow().instructions.last() {
        Some((RASTNode::Block(body), _)) => {
            interprete(body.clone(), vec![Rc::new(RefCell::new(init_ctx))])
        }
        _ => panic!("Expected interpretation body to end with a block"),
    };

    res
}

/** Casts `value` into an instance of `into`.
    This asserts that `value` is a subtype of `into` and does not use any programmer-defined interpretation logic.
    @param value - The value to cast
    @param into - THe type to turn `value` into
**/
pub fn cast_value<'a>(value: VariableValue<'a>, into: RStructRef<'a>) -> VariableValue<'a> {
    if let VariableValue::Instance(_of, hashmap) = value {
        VariableValue::Instance(into, hashmap)
    } else {
        panic!("Expected value to be an instance!");
    }
}
