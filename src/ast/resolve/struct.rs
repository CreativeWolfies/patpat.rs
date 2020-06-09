use super::*;
use rusty_ulid::Ulid;

#[derive(Debug, Clone)]
pub struct RStruct<'a> {
    id: u128,
    pub name: TypeName,
    pub context: Option<RASTRef<'a>>,
    pub interpretations: Vec<(RStructWeak<'a>, RASTRef<'a>)>,
}

impl<'a> RStruct<'a> {
    pub fn new(name: TypeName) -> RStruct<'a> {
        RStruct {
            id: Ulid::generate().into(),
            name: name,
            context: None,
            interpretations: Vec::new(),
        }
    }

    pub fn add_interpretation(
        &mut self,
        to: RStructWeak<'a>,
        body: AST<'a>,
        loc: Location<'a>,
        parent: RASTWeak<'a>,
    ) {
        let init = Rc::new(RefCell::new(RAST::new(parent, ASTKind::Block)));
        init.borrow_mut()
            .variables
            .push(Rc::new(RefCell::new(RSymbol::new(String::from("from")))));
        init.borrow_mut()
            .variables
            .push(Rc::new(RefCell::new(RSymbol::new(String::from("to")))));
        let body = RAST::resolve(body, Rc::downgrade(&init));
        init.borrow_mut()
            .instructions
            .push((RASTNode::Block(body), loc));
        self.interpretations.push((to, init));
    }

    pub fn get_method(&self, name: String) -> Option<RPatRef<'a>> {
        if let Some(ctx) = &self.context {
            for pattern in &ctx.borrow().patterns {
                if pattern.get_name() == name {
                    return Some(pattern.clone())
                }
            }
        }
        None
    }

    pub fn is_subtype_of(&self, other: RStructRef<'_>) -> bool {
        //! Asserts that self.context has been set
        self.context.as_ref().map(|ctx_self| other.borrow().context.as_ref().map(|ctx_other| {
            for pattern_self in &ctx_self.borrow().patterns {
                let mut found = false;
                for pattern_other in &ctx_other.borrow().patterns {
                    if pattern_other.get_name() == pattern_self.get_name() {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return false;
                }
            }
            for variable_self in &ctx_self.borrow().variables {
                let mut found = false;
                for variable_other in &ctx_other.borrow().variables {
                    if variable_other.borrow().name == variable_self.borrow().name {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return false;
                }
            }
            true
        })).unwrap_or(Some(false)).unwrap()
    }

    pub fn can_turn_into(&self, other: RStructRef<'_>) -> bool {
        //! Asserts that self.context has been set
        self.context.as_ref().map(|ctx_self| other.borrow().context.as_ref().map(|ctx_other| {
            for variable_self in &ctx_self.borrow().variables {
                let mut found = false;
                for variable_other in &ctx_other.borrow().variables {
                    if variable_other.borrow().name == variable_self.borrow().name {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return false;
                }
            }
            true
        })).unwrap_or(Some(false)).unwrap()
    }
}

impl<'a> PartialEq for RStruct<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
