use std::fs;
use std::env;
use std::process;
pub mod parser;
pub mod error;
pub mod ast;
pub mod location;
pub use location::Location;
use ast::resolve;

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
        contents: raw
    };

    let parsed = parser::parse(&src_file);
    if let Some(_) = args.iter().find(|x| **x == String::from("--dump-parsed")) {
        println!("{:#?}", parsed);
    }

    let constructed = parser::construct(parsed);
    if let Some(_) = args.iter().find(|x| **x == String::from("--dump-constructed")) {
        println!("{:#?}", constructed);
    }

    let resolved = resolve::resolve(constructed);
    if let Some(_) = args.iter().find(|x| **x == String::from("--dump-resolved")) {
        println!("{:#?}", resolved.borrow());
    }
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
