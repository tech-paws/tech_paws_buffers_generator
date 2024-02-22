use crate::lexer::Literal;

#[derive(Debug)]
pub enum ASTNode {
    Enum(EnumASTNode),
    Struct(StructASTNode),
    Fn(FnASTNode),
    Directive(DirectiveASTNode),
    Const(ConstASTNode),
}

#[derive(Debug, Clone)]
pub struct ConstASTNode {
    pub id: String,
    pub items: Vec<ConstItemASTNode>,
}

#[derive(Debug, Clone)]
pub enum ConstItemASTNode {
    Value {
        id: String,
        type_id: TypeIDASTNode,
        value: ConstValueASTNode,
    },
    ConstNode {
        node: ConstASTNode,
    },
}

#[derive(Debug)]
pub enum DirectiveASTNode {
    Value {
        id: String,
        value: ConstValueASTNode,
    },
    Group {
        group_id: String,
        values: Vec<IdValuePair>,
    },
}

#[derive(Debug)]
pub struct EnumASTNode {
    pub id: String,
    pub items: Vec<EnumItemASTNode>,
}

#[derive(Debug)]
pub struct StructASTNode {
    pub id: String,
    pub fields: Vec<StructFieldASTNode>,
    pub emplace_buffers: bool,
    pub into_buffers: bool,
}

#[derive(Debug)]
pub struct FnASTNode {
    pub id: String,
    pub position: u32,
    pub args: Vec<FnArgASTNode>,
    pub return_type_id: Option<TypeIDASTNode>,
    pub is_read: bool,
    pub is_async: bool,
}

#[derive(Debug)]
pub enum EnumItemASTNode {
    Empty {
        position: u32,
        id: String,
    },
    Tuple {
        position: u32,
        id: String,
        values: Vec<TupleFieldASTNode>,
    },
    Struct {
        position: u32,
        id: String,
        fields: Vec<StructFieldASTNode>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstValueASTNode {
    Literal {
        literal: Literal,
        type_id: TypeIDASTNode,
    },
}

#[derive(Debug)]
pub struct IdValuePair {
    pub id: String,
    pub value: ConstValueASTNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeIDASTNode {
    Integer {
        id: String,
        size: usize,
        signed: bool,
    },
    Number {
        id: String,
        size: usize,
    },
    Bool {
        id: String,
    },
    Char {
        id: String,
    },
    Generic {
        id: String,
        generics: Vec<TypeIDASTNode>,
    },
    Other {
        id: String,
    },
}

impl EnumItemASTNode {
    pub fn id(&self) -> &str {
        match self {
            EnumItemASTNode::Empty { position: _, id } => id,
            EnumItemASTNode::Tuple {
                position: _,
                id,
                values: _,
            } => id,
            EnumItemASTNode::Struct {
                position: _,
                id,
                fields: _,
            } => id,
        }
    }

    pub fn position(&self) -> u32 {
        match self {
            EnumItemASTNode::Empty { position, id: _ } => *position,
            EnumItemASTNode::Tuple {
                position,
                id: _,
                values: _,
            } => *position,
            EnumItemASTNode::Struct {
                position,
                id: _,
                fields: _,
            } => *position,
        }
    }
}

impl TypeIDASTNode {
    pub fn u32_type_id() -> Self {
        TypeIDASTNode::Integer {
            id: String::from("u32"),
            size: 8,
            signed: false,
        }
    }
}

#[derive(Debug)]
pub struct TupleFieldASTNode {
    pub position: u32,
    pub type_id: TypeIDASTNode,
}

#[derive(Debug, Clone)]
pub struct StructFieldASTNode {
    pub position: u32,
    pub name: String,
    pub type_id: TypeIDASTNode,
}

#[derive(Debug, Clone)]
pub struct FnArgASTNode {
    pub id: String,
    pub type_id: TypeIDASTNode,
}

pub fn contains_consts_nodes(ast: &[ASTNode]) -> bool {
    for node in ast {
        if let ASTNode::Const(_) = node {
            return true;
        }
    }

    false
}

pub fn find_fn_nodes(ast: &[ASTNode]) -> Vec<&FnASTNode> {
    let mut res = vec![];

    for node in ast {
        if let ASTNode::Fn(node) = node {
            res.push(node);
        }
    }

    res
}

pub fn find_directive_value(ast: &[ASTNode], target_id: &str) -> Option<ConstValueASTNode> {
    for node in ast {
        if let ASTNode::Directive(DirectiveASTNode::Value { id, value }) = node {
            if target_id == id {
                return Some(value.clone());
            }
        }
    }

    None
}

pub fn find_directive_group_value(
    ast: &[ASTNode],
    target_group_id: &str,
    target_id: &str,
) -> Option<ConstValueASTNode> {
    for node in ast {
        if let ASTNode::Directive(DirectiveASTNode::Group { group_id, values }) = node {
            if target_group_id == group_id {
                for value in values {
                    if value.id == target_id {
                        return Some(value.value.clone());
                    }
                }
            }
        }
    }

    None
}

pub fn find_directive_group_values(
    ast: &[ASTNode],
    target_group_id: &str,
    target_id: &str,
) -> Vec<ConstValueASTNode> {
    let mut res = vec![];

    for node in ast {
        if let ASTNode::Directive(DirectiveASTNode::Group { group_id, values }) = node {
            if target_group_id == group_id {
                for value in values {
                    if value.id == target_id {
                        res.push(value.value.clone());
                    }
                }
            }
        }
    }

    res
}
