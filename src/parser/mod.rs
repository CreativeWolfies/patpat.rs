use regex::{Regex, Captures};
use std::{
    process
};
mod token;

pub fn parse(raw: String) -> AST {
    let lines: Vec<&str> = raw.split('\n').collect();
    let mut tokens: Vec<AST> = Vec::new();
    tokens.push(AST::new());

    // generate the Regex objects from the MATCHERS array
    let mut regexes: Vec<(token::Kind, Regex)> = Vec::new();
    for matcher in MATCHERS.iter() {
        regexes.push((matcher.0, match Regex::new(matcher.1) {
            Ok(val) => val,
            _ => {
                eprintln!("     __\n  _ / /\n (_) | \n   | | \n  _| | \n (_) | \n    \\_\\\n"); // x3
                process::exit(1);
            },
        }));
    }

    for line in lines.iter() { // for each line
        let mut trimmed_line = line.clone(); // a copy of the line, which progressively gets trimmed
        while trimmed_line.len() > 0 {
            let mut matched = false;
            for matcher in regexes.iter() {
                match matcher.1.captures(trimmed_line) {
                    Some(caps) => {
                        trimmed_line = trimmed_line.split_at(caps.get(0).unwrap().as_str().len()).1;
                        match matcher.0 {
                            token::Kind::Space => { /* noop */ },
                            token::Kind::Comment => {trimmed_line = ""; matched = true; break;},
                            _ => {
                                let term = token::Token::from_match(&caps, &matcher.0);
                                println!("{:?}", &term);
                                match tokens.last_mut() {
                                    Some(t) => t.tokens.push(term),
                                    None => {
                                        eprintln!("Empty token stack (1)");
                                        process::exit(1);
                                    }
                                }
                            }
                        };
                        matched = true;
                        break;
                    },
                    None => {
                        // println!("No match -- {:?} ({:?})", &matcher.0, &trimmed_line); // DEBUG
                    },
                }
            }
            if !matched {
                eprintln!("Unrecognized term: '{}'", trimmed_line);
                process::exit(3);
            }
        }
    }

    if tokens.len() > 1 {
        eprintln!("Unexpected EOF; did you forget a closing parenthesis?");
        process::exit(5);
    }

    match tokens.pop() {
        Some(t) => t,
        None => {
            eprintln!("Empty token stack (2)");
            process::exit(1);
        }
    }
}

pub const MATCHERS: [(token::Kind, &str); 7] = [
    (token::Kind::Boolean, "^(true|false)"),
    (token::Kind::Let, "^let"),
    (token::Kind::Symbol, "^[a-z_][a-z_\\d]*"),
    (token::Kind::Define, "^:"),
    (token::Kind::Space, "^[\\s\\t]+"),
    (token::Kind::Comment, "^//"),
    (token::Kind::Pattern, "^['#]\\w(?:[\\w_\\d]|::)*"),
];
// This should be enough to be able to parse `let is_toast: true`

pub struct AST {
    tokens: Vec<token::Token>
}

impl AST {
    fn new() -> AST {
        AST {
            tokens: Vec::new()
        }
    }
}
