use regex::{Regex};
use std::{
    process
};

pub mod token;
pub mod construct;
use super::error::{CompError, CompLocation};
use super::ast::{AST, ASTKind};
use token::{TokenTree, Token};
use crate::{SrcFile, Location};

pub fn parse<'a>(file: &'a SrcFile) -> TokenTree<'a> {
    let raw = &file.contents;
    let lines: Vec<&str> = raw.lines().collect();
    let mut token_stack: Vec<TokenTree> = Vec::new();
    token_stack.push(TokenTree::new(
        token::Kind::TokenTreeRoot,
        Location::start(file)
    ));

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
            let matched = match_next_term(file, index, &mut current_char, &mut trimmed_line, &mut token_stack, &regexes);
            if !matched {
                CompError::new(
                    3,
                    String::from("Unrecognized term"),
                    CompLocation::Char(raw, index, current_char)
                ).print_and_exit();
            }
        }
    }

    if token_stack.len() > 1 {
        CompError::new(
            5,
            String::from("Unexpected EOF; did you forget a closing parenthesis?"),
            CompLocation::Char(raw, lines.len() - 1, lines[lines.len() - 1].len())
        ).print_and_exit();
    }

    match token_stack.pop() {
        Some(t) => {
            t
        },
        None => {
            eprintln!("Empty token stack (1)");
            process::exit(1);
        }
    }
}

pub fn construct<'a>(parsed: TokenTree<'a>) -> AST<'a> {
    let ast = AST::parse(parsed, ASTKind::File);
    ast
}

fn match_next_term<'a>(
    file: &'a SrcFile,
    line_index: usize,
    char_index: &mut usize,
    trimmed_line: &mut &str,
    token_stack: &mut Vec<TokenTree<'a>>,
    regexes: &Vec<(token::Kind, Regex)>
) -> bool {
    /*! Tries to match a term; it does this by trying to match with each term `MATCHER`.
    * String, blocks are handled by this. Block and tuple nesting are done using a "token stack", which grows as the nesting goes on.
    * This way, the algorithm can keep a linear approach to the code parsing.
    * <!-- (thanks to @PhirosWolf for having helped me with this) -->
    */
    let raw: &str = &file.contents;
    let mut res = false; // wether or not a match occured
    for matcher in regexes.iter() {
        if let Some(caps) = matcher.1.captures(trimmed_line) {
            let cap_length = caps.get(0).unwrap().as_str().len();
            let old_char_index = *char_index;
            *char_index += cap_length;
            *trimmed_line = trimmed_line.split_at(cap_length).1;
            match matcher.0 {
                token::Kind::Space => { /* noop */ },
                token::Kind::Comment => {*trimmed_line = ""; res = true; break;},
                token::Kind::TupleStart => {
                    token_stack.push(TokenTree::new(
                        token::Kind::Tuple,
                        Location::new(file, line_index, old_char_index)
                    ));
                },
                token::Kind::BlockStart => {
                    token_stack.push(TokenTree::new(
                        token::Kind::Block,
                        Location::new(file, line_index, old_char_index)
                    ));
                },
                token::Kind::TupleEnd => {
                    if let Some(ast) = token_stack.pop() {
                        match ast.kind {
                            token::Kind::Tuple => {},
                            _ => {
                                CompError::new(
                                    101,
                                    String::from("Unexpected token TupleEnd ')': not in a tuple"),
                                    CompLocation::Char(raw, line_index, *char_index - 1)
                                ).print_and_exit();
                            }
                        }
                        if let Some(parent_ast) = token_stack.last_mut() {
                            parent_ast.tokens.push((
                                token::Token::Tuple(ast),
                                Location::new(file, line_index, old_char_index)
                            ));
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
                                CompError::new(
                                    102,
                                    String::from("Unexpected token BlockEnd '}': not in a block"),
                                    CompLocation::Char(raw, line_index, *char_index - 1)
                                ).print_and_exit();
                            }
                        }
                        if let Some(parent_ast) = token_stack.last_mut() {
                            parent_ast.tokens.push((
                                token::Token::Block(ast),
                                Location::new(file, line_index, old_char_index)
                            ));
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
                                            CompError::new(
                                                103,
                                                format!("Unexpected character following backslash in string literal: {}", current_char),
                                                CompLocation::Char(raw, line_index, *char_index + length - 1)
                                            ).print_and_exit();
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
                                CompError::new(
                                    103,
                                    String::from("Unexpected EOL in string literal"),
                                    CompLocation::Char(raw, line_index, *char_index + length - 1)
                                ).print_and_exit();
                            }
                        }
                    }
                    *char_index += length;
                    *trimmed_line = trimmed_line.split_at(length).1;
                    if let Some(t) = token_stack.last_mut() {
                        t.tokens.push((
                            token::Token::String(buff),
                            Location::new(file, line_index, old_char_index)
                        ));
                    } else {
                        eprintln!("Empty token stack (6)");
                        process::exit(1);
                    }
                },
                _ => {
                    let term = token::Token::from_match(
                        &caps, &matcher.0,
                        Location::new(file, line_index, old_char_index)
                    );
                    if let Some(t) = token_stack.last_mut() {
                        t.tokens.push((
                            term,
                            Location::new(file, line_index, old_char_index)
                        ));
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
    (token::Kind::Number, "^-?\\d+(?:\\.\\d*)?[\\w.]*"), // intentionally loose
    (token::Kind::Arrow, "^=>"),
    (token::Kind::Operator, "^(->|\\.|>=|<=|==|!=|&&|\\|\\||[!+\\-/*<>%])"),
    (token::Kind::Type, "^<\\s*([!~]?)\\s*([A-Z][\\w_\\d]*|number|bool|string|function)\\s*>"),
    (token::Kind::TypeName, "^[A-Z][\\w_\\d]*"),
    (token::Kind::BlockStart, "^\\{"),
    (token::Kind::BlockEnd, "^\\}"),
    (token::Kind::StringDelimiter, "\""),
    (token::Kind::Separator, "^,"),
];
