use std::fs;
use std::env;
use std::process;
pub mod parser;
pub mod error;
pub mod ast;

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
    parser::parse(raw);
}

fn exit_with_style(msg: &str) {
    // have ~style~
    eprintln!("{}", msg);
    process::exit(1);
}
