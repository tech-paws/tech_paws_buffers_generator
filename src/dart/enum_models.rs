use crate::parser::{EnumASTNode, EnumItemASTNode, StructASTNode, StructFieldASTNode};

pub fn create_enum_item_struct_ast_node(
    node: &EnumASTNode,
    item_node: &EnumItemASTNode,
) -> StructASTNode {
    match item_node {
        EnumItemASTNode::Empty { position: _, id } => {
            let class_id = format!("{}{}", node.id, id);

            StructASTNode {
                id: class_id,
                fields: vec![],
            }
        }
        EnumItemASTNode::Tuple {
            position: _,
            id,
            values,
        } => {
            let class_id = format!("{}{}", node.id, id);
            let mut args_struct_fields = vec![];

            for (i, value) in values.iter().enumerate() {
                args_struct_fields.push(StructFieldASTNode {
                    position: i as u32,
                    name: format!("v{}", i),
                    type_id: value.type_id.clone(),
                });
            }

            StructASTNode {
                id: class_id,
                fields: args_struct_fields,
            }
        }
        EnumItemASTNode::Struct {
            position: _,
            id,
            fields,
        } => {
            let class_id = format!("{}{}", node.id, id);
            StructASTNode {
                id: class_id,
                fields: fields.to_vec(),
            }
        }
    }
}
