use regex::Captures;
use crate::{Location};

// tokens that will end up in the TokenTree
#[derive(Debug)]
#[derive(Clone)]
pub enum Token<'a> {
    Boolean(Boolean),
    Symbol(Symbol),
    Define,
    Let,
    Struct,
    Use,
    Load,
    Pattern(String),
    Tuple(TokenTree<'a>),
    Block(TokenTree<'a>),
    Number(Number),
    Arrow,
    Interpretation,
    MemberAccessor,
    Type(Type),
    TypeName(TypeName),
    String(String),
}

impl<'a> Token<'a> {
    pub fn from_match(caps: &Captures, matcher: &Kind) -> Token<'a> {
        match matcher {
            Kind::Boolean => Token::Boolean(Boolean {
                state: caps.get(1).unwrap().as_str() == "true"
            }),
            Kind::Let => Token::Let,
            Kind::Symbol => Token::Symbol(Symbol {
                name: String::from(caps.get(0).unwrap().as_str())
            }),
            Kind::Define => Token::Define,
            Kind::Pattern => Token::Pattern(String::from(caps.get(0).unwrap().as_str())),
            Kind::Number => Token::Number(Number {
                value: match caps.get(0).unwrap().as_str().parse::<f64>() {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Invalid number literal: {} ({})", caps.get(0).unwrap().as_str(), e);
                        std::process::exit(6);
                    }
                }
            }),
            Kind::Arrow => Token::Arrow,
            Kind::MemberAccessor => Token::MemberAccessor,
            Kind::TypeName => Token::TypeName(TypeName {
                name: String::from(caps.get(0).unwrap().as_str())
            }),
            Kind::Type => Token::Type(Type {
                name: String::from(caps.get(2).unwrap().as_str()),
                strictness: match caps.get(1).unwrap().as_str() {
                    "!" => TypeStrictness::Strict,
                    "~" => TypeStrictness::Loose,
                    _ => TypeStrictness::Normal,
                }
            }),
            Kind::Interpretation => Token::Interpretation,
            Kind::Struct => Token::Struct,
            Kind::Load => Token::Load,
            Kind::Use => Token::Use,
            _ => {
                eprintln!("Unknown token kind: {:?}", matcher);
                std::process::exit(4);
            },
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TokenTree<'a> {
    pub tokens: Vec<(Token<'a>, Location<'a>)>,
    pub kind: Kind,
    pub start_loc: Location<'a>,
}

impl<'a> TokenTree<'a> {
    pub fn new(kind: Kind, start_loc: Location<'a>) -> TokenTree<'a> {
        TokenTree {
            tokens: Vec::new(),
            kind,
            start_loc,
        }
    }
}

// value-less tokens
#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub enum Kind {
    Boolean,
    Symbol,
    Define,
    Space,
    Let,
    Struct,
    Use,
    Load,
    Comment,
    Pattern,
    TupleStart,
    TupleEnd,
    Tuple,
    TokenTreeRoot,
    Number,
    Arrow,
    Interpretation,
    MemberAccessor,
    Type,
    TypeName,
    BlockStart,
    BlockEnd,
    Block,
    StringDelimiter,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Boolean {
    pub state: bool
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Symbol {
    pub name: String
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Number {
    pub value: f64
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Type {
    pub name: String,
    pub strictness: TypeStrictness
}

#[derive(Debug)]
#[derive(Clone)]
pub enum TypeStrictness {
    Loose,
    Normal,
    Strict
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TypeName {
    pub name: String
}
