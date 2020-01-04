use std::fs;
use std::env;
use std::process;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        exit_with_style("Invalid number of arguments");
    }

    let raw = fs::read_to_string(&args[1]).unwrap();
    parser::parse(raw);
}

fn exit_with_style(msg: &str) {
    // have ~style~
    eprintln!("{}", msg);
    process::exit(1);
}
