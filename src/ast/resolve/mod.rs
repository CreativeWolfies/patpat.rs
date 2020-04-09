pub mod expr;
pub mod function;
pub mod lookup;
pub mod node;
pub mod pattern;
pub mod r#struct;
pub mod variable;

pub use super::*;
pub use expr::*;
pub use function::*;
pub use node::*;
pub use pattern::*;
pub use r#struct::*;
use std::cell::RefCell;
use std::rc::Weak;
pub use variable::*;

/** Resolved abstract syntax tree (RAST): an AST referencing itself through its variables, functions, etc.
This resolved AST has all of its variables, patterns, etc. resolved (ie. they all point to their value's respective memory location).

TODO: cut down on RefCells
*/

pub type RPatRef<'a> = Rc<RefCell<RPattern<'a>>>;
pub type RStructRef<'a> = Rc<RefCell<RStruct<'a>>>;
pub type RStructWeak<'a> = Weak<RefCell<RStruct<'a>>>;
pub type RFunRef<'a> = Rc<RefCell<RFunction<'a>>>;
pub type RASTRef<'a> = Rc<RefCell<RAST<'a>>>;
pub type RASTWeak<'a> = Weak<RefCell<RAST<'a>>>;

#[derive(Clone, Debug)]
pub struct RAST<'a> {
    pub instructions: Vec<(RASTNode<'a>, Location<'a>)>,
    pub parent: RASTWeak<'a>,
    pub variables: Vec<Rc<RefCell<RSymbol>>>,
    pub patterns: Vec<RPatRef<'a>>,
    pub structs: Vec<RStructRef<'a>>,
    pub depth: usize,
    pub kind: ASTKind,
}

/** Calls RAST::resolve, returns the root node of the corresponding tree
* TODO: make it use a standard set of variables, structs, etc.
*/
pub fn resolve<'a>(ast: AST<'a>) -> RASTRef {
    RAST::resolve(ast, Weak::new())
}

impl<'a> RAST<'a> {
    /**
      Creates a new, empty RAST instance with as parent `parent`.
    */
    pub fn new(parent: RASTWeak<'a>, kind: ASTKind) -> RAST<'a> {
        RAST {
            instructions: Vec::new(),
            parent: parent.clone(),
            variables: Vec::new(),
            patterns: Vec::new(),
            structs: Vec::new(),
            depth: parent.upgrade().map(|p| p.borrow().depth + 1).unwrap_or(0),
            kind,
        }
    }

    /** Resolves the links within `ast`, returning a populated RAST.
    Note that this function is recursive.

    The resolution process has two phases: the first one (first pass) looks for declarations and registers them while the second one (second pass) registers the individual instructions to be carried out during runtime.

    */
    pub fn resolve(ast: AST<'a>, parent: RASTWeak<'a>) -> RASTRef<'a> {
        let res = Rc::new(RefCell::new(RAST::new(parent.clone(), ast.kind)));

        for instruction in ast.instructions.iter() {
            // first pass: find variables and patterns
            match &instruction.0 {
                ASTNode::VariableDecl(name) | ASTNode::VariableInit(name, _) => res
                    .borrow_mut()
                    .variables
                    .push(Rc::new(RefCell::new(RSymbol::new(name.clone())))),
                ASTNode::PatternDecl(p) => res
                    .borrow_mut()
                    .patterns
                    .push(Rc::new(RefCell::new(RPattern::new(p.name.clone())))),
                ASTNode::Struct(name, _) => res
                    .borrow_mut()
                    .structs
                    .push(Rc::new(RefCell::new(RStruct::new(name.clone())))),
                _ => {}
            }
        }

        for instruction in ast.instructions.into_iter() {
            // second pass: resolve instructions
            let loc = instruction.1.clone();
            let instruction = RAST::resolve_node(instruction, res.clone());
            match instruction {
                Some(i) => {
                    res.borrow_mut().instructions.push((i, loc));
                }
                None => {}
            }
        }

        res
    }

    /** Resolves an individual node and optionally returns an instruction
     */
    pub fn resolve_node(
        node: (ASTNode<'a>, Location<'a>),
        res: RASTRef<'a>,
    ) -> Option<RASTNode<'a>> {
        let loc = node.1;
        let parent = res.borrow().parent.clone();
        match node.0 {
            ASTNode::VariableInit(name, expr) | ASTNode::VariableDef(name, expr) => {
                let s = lookup::lookup_variable(
                    name,
                    loc.clone(),
                    &res.borrow().variables,
                    parent.clone(),
                );
                Some(RASTNode::VariableDef(
                    s,
                    Box::new(
                        RAST::resolve_node((*expr, loc.clone()), res.clone())
                            .unwrap_or(RASTNode::Nil),
                    ),
                ))
            }
            ASTNode::Interpretation(from, to, body) => {
                let from =
                    lookup::lookup_struct(from, loc.clone(), &res.borrow().structs, parent.clone());
                let to =
                    lookup::lookup_struct(to, loc.clone(), &res.borrow().structs, parent.clone());
                from.borrow_mut().add_interpretation(
                    Rc::downgrade(&to),
                    body,
                    loc,
                    Rc::downgrade(&res),
                );
                None
            }
            ASTNode::PatternDecl(p) => {
                let pat = lookup::lookup_pattern(
                    p.name,
                    loc.clone(),
                    &res.borrow().patterns,
                    parent.clone(),
                );
                let function = RFunction::from((p.function, Rc::downgrade(&res), loc));
                pat.borrow_mut().function = Some(function);
                None
            }
            ASTNode::PatternCall(name, args) => {
                let pat = lookup::lookup_pattern(
                    name,
                    loc.clone(),
                    &res.borrow().patterns,
                    parent.clone(),
                );
                let args = RAST::resolve(args, Rc::downgrade(&res));
                Some(RASTNode::PatternCall(pat, args))
            }
            ASTNode::MethodCall(name, args) => {
                let args = RAST::resolve(args, Rc::downgrade(&res));
                Some(RASTNode::MethodCall(name, args))
            }
            ASTNode::Struct(name, body) => {
                let st =
                    lookup::lookup_struct(name, loc.clone(), &res.borrow().structs, parent.clone());
                st.borrow_mut().context = Some(RAST::resolve(body, Rc::downgrade(&res)));
                None
            }
            ASTNode::Function(function) => {
                let rfn = RFunction::from((function, Rc::downgrade(&res), loc));
                Some(RASTNode::Function(Rc::new(RefCell::new(rfn))))
            }
            ASTNode::Pattern(name) => {
                let pat = lookup::lookup_pattern(
                    name,
                    loc.clone(),
                    &res.borrow().patterns,
                    parent.clone(),
                );
                Some(RASTNode::Pattern(pat))
            }
            ASTNode::Variable(name) => {
                let var = lookup::lookup_variable(
                    name,
                    loc.clone(),
                    &res.borrow().variables,
                    parent.clone(),
                );
                Some(RASTNode::Variable(var))
            }
            ASTNode::Member(name) => Some(RASTNode::Member(name)),
            ASTNode::Boolean(b) => Some(RASTNode::Boolean(b)),
            ASTNode::Number(num) => Some(RASTNode::Number(num)),
            ASTNode::String(string) => Some(RASTNode::String(string)),
            ASTNode::Expression(expr) => {
                let mut terms: Vec<RExprTerm<'a>> = Vec::with_capacity(expr.terms.len());
                let mut depth: usize = 0;
                let mut max_depth: usize = 0;
                for term in expr.terms.into_iter() {
                    match term {
                        ExprTerm::Push(node, loc) => {
                            terms.push(RExprTerm::Push(
                                RAST::resolve_node((node, loc), res.clone())
                                    .unwrap_or(RASTNode::Nil),
                            ));
                            depth += 1;
                            if depth > max_depth {
                                max_depth = depth;
                            }
                        }
                        ExprTerm::Op(operator) => {
                            terms.push(RExprTerm::Op(operator));
                            if operator.is_unary() {
                                depth -= 1;
                            }
                        }
                    }
                }
                Some(RASTNode::Expression(RExpression { terms, max_depth }))
            }
            ASTNode::ComplexDef(expr, member, val) => {
                let expr =
                    RAST::resolve_node((ASTNode::Expression(expr), loc.clone()), res.clone())
                        .unwrap();
                let val = RAST::resolve_node((*val, loc), res.clone()).unwrap_or(RASTNode::Nil);
                if let RASTNode::Expression(expr) = expr {
                    Some(RASTNode::ComplexDef(expr, member, Box::new(val)))
                } else {
                    panic!("RAST::resolve_node did not return an expression");
                }
            }
            ASTNode::Tuple(ast) => {
                let mut elements: Vec<(RASTNode<'a>, Location<'a>)> =
                    Vec::with_capacity(ast.instructions.len());
                for instruction in ast.instructions.into_iter() {
                    let loc = instruction.1.clone();
                    elements.push((
                        RAST::resolve_node(instruction, res.clone()).unwrap_or(RASTNode::Nil),
                        loc,
                    ));
                }
                Some(RASTNode::Tuple(elements))
            }
            ASTNode::Block(ast) => {
                let block = RAST::resolve(ast, Rc::downgrade(&res));
                Some(RASTNode::Block(block))
            }
            ASTNode::TypeName(name) => {
                let st =
                    lookup::lookup_struct(name, loc.clone(), &res.borrow().structs, parent.clone());
                Some(RASTNode::TypeName(st))
            }
            ASTNode::Nil => Some(RASTNode::Nil),
            _ => None,
        }
    }
}
