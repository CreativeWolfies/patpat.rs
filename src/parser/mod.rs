use regex::{Regex, Captures};
use std::{
    process
};

pub fn parse(raw: String) {
    let lines: Vec<&str> = raw.split('\n').collect();
    let mut terms: Vec<Term> = Vec::new();
    for line in lines.iter() {
        let mut trimmed_line = line.clone();
        while trimmed_line.len() > 0 {
            let mut matched = false;
            for matcher in MATCHERS.iter() {
                let regex = match Regex::new(matcher.1) {
                    Ok(val) => val,
                    _ => {
                        eprintln!("     __\n  _ / /\n (_) | \n   | | \n  _| | \n (_) | \n    \\_\\\n"); // x3
                        process::exit(1);
                    },
                };
                // tu as besoin d'aide ? - ?
                match regex.captures(trimmed_line) {
                    Some(caps) => {
                        trimmed_line = trimmed_line.split_at(caps.get(0).unwrap().as_str().len()).1;
                        match matcher.0 {
                            patpat::Kinds::Space => { /* noop */ },
                            _ => {
                                let term = Term::from_match(&caps, &matcher.0);

                                println!("{:?}", &term);

                                terms.push(term);
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
}

#[derive(Debug)]
enum Term {
    Boolean(patpat::Boolean),
    Symbol(patpat::Symbol),
    Define,
    Space,
    Let
}

impl Term {
    fn from_match(caps: &Captures, matcher: &patpat::Kinds) -> Term {
        match matcher {
            patpat::Kinds::Boolean => Term::Boolean(patpat::Boolean {
                state: caps.get(1).unwrap().as_str() == "true"
            }),
            patpat::Kinds::Let => Term::Let,
            patpat::Kinds::Symbol => Term::Symbol(patpat::Symbol {
                name: String::from(caps.get(0).unwrap().as_str())
            }),
            patpat::Kinds::Define => Term::Define,
            _ => Term::Space,
        }
    }
}

mod patpat {
    #[derive(Debug)]
    pub struct Boolean {
        pub state: bool
    }

    #[derive(Debug)]
    pub struct Symbol {
        pub name: String
    }

    #[derive(Debug)]
    pub enum Kinds { // value-less enum of the different kinds of things that there can be
        Boolean,
        Symbol,
        Define,
        Space,
        Let
    }
}

pub const MATCHERS: [(patpat::Kinds, &str); 5] = [
    (patpat::Kinds::Boolean, "^(true|false)"),
    (patpat::Kinds::Let, "^let"),
    (patpat::Kinds::Symbol, "^[a-z_][a-z_\\d]*"),
    (patpat::Kinds::Define, "^:"),
    (patpat::Kinds::Space, "^\\s+")
];

// This should be enough to be able to parse `let is_toast: true`
