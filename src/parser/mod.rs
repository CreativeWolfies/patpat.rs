use regex::{Regex};
use std::{
    process
};
mod token;
use super::error::{CompError, CompInfo, Location};

pub fn parse(raw: String) -> TokenTree {
    let lines: Vec<&str> = raw.split('\n').collect();
    let mut token_stack: Vec<TokenTree> = Vec::new();
    token_stack.push(TokenTree::new(token::Kind::TokenTreeRoot));

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

    for (index, line) in lines.iter().enumerate() { // for each line
        //      v-- mut &str
        let mut trimmed_line = line.clone(); // a copy of the line, which progressively gets trimmed
        let mut current_char = 0usize;
        while trimmed_line.len() > 0 {
            let matched = match_next_term(&raw, index, &mut current_char, &mut trimmed_line, &mut token_stack, &regexes);
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

fn match_next_term(raw: &str, line_index: usize, char_index: &mut usize, trimmed_line: &mut &str, token_stack: &mut Vec<TokenTree>, regexes: &Vec<(token::Kind, Regex)>) -> bool {
    let mut res = false; // wether or not a match occured
    for matcher in regexes.iter() {
        if let Some(caps) = matcher.1.captures(trimmed_line) {
            let cap_length = caps.get(0).unwrap().as_str().len();
            *char_index += cap_length;
            *trimmed_line = trimmed_line.split_at(cap_length).1;
            match matcher.0 {
                token::Kind::Space => { /* noop */ },
                token::Kind::Comment => {*trimmed_line = ""; res = true; break;},
                token::Kind::TupleStart => {
                    token_stack.push(TokenTree::new(token::Kind::Tuple));
                },
                token::Kind::BlockStart => {
                    token_stack.push(TokenTree::new(token::Kind::Block));
                },
                token::Kind::TupleEnd => {
                    if let Some(ast) = token_stack.pop() {
                        match ast.kind {
                            token::Kind::Tuple => {},
                            _ => {
                                CompError::new(
                                    101, vec![CompInfo::new(
                                        "Unexpected token TupleEnd ')': not in a tuple",
                                        Location::Char(raw, line_index, *char_index)
                                    )]
                                ).print_and_exit();
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
                },
                token::Kind::BlockEnd => {
                    if let Some(ast) = token_stack.pop() {
                        match ast.kind {
                            token::Kind::Block => {},
                            _ => {
                                // TODO: syntax error
                                eprintln!("Unexpected token BlockEnd '}}': not in a block");
                                process::exit(102);
                            }
                        }
                        if let Some(parent_ast) = token_stack.last_mut() {
                            parent_ast.tokens.push(token::Token::Block(ast));
                        } else {
                            eprintln!("Empty token stack (4)");
                            process::exit(1);
                        }
                    } else {
                        eprintln!("Empty token stack (5)");
                        process::exit(1);
                    }
                },
                token::Kind::StringDelimiter => {
                    let mut iter = trimmed_line.chars();
                    let mut was_backslash = false;
                    let mut length = 0usize;
                    let mut buff = String::new();
                    loop {
                        match iter.next() {
                            Some(current_char) => {
                                length += 1;
                                if was_backslash {
                                    was_backslash = false;
                                    match current_char {
                                        '\\' => buff.push('\\'),
                                        '"' => buff.push('"'),
                                        'n' => buff.push('\n'),
                                        _ => {
                                            eprintln!("Unexpected character following backslash in stinrg literal: {}", current_char);
                                            process::exit(103);
                                        }
                                    }
                                } else {
                                    match current_char {
                                        '\\' => was_backslash = true,
                                        '"' => break,
                                        _ => buff.push(current_char),
                                    }
                                }
                            },
                            None => {
                                eprintln!("Unexpected EOL, expected string end");
                                process::exit(103);
                            }
                        }
                    }
                    *char_index += length;
                    *trimmed_line = trimmed_line.split_at(length).1;
                    if let Some(t) = token_stack.last_mut() {
                        t.tokens.push(token::Token::String(buff));
                    } else {
                        eprintln!("Empty token stack (6)");
                        process::exit(1);
                    }
                },
                _ => {
                    let term = token::Token::from_match(&caps, &matcher.0);
                    println!("{:?}", &term);
                    if let Some(t) = token_stack.last_mut() {
                        t.tokens.push(term);
                    } else {
                        eprintln!("Empty token stack (7)");
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

pub const MATCHERS: [(token::Kind, &str); 21] = [
    (token::Kind::Boolean, "^(true|false)"),
    (token::Kind::Let, "^let"),
    (token::Kind::Struct, "^struct"),
    (token::Kind::Use, "^#use"),
    (token::Kind::Load, "^#load"),
    (token::Kind::Symbol, "^[a-z_][a-z_\\d]*"),
    (token::Kind::Define, "^:"),
    (token::Kind::Space, "^[\\s\\t]+"),
    (token::Kind::Comment, "^//"),
    (token::Kind::Pattern, "^['#]\\w(?:[\\w_\\d]|::)*"),
    (token::Kind::TupleStart, "^\\("),
    (token::Kind::TupleEnd, "^\\)"),
    (token::Kind::Number, "^-?\\d+(?:\\.\\d*)?"),
    (token::Kind::Arrow, "^=>"),
    (token::Kind::Interpretation, "^->"),
    (token::Kind::MemberAccessor, "^\\."),
    (token::Kind::Type, "^<\\s*([!~]?)\\s*([A-Z][\\w_\\d]*|number|bool|string|function)\\s*>"),
    (token::Kind::TypeName, "^[A-Z][\\w_\\d]*"),
    (token::Kind::BlockStart, "^\\{"),
    (token::Kind::BlockEnd, "^\\}"),
    (token::Kind::StringDelimiter, "\""),
];

#[derive(Debug)]
pub struct TokenTree {
    tokens: Vec<token::Token>,
    kind: token::Kind,
}

impl TokenTree {
    fn new(kind: token::Kind) -> TokenTree {
        TokenTree {
            tokens: Vec::new(),
            kind
        }
    }
}
