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
}

impl<'a> PartialEq for RStruct<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
