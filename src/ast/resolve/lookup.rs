use super::*;

// TODO: Support #use and #load

/** Looks up a symbol in the RAST
  This function walks up through the RAST to find any a variable named `name`.
*/
pub fn lookup_symbol<'a, 'b>(name: String, loc: Location<'b>, variables: &'b Vec<Rc<RefCell<RSymbol<'a>>>>, parent: Weak<RefCell<RAST<'a>>>) -> Rc<RefCell<RSymbol<'a>>> {
  for var in variables {
    if var.borrow().name == name {
      return var.clone(); // move out of 'b
    }
  }
  match parent.upgrade() {
    Some(p) => {
      lookup_symbol(
        name,
        loc.clone(),
        &p.borrow().variables,
        p.borrow().parent.clone()
      )
    }
    None => {
      CompError::new(
        151,
        format!("Unknown symbol {}: couldn't resolve it", name),
        CompLocation::from(loc)
      ).print_and_exit();
    }
  }
}

/** Looks up a pattern in the RAST
  This function walks up through the RAST to find any a pattern named `name`.
*/
pub fn lookup_pattern<'a, 'b>(name: String, loc: Location<'b>, patterns: &'b Vec<Rc<RefCell<RPattern<'a>>>>, parent: Weak<RefCell<RAST<'a>>>) -> Rc<RefCell<RPattern<'a>>> {
  for pat in patterns {
    if pat.borrow().name == name {
      return pat.clone(); // move out of 'b
    }
  }
  match parent.upgrade() {
    Some(p) => {
      lookup_pattern(
        name,
        loc.clone(),
        &p.borrow().patterns,
        p.borrow().parent.clone()
      )
    }
    None => {
      CompError::new(
        152,
        format!("Unknown pattern {}: couldn't resolve it", name),
        CompLocation::from(loc)
      ).print_and_exit();
    }
  }
}
