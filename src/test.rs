// This only contains test utilities
use super::{ast, internal, error, interpreter, parser, SrcFile};
use std::fs;

pub fn init_testenv() {
    error::COMPERROR_EXIT.with(|e| *e.borrow_mut() = false);
    internal::TEST_LOG.with(|t| *t.borrow_mut() = String::new());
}

pub fn load(path: &str) -> SrcFile {
    let path = String::from(path);
    let raw = match fs::read_to_string(&path) {
        Ok(v) => v,
        Err(e) => {
            panic!("Couldn't read file ({}): {}", &path, e);
        }
    };

    SrcFile {
        path,
        contents: raw,
    }
}

pub fn compile<'a>(src_file: &'a SrcFile) -> ast::RASTRef<'a> {
    let parsed = parser::parse(&src_file);
    let constructed = parser::construct(parsed);
    ast::resolve::resolve(constructed)
}

pub fn execute<'a>(program: ast::RASTRef<'a>) -> interpreter::VariableValue<'a> {
    interpreter::interprete(program, Vec::new())
}

pub fn get_logs() -> String {
    internal::TEST_LOG.with(|t| t.borrow().clone())
}
