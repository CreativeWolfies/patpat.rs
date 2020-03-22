pub use super::*;
pub use super::symbol::Symbol;
use std::cell::RefCell;

#[derive(Debug)]
#[derive(Clone)]
pub struct RAST<'a> { // resolved AST
  pub instructions: Vec<(RASTNode<'a>, Location<'a>)>,
  pub parent: Option<Rc<RefCell<RAST<'a>>>>,
  pub variables: Vec<Rc<RefCell<Symbol<'a>>>>,
}

pub fn resolve<'a>(ast: AST<'a>) -> Rc<RefCell<RAST<'a>>> {
  RAST::resolve(ast, None)
}

impl<'a> RAST<'a> {
  pub fn new(parent: Option<Rc<RefCell<RAST<'a>>>>) -> RAST<'a> {
    RAST {
      instructions: Vec::new(),
      parent,
      variables: Vec::new()
    }
  }

  pub fn resolve(ast: AST<'a>, parent: Option<Rc<RefCell<RAST<'a>>>>) -> Rc<RefCell<RAST<'a>>> {
    let res = Rc::new(RefCell::new(RAST {
      instructions: Vec::new(),
      parent: parent.clone(),
      variables: Vec::new(),
    }));

    for instruction in ast.instructions.into_iter() {
      match instruction {
        (ASTNode::VariableDecl(name), _) => res.borrow_mut().variables.push(Rc::new(RefCell::new(Symbol::new(name)))),
        (ASTNode::VariableInit(name, expr), loc) => {
          let s = Rc::new(RefCell::new(Symbol::new(name)));
          res.borrow_mut().variables.push(s.clone());
          res.borrow_mut().instructions.push((
            RASTNode::VariableDef(s, Rc::new(RAST::resolve_node((*expr, loc.clone()), parent.clone()))),
            loc
          ));
        },
        _ => {}
      }
    }

    res
  }

  pub fn resolve_node(node: (ASTNode<'a>, Location<'a>), parent: Option<Rc<RefCell<RAST<'a>>>>) -> RefCell<RAST<'a>> {
    match node {
      (ASTNode::VariableDef(name, expr), loc) => {
        let variables: Vec<Rc<RefCell<Symbol>>> = Vec::new();
        RefCell::new(RAST {
          instructions: vec![(RASTNode::VariableDef(
            lookup_symbol(name, loc.clone(), &variables, parent.clone()),
            Rc::new(RAST::resolve_node((*expr, loc.clone()), parent.clone()))
          ), loc)],
          parent: parent.clone(),
          variables,
        })
      },
      _ => RefCell::new(RAST::new(parent))
    }
  }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum RASTNode<'a> { // resolved AST node
  PatternCall(Rc<Pattern<'a>>, Rc<RefCell<RAST<'a>>>),
  VariableDef(Rc<RefCell<Symbol<'a>>>, Rc<RefCell<RAST<'a>>>),
}

// TODO: Support #use and #load
fn lookup_symbol<'a, 'b>(name: String, loc: Location<'b>, variables: &'b Vec<Rc<RefCell<Symbol<'a>>>>, parent: Option<Rc<RefCell<RAST<'a>>>>) -> Rc<RefCell<Symbol<'a>>> {
  for var in variables {
    if var.borrow().name == name {
      return var.clone(); // move out of 'b
    }
  }
  match parent {
    Some(p) => {
      lookup_symbol(name, loc.clone(), &p.borrow().variables, p.borrow().parent.clone())
    }
    None => {
      CompError::new(
        151,
        String::from("Unknown symbol: couldn't resolve it"),
        CompLocation::from(loc)
      ).print_and_exit();
    }
  }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn lookup_pattern<'a, 'b>(name: String, loc: Location<'b>, patterns: &'b Vec<Rc<RefCell<Pattern<'a>>>>, parent: Option<Rc<RefCell<RAST<'a>>>>) -> Rc<RefCell<Pattern<'a>>> {
  panic!("Pattern lookup is unimplemented!");
}
