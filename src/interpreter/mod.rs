pub use super::ast::resolve::*;
use std::cell::RefCell;
use std::ops::Deref;

pub mod callable;
pub mod context;
pub mod expr;
pub mod value;

pub use callable::*;
pub use context::*;
pub use expr::*;
pub use value::*;

pub fn interprete<'a>(ast: RASTRef<'a>, contexes: Vec<ContextRef<'a>>) -> VariableValue<'a> {
    //! Interpretes an `RAST` block
    let mut contexes = contexes.clone();
    contexes.push(Rc::new(RefCell::new(Context::from(ast.clone()))));
    let mut last_value: VariableValue = VariableValue::Nil;

    if let ASTKind::Tuple | ASTKind::ArgTuple = ast.borrow().kind {
        let mut res: Vec<VariableValue<'a>> = Vec::new();
        for instruction in &ast.borrow().instructions {
            last_value = interprete_instruction(&instruction.0, instruction.1.clone(), &contexes);
            contexes.last().unwrap().borrow_mut().last_value = last_value.clone();
            println!("=> {:?}", last_value);
            res.push(last_value);
        }

        VariableValue::Tuple(res)
    } else {
        for instruction in &ast.borrow().instructions {
            last_value = interprete_instruction(&instruction.0, instruction.1.clone(), &contexes);
            contexes.last().unwrap().borrow_mut().last_value = last_value.clone();
            println!("-> {:?}", last_value);
        }

        last_value
    }
}

pub fn interprete_instruction<'a, 'b>(
    instruction: &'b RASTNode<'a>,
    location: Location<'a>,
    contexes: &'b Vec<ContextRef<'a>>,
) -> VariableValue<'a> {
    //! Interpretes a single `RASTNode` instruction
    match &instruction {
        RASTNode::Number(x) => VariableValue::Number(*x),
        RASTNode::String(x) => VariableValue::String(x.clone()),
        RASTNode::Boolean(x) => VariableValue::Boolean(*x),
        RASTNode::Tuple(instructions) => VariableValue::Tuple(
            instructions
                .iter()
                .map(|ins| interprete_instruction(&ins.0, ins.1.clone(), contexes))
                .collect(),
        ),
        RASTNode::VariableDef(var, value) => {
            let res = with_variable(var, contexes, |var| var.clone());
            let value = interprete_instruction(value.deref(), location.clone(), contexes);
            with_variable(var, contexes, |var| {
                *var = value;
                VariableValue::Nil
            });
            res
        }
        RASTNode::PatternCall(pat, args) => {
            let args = interprete(args.clone(), contexes.clone());
            pat.call(
                match args {
                    VariableValue::Tuple(list) => list,
                    x => vec![x],
                },
                location.clone(),
                contexes,
            )
        }
        RASTNode::Expression(expr) => interprete_expression(expr, location, contexes),
        RASTNode::Block(ast) => interprete(ast.clone(), contexes.clone()),
        RASTNode::Variable(var) => with_variable(var, contexes, |var| var.clone()),
        RASTNode::Nil => VariableValue::Nil,
        _ => VariableValue::Nil,
    }
}

pub fn with_variable<'a, F>(
    variable: &RSymRef,
    contexes: &Vec<ContextRef<'a>>,
    func: F,
) -> VariableValue<'a>
where
    F: FnOnce(&mut VariableValue<'a>) -> VariableValue<'a>,
{
    /*! Lets you manipulate the value to which `variable` points to in the context stack (`contexes`) with `func`.
    The context in which this variable is will be borrowed mutably.
    `func` should thus not have any side-effect and must not try to access the context stack.
    */
    let depth = variable.depth;
    for ctx in contexes.iter().rev() {
        if ctx.borrow().depth == depth {
            return ctx
                .borrow_mut()
                .variables
                .get_mut(&variable.name)
                .map(|mut x| func(&mut x))
                .unwrap_or_else(|| {
                    panic!("Variable {} not found at depth {}!", variable.name, depth)
                });
        }
    }
    panic!("Couldn't find context at depth {}!", depth);
}
