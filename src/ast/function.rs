use super::*;

/// The Function type, corresponds to `TUPLE ARROW BLOCK`
#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub args: Vec<FunctionArg>,
    pub body: AST<'a>,
    pub has_self: bool,
    pub has_lhs: bool,
    pub has_new: bool,
}

impl<'a> Function<'a> {
    pub fn parse(
        one: (Token<'a>, Location<'a>),
        two: (Token<'a>, Location<'a>),
        three: (Token<'a>, Location<'a>),
        is_pattern: bool,
    ) -> Option<Function<'a>> {
        /*! Takes as input three tokens and tries to parse them into a function
         * If these three tokens happen to be a Tuple, an Arrow and a Block, then this function yields a Function.
         * Otherwise it will return None
         */
        match (one.0, two.0, three.0) {
            (Token::Tuple(raw_tuple), Token::Arrow, Token::Block(raw_body)) => {
                let tuple = AST::parse(raw_tuple, ASTKind::ArgTuple);
                let body = AST::parse(raw_body, ASTKind::Block);
                let mut has_self = false;
                let mut has_lhs = false;
                let mut has_new = false;
                let mut args = Vec::<FunctionArg>::new();
                let mut visited = Vec::<(ASTNode, Location)>::new();
                for (raw_arg, location) in tuple.instructions {
                    match &raw_arg {
                        ASTNode::Variable(name) => args.push(FunctionArg {
                            argtype: None,
                            name: name.to_string(),
                        }),
                        ASTNode::TypedVariable(name, argtype) => args.push(FunctionArg {
                            argtype: Some(argtype.clone()),
                            name: name.to_string(),
                        }),
                        ASTNode::PatternCall(name, _args) => {
                            if name == "#self" {
                                if has_self {
                                    error_double_flag(name, visited, location);
                                } else if !is_pattern {
                                    CompError::new(
                                        105,
                                        String::from(
                                            "#self() can only be used as a pattern's argument",
                                        ),
                                        CompLocation::from(&location),
                                    )
                                    .print_and_exit();
                                } else {
                                    has_self = true;
                                }
                            } else if name == "#lhs" {
                                if has_lhs {
                                    error_double_flag(name, visited, location);
                                } else {
                                    has_lhs = true;
                                }
                            } else if name == "#new" {
                                if has_new {
                                    error_double_flag(name, visited, location);
                                } else if !is_pattern {
                                    CompError::new(
                                        105,
                                        String::from(
                                            "#new() can only be used as a pattern's argument",
                                        ),
                                        CompLocation::from(&location),
                                    )
                                    .print_and_exit();
                                } else {
                                    has_new = true;
                                }
                            } else {
                                CompError::new(
                                    12,
                                    String::from("Invalid argument in function definition: unrecognized pattern"),
                                    CompLocation::from(location)
                                    ).print_and_exit();
                            }
                        }
                        _ => {
                            CompError::new(
                                12,
                                String::from("Invalid argument in function definition"),
                                CompLocation::from(location),
                            )
                            .print_and_exit();
                        }
                    }
                    visited.push((raw_arg, location));
                }
                Some(Function {
                    args,
                    body,
                    has_self,
                    has_lhs,
                    has_new,
                })
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionArg {
    pub argtype: Option<Type>,
    pub name: String,
}

fn error_double_flag(name: &str, visited: Vec<(ASTNode<'_>, Location<'_>)>, location: Location<'_>) -> ! {
    let mut err = CompError::new(
        104,
        format!("Duplicate flag {} in pattern declaration", name),
        CompLocation::from(&location),
    );
    for (r, l) in visited.into_iter() {
        if let ASTNode::PatternCall(n, _) = r {
            if n == name {
                err.add_info(CompInfo::new(
                    format!("{} is used here", name),
                    CompLocation::from(l),
                ));
                break;
            }
        }
    }
    err.print_and_exit();
}
