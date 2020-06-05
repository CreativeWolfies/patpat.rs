use super::*;

#[derive(Debug, Clone)]
pub struct RSymbol {
    // Only used during interpretation
    pub name: String,
}

impl RSymbol {
    pub fn new(name: String) -> RSymbol {
        RSymbol { name }
    }
}

#[derive(Debug, Clone)]
pub struct RSymRef {
    pub ast_ref: Rc<RefCell<RSymbol>>,
    pub name: String,
    pub depth: usize,
    pub ulid: u128,
}

impl RSymRef {
    pub fn new(ast_ref: Rc<RefCell<RSymbol>>, depth: usize, ulid: u128) -> RSymRef {
        let name = ast_ref.borrow().name.clone();
        RSymRef {
            name,
            ast_ref,
            depth,
            ulid,
        }
    }
}
