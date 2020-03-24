pub mod symbol;
pub mod pattern;
pub mod function;
pub mod lookup;
pub mod node;

pub use super::*;
pub use symbol::*;
pub use pattern::*;
pub use function::*;
pub use lookup::*;
pub use node::*;
use std::cell::RefCell;
use std::rc::Weak;


/** Resolved abstract syntax tree (RAST): an AST referencing itself through its variables, functions, etc.
This resolved AST has all of its variables, patterns, etc. resolved (ie. they all point to their value's respective memory location).
RefCells are needed as these references may be neede to borrow the value mutably later.
*/
#[derive(Clone)]
#[derive(Debug)]
pub struct RAST<'a> {
  pub instructions: Vec<(RASTNode<'a>, Location<'a>)>,
  pub parent: Weak<RefCell<RAST<'a>>>,
  pub variables: Vec<Rc<RefCell<RSymbol<'a>>>>,
  pub patterns: Vec<Rc<RefCell<RPattern<'a>>>>,
}

/// Calls RAST::resolve, returns the root node of the corresponding tree
pub fn resolve<'a>(ast: AST<'a>) -> Rc<RefCell<RAST<'a>>> {
  RAST::resolve(ast, Weak::new())
}

impl<'a> RAST<'a> {
  /**
    Creates a new, empty RAST instance with as parent `parent`.
  */
  pub fn new(parent: Weak<RefCell<RAST<'a>>>) -> RAST<'a> {
    RAST {
      instructions: Vec::new(),
      parent,
      variables: Vec::new(),
      patterns: Vec::new(),
    }
  }

  /** Resolves the links within `ast`, returning a populated RAST.
  Note that this function is recursive.

  The resolution process has two phases: the first one (first pass) looks for declarations and registers them while the second one (second pass) registers the individual instructions to be carried out during runtime.

  */
  pub fn resolve(ast: AST<'a>, parent: Weak<RefCell<RAST<'a>>>) -> Rc<RefCell<RAST<'a>>> {
    let res = Rc::new(RefCell::new(RAST::new(parent.clone())));

    for instruction in ast.instructions.iter() {
      // first pass: find symbols and patterns
      match &instruction.0 {
        ASTNode::VariableDecl(name)
        | ASTNode::VariableInit(name, _) =>
          res.borrow_mut().variables.push(Rc::new(RefCell::new(RSymbol::new(name.clone())))),
        ASTNode::PatternDecl(p) =>
          res.borrow_mut().patterns.push(Rc::new(RefCell::new(RPattern::new(p.name.clone())))),
        _ => {}
      }
    }

    for instruction in ast.instructions.into_iter() {
      // second pass: resolve instructions
      let loc = instruction.1;
      match instruction.0 {
        ASTNode::VariableInit(name, expr) => {
          let s = lookup_symbol(name, loc.clone(), &res.borrow().variables, parent.clone());
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
        }
        _ => {}
      }
    }

    res
  }

  /** Resolves an individual node

  This is just a call to RAST::resolve (TODO)
  */
  pub fn resolve_node(node: (ASTNode<'a>, Location<'a>), parent: Weak<RefCell<RAST<'a>>>) -> RefCell<RAST<'a>> {
    match node {
      (ASTNode::VariableDef(name, expr), loc) => {
        let variables: Vec<Rc<RefCell<RSymbol>>> = Vec::new();
        RefCell::new(RAST {
          instructions: vec![(RASTNode::VariableDef(
            lookup_symbol(name, loc.clone(), &variables, parent.clone()),
            Rc::new(RAST::resolve_node((*expr, loc.clone()), parent.clone()))
          ), loc)],
          parent: parent.clone(),
          variables,
          patterns: Vec::new(),
        })
      },
      _ => RefCell::new(RAST::new(parent))
    }
  }
}
