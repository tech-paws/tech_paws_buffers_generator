
#[derive(Debug, Clone)]
pub struct ConstBlockASTNode {
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
    ConstsBlock {
        node: ConstBlockASTNode,
    },
}

#[derive(Debug, Clone)]
pub enum ConstItemValueASTNode {
    Value {
        id: String,
        type_id: TypeIDASTNode,
        value: ConstValueASTNode,
    },
    Struct {
        id: String,
        type_id: TypeIDASTNode,
        parameters: Vec<ConstAssignASTNode>,
    },
    Enum {
        id: String,
        type_id: TypeIDASTNode,
        case_id: String,
        parameters: EnumAssigns,
    },
}

#[derive(Debug, Clone)]
pub enum EnumAssigns {
    Struct {
        parameters: Vec<ConstAssignASTNode>,
    },
    Tuple {
        parameters: Vec<ConstItemValueASTNode>,
    },
}

#[derive(Debug, Clone)]
pub struct ConstAssignASTNode {
    pub id: String,
    pub value: ConstItemValueASTNode,
}