pub mod variable;
pub mod pattern;
pub mod function;
pub mod lookup;
pub mod node;
pub mod r#struct;

pub use super::*;
pub use variable::*;
pub use pattern::*;
pub use function::*;
pub use lookup::*;
pub use node::*;
pub use r#struct::*;
use std::cell::RefCell;
use std::rc::Weak;

/** Resolved abstract syntax tree (RAST): an AST referencing itself through its variables, functions, etc.
This resolved AST has all of its variables, patterns, etc. resolved (ie. they all point to their value's respective memory location).
RefCells are needed as these references may be neede to borrow the value mutably later.
*/

pub type RSymRef<'a> = Rc<RefCell<RSymbol<'a>>>;
pub type RPatRef<'a> = Rc<RefCell<RPattern<'a>>>;
pub type RStructRef<'a> = Rc<RefCell<RStruct<'a>>>;
pub type RFunRef<'a> = Rc<RefCell<RFunction<'a>>>;
pub type RASTRef<'a> = Rc<RefCell<RAST<'a>>>;
pub type RASTWeak<'a> = Weak<RefCell<RAST<'a>>>;

#[derive(Clone)]
#[derive(Debug)]
pub struct RAST<'a> {
  pub instructions: Vec<(RASTNode<'a>, Location<'a>)>,
  pub parent: RASTWeak<'a>,
  pub variables: Vec<RSymRef<'a>>,
  pub patterns: Vec<RPatRef<'a>>,
  pub structs: Vec<RStructRef<'a>>,
}

/// Calls RAST::resolve, returns the root node of the corresponding tree
pub fn resolve<'a>(ast: AST<'a>) -> RASTRef {
  RAST::resolve(ast, Weak::new())
}

impl<'a> RAST<'a> {
  /**
    Creates a new, empty RAST instance with as parent `parent`.
  */
  pub fn new(parent: RASTWeak<'a>) -> RAST<'a> {
    RAST {
      instructions: Vec::new(),
      parent,
      variables: Vec::new(),
      patterns: Vec::new(),
      structs: Vec::new(),
    }
  }

  /** Resolves the links within `ast`, returning a populated RAST.
  Note that this function is recursive.

  The resolution process has two phases: the first one (first pass) looks for declarations and registers them while the second one (second pass) registers the individual instructions to be carried out during runtime.

  */
  pub fn resolve(ast: AST<'a>, parent: RASTWeak<'a>) -> RASTRef<'a> {
    let res = Rc::new(RefCell::new(RAST::new(parent.clone())));

    for instruction in ast.instructions.iter() {
      // first pass: find variables and patterns
      match &instruction.0 {
        ASTNode::VariableDecl(name)
        | ASTNode::VariableInit(name, _) =>
          res.borrow_mut().variables.push(Rc::new(RefCell::new(RSymbol::new(name.clone())))),
        ASTNode::PatternDecl(p) =>
          res.borrow_mut().patterns.push(Rc::new(RefCell::new(RPattern::new(p.name.clone())))),
        ASTNode::Struct(name, _) =>
          res.borrow_mut().structs.push(Rc::new(RefCell::new(RStruct::new(name.clone())))),
        _ => {}
      }
    }

    for instruction in ast.instructions.into_iter() {
      // second pass: resolve instructions
      let loc = instruction.1;
      match instruction.0 {
        ASTNode::VariableInit(name, expr) => {
          let s = lookup_variable(name, loc.clone(), &res.borrow().variables, parent.clone());
          res.borrow_mut().instructions.push((
            RASTNode::VariableDef(
              s,
              Rc::new(RAST::resolve_node((*expr, loc.clone()), parent.clone()))
            ),
            loc
          ));
        },
        ASTNode::PatternDecl(p) => {
          let pat = lookup_pattern(p.name, loc, &res.borrow().patterns, parent.clone());
          let function = RFunction::from((p.function, Rc::downgrade(&res)));
          pat.borrow_mut().function = Some(function);
        },
        ASTNode::PatternCall(name, args) => {
          let pat = lookup_pattern(name, loc.clone(), &res.borrow().patterns, parent.clone());
          let args = RAST::resolve(args, Rc::downgrade(&res));
          res.borrow_mut().instructions.push((RASTNode::PatternCall(pat, args), loc));
        },
        ASTNode::Struct(name, body) => {
          let st = lookup_struct(name, loc.clone(), &res.borrow().structs, parent.clone());
          st.borrow_mut().context = Some(RAST::resolve(body, Rc::downgrade(&res)));
        },
        ASTNode::Function(function) => {
          let rfn = RFunction::from((function, Rc::downgrade(&res)));
          res.borrow_mut().instructions.push((
            RASTNode::Function(Rc::new(RefCell::new(rfn))),
            loc
          ));
        },
        ASTNode::Pattern(name) => {
          let pat = lookup_pattern(name, loc.clone(), &res.borrow().patterns, parent.clone());
          res.borrow_mut().instructions.push((RASTNode::Pattern(pat), loc));
        },
        ASTNode::Variable(name) => {
          let var = lookup_variable(name, loc.clone(), &res.borrow().variables, parent.clone());
          res.borrow_mut().instructions.push((RASTNode::Variable(var), loc));
        },
        ASTNode::Boolean(b) => {
          res.borrow_mut().instructions.push((
            RASTNode::Boolean(b),
            loc
          ));
        },
        ASTNode::Number(num) => {
          res.borrow_mut().instructions.push((
            RASTNode::Number(num),
            loc
          ));
        },
        _ => {}
      }
    }

    res
  }

  /** Resolves an individual node

  This is just a call to RAST::resolve (TODO)
  */
  pub fn resolve_node(node: (ASTNode<'a>, Location<'a>), parent: RASTWeak<'a>) -> RefCell<RAST<'a>> {
    match node {
      (ASTNode::VariableDef(name, expr), loc) => {
        let variables: Vec<Rc<RefCell<RSymbol>>> = Vec::new();
        RefCell::new(RAST {
          instructions: vec![(RASTNode::VariableDef(
            lookup_variable(name, loc.clone(), &variables, parent.clone()),
            Rc::new(RAST::resolve_node((*expr, loc.clone()), parent.clone()))
          ), loc)],
          parent: parent.clone(),
          variables,
          patterns: Vec::new(),
          structs: Vec::new(),
        })
      },
      _ => RefCell::new(RAST::new(parent))
    }
  }
}
