use super::TokenTree;
use regex::Captures;

// tokens that will end up in the TokenTree
#[derive(Debug)]
pub enum Token {
    Boolean(Boolean),
    Symbol(Symbol),
    Define,
    Let,
    Struct,
    Use,
    Load,
    Pattern(Pattern),
    Tuple(TokenTree),
    Block(TokenTree),
    Number(Number),
    Arrow,
    Interpretation,
    MemberAccessor,
    Type(Type),
    TypeName(TypeName),
    String(String),
}

impl Token {
    pub fn from_match(caps: &Captures, matcher: &Kind) -> Token {
        match matcher {
            Kind::Boolean => Token::Boolean(Boolean {
                state: caps.get(1).unwrap().as_str() == "true"
            }),
            Kind::Let => Token::Let,
            Kind::Symbol => Token::Symbol(Symbol {
                name: String::from(caps.get(0).unwrap().as_str())
            }),
            Kind::Define => Token::Define,
            Kind::Pattern => Token::Pattern(Pattern {
                name: String::from(caps.get(0).unwrap().as_str())
            }),
            Kind::Number => Token::Number(Number {
                value: match caps.get(0).unwrap().as_str().parse::<f64>() {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Invalid number: {} ({})", caps.get(0).unwrap().as_str(), e);
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
pub struct Boolean {
    pub state: bool
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String
}

#[derive(Debug)]
pub struct Pattern {
    pub name: String
}

#[derive(Debug)]
pub struct Number {
    pub value: f64
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub strictness: TypeStrictness
}

#[derive(Debug)]
pub enum TypeStrictness {
    Loose,
    Normal,
    Strict
}

#[derive(Debug)]
pub struct TypeName {
    pub name: String
}
