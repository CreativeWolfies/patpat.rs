use super::*;

// TODO: Support #use and #load

/** Looks up a variable in the RAST
  This function walks up through the RAST to find any a variable named `name`.
*/
pub fn lookup_variable<'a, 'b>(name: String, loc: Location<'b>, variables: &'b Vec<Rc<RefCell<RSymbol>>>, parent: RASTWeak<'a>) -> RSymRef {
  lookup_variable_rec(name, loc, variables, parent, 0)
}

fn lookup_variable_rec<'a, 'b>(
  name: String,
  loc: Location<'b>,
  variables: &'b Vec<Rc<RefCell<RSymbol>>>,
  parent: RASTWeak<'a>,
  depth: usize
) -> RSymRef {
  for var in variables {
    if var.borrow().name == name {
      return RSymRef::new(var.clone(), depth);
    }
  }
  match parent.upgrade() {
    Some(p) => {
      lookup_variable_rec(
        name,
        loc.clone(),
        &p.borrow().variables,
        p.borrow().parent.clone(),
        depth + 1
      )
    }
    None => {
      CompError::new(
        151,
        format!("Unknown variable {}: couldn't resolve it", name),
        CompLocation::from(loc)
      ).print_and_exit();
    }
  }
}

/** Looks up a pattern in the RAST
  This function walks up through the RAST to find any a pattern named `name`.
*/
pub fn lookup_pattern<'a, 'b>(name: String, loc: Location<'b>, patterns: &'b Vec<RPatRef<'a>>, parent: RASTWeak<'a>) -> RPatRef<'a> {
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

/** Looks up a struct in the RAST
  This function walks up through the RAST to find any a struct named `name`.
*/
pub fn lookup_struct<'a, 'b>(name: TypeName, loc: Location<'b>, structs: &'b Vec<RStructRef<'a>>, parent: RASTWeak<'a>) -> RStructRef<'a> {
  for st in structs {
    if st.borrow().name == name {
      return st.clone(); // move out of 'b
    }
  }
  match parent.upgrade() {
    Some(p) => {
      lookup_struct(
        name,
        loc.clone(),
        &p.borrow().structs,
        p.borrow().parent.clone()
      )
    }
    None => {
      CompError::new(
        153,
        format!("Unknown struct {}: couldn't resolve it", name),
        CompLocation::from(loc)
      ).print_and_exit();
    }
  }
}
