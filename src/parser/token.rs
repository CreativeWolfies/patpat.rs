use super::AST;
use regex::Captures;

// tokens that will end up in the AST
#[derive(Debug)]
pub enum Token {
    Boolean(Boolean),
    Symbol(Symbol),
    Define,
    Let,
    Pattern(Pattern),
    Tuple(AST),
    Number(Number),
    Arrow,
    MemberAccessor,
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
    Comment,
    Pattern,
    TupleStart,
    TupleEnd,
    Tuple,
    ASTRoot,
    Number,
    Arrow,
    MemberAccessor,
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
