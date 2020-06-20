use super::*;
use std::fmt;
use token::TypeStrictness;

#[derive(Clone)]
pub struct RFunction<'a> {
    pub args: Vec<RFunctionArg<'a>>,
    pub body: Rc<RefCell<RAST<'a>>>,
    pub has_self: bool,
    pub has_lhs: bool,
    pub has_new: bool,
    pub closure: Vec<(String, RASTRef<'a>)>,
    pub required_ctx: Option<(usize, u128, Location<'a>)>,
}

impl<'a> From<(Function<'a>, RASTWeak<'a>, Location<'a>)> for RFunction<'a> {
    /**
      Creates an RFunction off a Function and a parent RAST.
    */
    fn from(input: (Function<'a>, RASTWeak<'a>, Location<'a>)) -> RFunction<'a> {
        let function = input.0;
        let parent = input.1;
        let loc = input.2;

        // TODO: prevent access to variables outside of functions
        // TODO: prevent #with inside of patterns
        let init = Rc::new(RefCell::new(RAST::new(parent.clone(), ASTKind::Block)));
        let mut closure = Vec::<(String, RASTRef<'a>)>::with_capacity(function.closure.len());

        for arg in function.args.iter() {
            init.borrow_mut()
                .variables
                .push(Rc::new(RefCell::new(RSymbol::new(arg.name.clone()))));
        }

        for (name, value) in function.closure.into_iter() {
            init.borrow_mut()
                .variables
                .push(Rc::new(RefCell::new(RSymbol::new(name.clone()))));
            closure.push((name, RAST::resolve(value, parent.clone())));
        }

        if function.has_lhs {
            init.borrow_mut()
                .variables
                .push(Rc::new(RefCell::new(RSymbol::new(String::from("lhs")))));
        }
        if function.has_self || function.has_new {
            init.borrow_mut()
                .variables
                .push(Rc::new(RefCell::new(RSymbol::new(String::from("self")))));
        }

        let body = RAST::resolve(function.body, Rc::downgrade(&init));

        let mut required_ctx =
            scan_body_reqs(body.clone(), &function.refs, init.borrow().depth, &loc);
        if let Some((depth, _, _)) = required_ctx {
            if depth >= init.borrow().depth {
                required_ctx = None;
            }
        }

        init.borrow_mut()
            .instructions
            .push((RASTNode::Block(body), loc.clone()));

        RFunction {
            args: function
                .args
                .into_iter()
                .map(|arg| RFunctionArg::from((arg, parent.clone(), loc.clone())))
                .collect(),
            body: init,
            has_lhs: function.has_lhs,
            has_self: function.has_self,
            has_new: function.has_new,
            closure,
            required_ctx,
        }
    }
}

// NOTE: this assumes that the block is the last instruction of the `init` RAST
impl<'a> fmt::Debug for RFunction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("RFunction");
        builder.field("args", &self.args);
        let instructions = &self.body.borrow().instructions;
        match &instructions[instructions.len() - 1].0 {
            RASTNode::Block(expr) => {
                builder.field("body", &expr.borrow());
            }
            _ => {
                builder.field("body", &String::from("<error>"));
            }
        }
        builder.field("has_lhs", &self.has_lhs);
        builder.field("has_self", &self.has_self);
        builder.finish()
    }
}

#[derive(Clone, Debug)]
pub struct RFunctionArg<'a> {
    pub name: String,
    pub argtype: RStructWeak<'a>,
    pub strictness: TypeStrictness,
}

impl<'a> From<(FunctionArg, RASTWeak<'a>, Location<'a>)> for RFunctionArg<'a> {
    fn from(input: (FunctionArg, RASTWeak<'a>, Location<'a>)) -> RFunctionArg<'a> {
        let name = input.0.name;
        let parent = input.1;
        let loc = input.2;

        if let Some(argtype) = input.0.argtype {
            let st = lookup::lookup_struct(
                TypeName { name: argtype.name },
                loc.clone(),
                &Vec::new(),
                parent.clone(),
            );

            RFunctionArg {
                name,
                argtype: Rc::downgrade(&st),
                strictness: argtype.strictness,
            }
        } else {
            RFunctionArg {
                name,
                argtype: Weak::new(),
                strictness: TypeStrictness::Normal,
            }
        }
    }
}

fn scan_body_reqs<'a>(
    body: RASTRef<'a>,
    refs: &'_ Vec<(String, Location<'a>)>,
    max_depth: usize,
    fn_location: &Location<'a>,
) -> Option<(usize, u128, Location<'a>)> {
    let mut res: Option<(usize, u128, Location)> = None;
    for instruction in &body.borrow().instructions {
        res = merge_reqs(
            res,
            scan_body_reqs_node(
                (&instruction.0, &instruction.1),
                refs,
                max_depth,
                fn_location,
            ),
        );
    }
    res
}

fn scan_body_reqs_node<'a>(
    instruction: (&RASTNode<'a>, &Location<'a>),
    refs: &'_ Vec<(String, Location<'a>)>,
    max_depth: usize,
    fn_location: &Location<'a>,
) -> Option<(usize, u128, Location<'a>)> {
    match instruction {
        (RASTNode::Variable(sym), loc) => {
            scan_body_reqs_sym(sym, loc, refs, max_depth, fn_location)
        }
        (RASTNode::VariableDef(sym, value), loc) => merge_reqs(
            scan_body_reqs_node((value.as_ref(), loc), refs, max_depth, fn_location),
            scan_body_reqs_sym(sym, loc, refs, max_depth, fn_location),
        ),
        (RASTNode::PatternCall(_, rast), _loc)
        | (RASTNode::Block(rast), _loc)
        | (RASTNode::MethodCall(_, rast), _loc) => {
            scan_body_reqs(rast.clone(), refs, max_depth, fn_location)
        }
        (RASTNode::ComplexDef(expr, _, value), loc) => merge_reqs(
            scan_body_reqs_node((value.as_ref(), loc), refs, max_depth, fn_location),
            scan_body_reqs_expr(expr, loc, refs, max_depth, fn_location),
        ),
        (RASTNode::Expression(expr), loc) => {
            scan_body_reqs_expr(expr, loc, refs, max_depth, fn_location)
        }
        (RASTNode::Tuple(tuple, _), _loc) => {
            let mut res: Option<(usize, u128, Location)> = None;
            for instruction in tuple {
                res = merge_reqs(
                    res,
                    scan_body_reqs_node(
                        (&instruction.0, &instruction.1),
                        refs,
                        max_depth,
                        fn_location,
                    ),
                );
            }
            res
        }
        _ => None,
    }
}

fn scan_body_reqs_sym<'a>(
    sym: &RSymRef,
    location: &Location<'a>,
    refs: &'_ Vec<(String, Location<'a>)>,
    max_depth: usize,
    fn_location: &Location<'a>,
) -> Option<(usize, u128, Location<'a>)> {
    if sym.depth < max_depth {
        if let None = refs.iter().find(|(name, _loc)| *name == sym.name) {
            CompError::new(
                154,
                format!("Expected symbol {} in function body to either be in a closure (#with) or to be explicitedly referenced (#ref)", sym.name),
                location.into()
            ).append(
                format!("Consider adding #with({}) or #ref({}) to the function's parameters", sym.name, sym.name),
                fn_location.into()
            ).print_and_exit();
        }
        Some((sym.depth, sym.ulid, location.clone()))
    } else {
        None
    }
}

fn scan_body_reqs_expr<'a>(
    expr: &RExpression<'a>,
    location: &Location<'a>,
    refs: &Vec<(String, Location<'a>)>,
    max_depth: usize,
    fn_location: &Location<'a>,
) -> Option<(usize, u128, Location<'a>)> {
    let mut res: Option<(usize, u128, Location)> = None;
    for term in &expr.terms {
        res = merge_reqs(
            res,
            match term {
                RExprTerm::Op(_) => None,
                RExprTerm::Push(node) => {
                    scan_body_reqs_node((node, location), refs, max_depth, fn_location)
                }
            },
        );
    }
    res
}

fn merge_reqs<'a>(
    a: Option<(usize, u128, Location<'a>)>,
    b: Option<(usize, u128, Location<'a>)>,
) -> Option<(usize, u128, Location<'a>)> {
    match (a, b) {
        (None, y) => y,
        (Some((a_depth, a_ulid, a_loc)), Some((b_depth, b_ulid, b_loc))) => {
            if a_depth < b_depth {
                Some((b_depth, b_ulid, b_loc))
            } else {
                Some((a_depth, a_ulid, a_loc))
            }
        }
        (x, None) => x,
    }
}
