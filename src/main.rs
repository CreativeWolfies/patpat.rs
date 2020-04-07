use std::env;
use std::fs;
use std::process;
pub mod ast;
pub mod error;
pub mod interpreter;
pub mod location;
pub mod parser;
pub mod test;
use ast::resolve;
pub use location::Location;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        exit_with_style("Invalid number of arguments");
    }

    let raw = match fs::read_to_string(&args[1]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Couldn't read file ({}): {}", &args[1], e);
            process::exit(7);
        }
    };
    let src_file = SrcFile {
        path: args[1].to_string(),
        contents: raw,
    };

    let parsed = parser::parse(&src_file);
    if let Some(_) = args.iter().find(|x| **x == String::from("--dump-parsed")) {
        println!("{:#?}", parsed);
    }

    let constructed = parser::construct(parsed);
    if let Some(_) = args
        .iter()
        .find(|x| **x == String::from("--dump-constructed"))
    {
        println!("{:#?}", constructed);
    }

    let resolved = resolve::resolve(constructed);
    if let Some(_) = args.iter().find(|x| **x == String::from("--dump-resolved")) {
        println!("{:#?}", resolved.borrow());
    }

    interpreter::interprete(resolved, Vec::new());
}

fn exit_with_style(msg: &str) {
    // have ~style~
    eprintln!("{}", msg);
    process::exit(1);
}

pub struct SrcFile {
    path: String,
    contents: String,
}
