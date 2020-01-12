use regex::{Regex, Captures};
use std::{
    process
};
mod token;

pub fn parse(raw: String) -> AST {
    let lines: Vec<&str> = raw.split('\n').collect();
    let mut token_stack: Vec<AST> = Vec::new();
    token_stack.push(AST::new(token::Kind::ASTRoot));

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
        //      v-- mut &str
        let mut trimmed_line = line.clone(); // a copy of the line, which progressively gets trimmed
        while trimmed_line.len() > 0 {
            let matched = match_next_term(&mut trimmed_line, &mut token_stack, &regexes);
            if !matched {
                eprintln!("Unrecognized term: '{}'", trimmed_line);
                process::exit(3);
            }
        }
    }

    if token_stack.len() > 1 {
        eprintln!("Unexpected EOF; did you forget a closing parenthesis?");
        process::exit(5);
    }

    match token_stack.pop() {
        Some(t) => {
            println!("{:?}", t);
            t
        },
        None => {
            eprintln!("Empty token stack (1)");
            process::exit(1);
        }
    }
}

fn match_next_term(trimmed_line: &mut &str, token_stack: &mut Vec<AST>, regexes: &Vec<(token::Kind, Regex)>) -> bool {
    let mut res = false; // wether or not a match occured
    for matcher in regexes.iter() {
        if let Some(caps) = matcher.1.captures(trimmed_line) {
            *trimmed_line = trimmed_line.split_at(caps.get(0).unwrap().as_str().len()).1;
            match matcher.0 {
                token::Kind::Space => { /* noop */ },
                token::Kind::Comment => {*trimmed_line = ""; res = true; break;},
                token::Kind::TupleStart => {
                    token_stack.push(AST::new(token::Kind::Tuple));
                },
                token::Kind::TupleEnd => {
                    if let Some(ast) = token_stack.pop() {
                        match ast.kind {
                            token::Kind::Tuple => {},
                            _ => {
                                // TODO: syntax error
                                eprintln!("Unexpected token TupleEnd ')': not in a tuple");
                                process::exit(101);
                            }
                        }
                        if let Some(parent_ast) = token_stack.last_mut() {
                            parent_ast.tokens.push(token::Token::Tuple(ast));
                        } else {
                            eprintln!("Empty token stack (2)");
                            process::exit(1);
                        }
                    } else {
                        eprintln!("Empty token stack (3)");
                        process::exit(1);
                    }
                }
                _ => {
                    let term = token::Token::from_match(&caps, &matcher.0);
                    println!("{:?}", &term);
                    if let Some(t) = token_stack.last_mut() {
                        t.tokens.push(term);
                    } else {
                        eprintln!("Empty token stack (4)");
                        process::exit(1);
                    }
                }
            };
            res = true;
            break;
        }
    }
    res
}

pub const MATCHERS: [(token::Kind, &str); 9] = [
    (token::Kind::Boolean, "^(true|false)"),
    (token::Kind::Let, "^let"),
    (token::Kind::Symbol, "^[a-z_][a-z_\\d]*"),
    (token::Kind::Define, "^:"),
    (token::Kind::Space, "^[\\s\\t]+"),
    (token::Kind::Comment, "^//"),
    (token::Kind::Pattern, "^['#]\\w(?:[\\w_\\d]|::)*"),
    (token::Kind::TupleStart, "^\\("),
    (token::Kind::TupleEnd, "^\\)"),
];
// This should be enough to be able to parse `let is_toast: true`

#[derive(Debug)]
pub struct AST {
    tokens: Vec<token::Token>,
    kind: token::Kind,
}

impl AST {
    fn new(kind: token::Kind) -> AST {
        AST {
            tokens: Vec::new(),
            kind
        }
    }
}
