use super::*;

/// A node in an AST
#[derive(Debug, Clone)]
pub enum ASTNode<'a> {
    Function(Function<'a>),
    PatternDecl(Pattern<'a>),
    PatternCall(String, AST<'a>), // name, tuple
    MethodCall(String, AST<'a>),
    Member(String),
    Pattern(String),
    Variable(String),
    TypedVariable(String, Type),
    TypeName(token::TypeName),
    VoidSymbol,
    VariableDecl(String),
    VariableInit(String, Box<ASTNode<'a>>),
    VariableDef(String, Box<ASTNode<'a>>),
    ComplexDef(Expression<'a>, DefineMember<'a>, Box<ASTNode<'a>>),
    Boolean(bool),
    Number(f64),
    String(String),
    Expression(Expression<'a>),
    Tuple(AST<'a>, bool), // body, is_partial
    Block(AST<'a>),
    Interpretation(token::TypeName, token::TypeName, AST<'a>), // from, to, body
    Struct(token::TypeName, AST<'a>),                          // name, body
    Nil,
}

impl<'a> ASTNode<'a> {
    pub fn is_valid_expr_term(&self) -> bool {
        match self {
            ASTNode::Function(_)
            | ASTNode::Pattern(_)
            | ASTNode::PatternCall(_, _)
            | ASTNode::MethodCall(_, _)
            | ASTNode::Variable(_)
            | ASTNode::Member(_)
            | ASTNode::Boolean(_)
            | ASTNode::Number(_)
            | ASTNode::String(_)
            | ASTNode::Tuple(_, _)
            | ASTNode::Block(_)
            | ASTNode::TypeName(_)
            | ASTNode::Nil
            | ASTNode::VariableDef(_, _)
            | ASTNode::ComplexDef(_, _, _)
            | ASTNode::VoidSymbol
            | ASTNode::Expression(_) => true,
            _ => false,
        }
    }

    pub fn is_valid_tuple_term(&self) -> bool {
        return self.is_valid_expr_term();
    }

    pub fn is_valid_argtuple_term(&self) -> bool {
        match self {
            ASTNode::Variable(_)
            | ASTNode::TypedVariable(_, _)
            | ASTNode::PatternCall(_, _)
            | ASTNode::VoidSymbol
            | ASTNode::Expression(_) => true,
            _ => false,
        }
    }

    pub fn is_valid_block_term(&self) -> bool {
        if self.is_valid_expr_term() {
            return true;
        }
        match self {
            ASTNode::PatternDecl(_)
            | ASTNode::Interpretation(_, _, _)
            | ASTNode::VariableDecl(_)
            | ASTNode::VariableInit(_, _) => true,
            _ => false,
        }
    }

    pub fn is_valid_file_term(&self) -> bool {
        if self.is_valid_block_term() {
            return true;
        }
        match self {
            ASTNode::Struct(_, _) => true,
            _ => false,
        }
    }

    pub fn is_valid_struct_term(&self) -> bool {
        match self {
            ASTNode::PatternDecl(_) | ASTNode::VariableDecl(_) | ASTNode::VariableInit(_, _) => {
                true
            }
            _ => false,
        }
    }
}
