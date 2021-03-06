pub use super::ast::resolve::*;
use std::cell::RefCell;
use std::ops::Deref;

pub mod callable;
pub mod context;
pub mod expr;
pub mod interpretation;
pub mod value;
pub mod composite_fn;

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
            // println!("=> {:?}", last_value);
            res.push(last_value);
        }

        VariableValue::Tuple(res)
    } else {
        for instruction in &ast.borrow().instructions {
            last_value = interprete_instruction(&instruction.0, instruction.1.clone(), &contexes);
            contexes.last().unwrap().borrow_mut().last_value = last_value.clone();
            // println!("-> {:?}", last_value);
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
        RASTNode::Tuple(instructions, _is_partial) => VariableValue::Tuple(
            instructions
                .iter()
                .map(|ins| interprete_instruction(&ins.0, ins.1.clone(), contexes))
                .collect(),
        ),
        RASTNode::VariableDef(var, value) => {
            let res = with_variable(var, contexes, |var| var.clone(), location.clone());
            let value = interprete_instruction(value.deref(), location.clone(), contexes);
            with_variable(
                var,
                contexes,
                |var| {
                    *var = value;
                    VariableValue::Nil
                },
                location.clone(),
            );
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
                vec![],
            )
        }
        RASTNode::Expression(expr) => interprete_expression(expr, location, contexes),
        RASTNode::Block(ast) => interprete(ast.clone(), contexes.clone()),
        RASTNode::Variable(var) => with_variable(var, contexes, |var| var.clone(), location),
        RASTNode::Nil => VariableValue::Nil,
        RASTNode::VoidSymbol => VariableValue::Nil,
        RASTNode::Pattern(pat) => VariableValue::Function(pat.clone(), vec![]),
        RASTNode::Function(fun) => VariableValue::Function(
            fun.clone(),
            fun.borrow()
                .closure
                .iter()
                .map(|(name, value)| (name.clone(), interprete(value.clone(), contexes.clone())))
                .collect(),
        ),
        RASTNode::TypeName(x) => VariableValue::Type(x.clone()),
        RASTNode::ComplexDef(expr, member, value) => {
            if let DefineMember::Member(name) = member {
                if let VariableValue::Instance(_t, vars) =
                    interprete_expression(expr, location.clone(), contexes)
                {
                    // TODO: check that `name` is part of _t
                    let res = vars
                        .borrow()
                        .get(name)
                        .map(|x| x.clone())
                        .unwrap_or(VariableValue::Nil);
                    vars.borrow_mut().insert(
                        name.clone(),
                        interprete_instruction(value, location, contexes),
                    );
                    res
                } else {
                    panic!("Trying to set value on non-object");
                }
            } else {
                unimplemented!("DefineMember::*");
            }
        }
        _ => VariableValue::Nil,
    }
}

pub fn with_variable<'a, F>(
    variable: &RSymRef,
    contexes: &Vec<ContextRef<'a>>,
    func: F,
    location: Location<'a>,
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
                    CompError::new(
                        1,
                        format!("Variable {} not found at depth {}!", variable.name, depth),
                        location.into(),
                    )
                    .print_and_exit();
                });
        }
    }
    CompError::new(
        1,
        format!("Couldn't find context at depth {}!", depth),
        location.into(),
    )
    .print_and_exit();
}

pub fn is_truthy(value: &VariableValue) -> bool {
    match value {
        VariableValue::Number(x) => *x != 0.0,
        VariableValue::Boolean(x) => *x,
        VariableValue::String(x) => x.len() > 0,
        VariableValue::Nil => false,
        _ => true,
    }
}
