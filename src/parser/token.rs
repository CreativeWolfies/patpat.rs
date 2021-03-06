use crate::error::*;
use crate::Location;
use regex::Captures;
use std::fmt;

// tokens that will end up in the TokenTree
#[derive(Debug, Clone)]
pub enum Token<'a> {
    Boolean(bool),
    Symbol(String),
    VoidSymbol,
    Define,
    Let,
    Struct,
    Use,
    Load,
    Pattern(String),
    Tuple(TokenTree<'a>),
    Block(TokenTree<'a>),
    Number(f64),
    Arrow,
    Operator(Operator),
    Type(Type),
    TypeName(TypeName),
    String(String),
    Separator,
}

impl<'a> Token<'a> {
    pub fn from_match(caps: &Captures, matcher: &Kind, loc: Location) -> Token<'a> {
        match matcher {
            Kind::Boolean => Token::Boolean(caps.get(1).unwrap().as_str() == "true"),
            Kind::Let => Token::Let,
            Kind::Symbol => Token::Symbol(String::from(caps.get(0).unwrap().as_str())),
            Kind::Define => Token::Define,
            Kind::Pattern => Token::Pattern(String::from(caps.get(0).unwrap().as_str())),
            Kind::Number => Token::Number(match caps.get(0).unwrap().as_str().parse::<f64>() {
                Ok(v) => v,
                Err(e) => {
                    CompError::new(
                        6,
                        format!(
                            "Invalid number literal: {} ({})",
                            caps.get(0).unwrap().as_str(),
                            e
                        ),
                        CompLocation::from(loc),
                    )
                    .print_and_exit();
                }
            }),
            Kind::Arrow => Token::Arrow,
            Kind::VoidSymbol => Token::VoidSymbol,
            Kind::TypeName => Token::TypeName(TypeName {
                name: String::from(caps.get(0).unwrap().as_str()),
            }),
            Kind::Type => Token::Type(Type {
                name: String::from(caps.get(2).unwrap().as_str()),
                strictness: match caps.get(1).unwrap().as_str() {
                    "!" => TypeStrictness::Strict,
                    "~" => TypeStrictness::Loose,
                    _ => TypeStrictness::Normal,
                },
            }),
            Kind::Operator => Token::Operator(match caps.get(1).unwrap().as_str() {
                "->" => Operator::Interpretation,
                "==" => Operator::Eq,
                "!=" => Operator::Neq,
                ">" => Operator::Gt,
                ">=" => Operator::Gte,
                "<" => Operator::Lt,
                "<=" => Operator::Lte,
                "!" => Operator::Not,
                "&&" => Operator::And,
                "||" => Operator::Or,
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                "/" => Operator::Div,
                "%" => Operator::Mod,
                "." => Operator::MemberAccessor,
                "~" => Operator::PartialApplication,
                _ => {
                    eprintln!("Unknown operator: {:?}", caps.get(1).unwrap().as_str());
                    std::process::exit(1);
                }
            }),
            Kind::Struct => Token::Struct,
            Kind::Load => Token::Load,
            Kind::Use => Token::Use,
            Kind::Separator => Token::Separator,
            _ => {
                eprintln!("Unknown token kind: {:?}", matcher);
                std::process::exit(4);
            }
        }
    }
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub enum Kind {
    Boolean,
    Symbol,
    VoidSymbol,
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
    Operator,
    Type,
    TypeName,
    BlockStart,
    BlockEnd,
    Block,
    StringDelimiter,
    Separator,
}

#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub strictness: TypeStrictness,
}

#[derive(Debug, Clone)]
pub enum TypeStrictness {
    Loose,
    Normal,
    Strict,
}

#[derive(Clone, PartialEq)]
pub struct TypeName {
    pub name: String,
}

impl fmt::Debug for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TypeName({})", self.name)
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Interpretation,
    MemberAccessor,
    PartialApplication,
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
    Not,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Operator {
    pub fn is_unary(&self) -> bool {
        match self {
            Operator::Not => true,
            _ => false,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Operator::Interpretation => "->",
            Operator::MemberAccessor => ".",
            Operator::PartialApplication => "~",
            Operator::Gt => ">",
            Operator::Gte => ">=",
            Operator::Lt => "<",
            Operator::Lte => "<=",
            Operator::Eq => "==",
            Operator::Neq => "!=",
            Operator::Not => "!",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
        })
    }
}
