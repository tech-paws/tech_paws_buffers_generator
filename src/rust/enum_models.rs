use crate::{
    ast::{EnumASTNode, EnumItemASTNode, TupleFieldASTNode},
    rust_generator::{generate_default_const, generate_type_id},
    writer::Writer,
};

use super::struct_models::generate_struct_parameters;

pub fn generate_enum_model(node: &EnumASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln("#[derive(Debug, Clone, PartialEq)]");
    writer.writeln(&format!("pub enum {} {{", node.id));

    for item in node.items.iter() {
        match item {
            EnumItemASTNode::Empty {
                doc_comments: _,
                position: _,
                id,
            } => writer.writeln_tab(1, &format!("{},", id)),
            EnumItemASTNode::Tuple {
                doc_comments: _,
                position: _,
                id,
                values,
            } => {
                writer.writeln_tab(1, &format!("{}(", id));
                writer.write(&generate_tuple_parameters(2, values));
                writer.writeln_tab(1, "),");
            }
            EnumItemASTNode::Struct {
                doc_comments: _,
                position: _,
                id,
                fields,
            } => {
                writer.writeln_tab(1, &format!("{} {{", id));
                writer.write(&generate_struct_parameters(2, false, fields));
                writer.writeln_tab(1, "},");
            }
        }
    }

    writer.writeln("}");

    writer.writeln("");

    // Default
    writer.writeln(&format!("impl Default for {} {{", node.id));
    writer.writeln_tab(1, "fn default() -> Self {");

    let default_item = node.items.first().unwrap();

    match default_item {
        EnumItemASTNode::Empty {
            doc_comments: _,
            position: _,
            id,
        } => writer.writeln_tab(2, &format!("Self::{}", id)),
        EnumItemASTNode::Tuple {
            doc_comments: _,
            position: _,
            id,
            values,
        } => {
            writer.writeln_tab(2, &format!("Self::{}(", id));

            for value in values {
                writer.writeln_tab(3, &format!("{},", generate_default_const(&value.type_id)));
            }

            writer.writeln_tab(2, ")");
        }
        EnumItemASTNode::Struct {
            doc_comments: _,
            position: _,
            id,
            fields,
        } => {
            writer.writeln_tab(2, &format!("Self::{} {{", id));

            for field in fields {
                writer.writeln_tab(
                    3,
                    &format!(
                        "{}: {},",
                        field.name,
                        generate_default_const(&field.type_id)
                    ),
                );
            }

            writer.writeln_tab(2, "}");
        }
    }

    writer.writeln_tab(1, "}");
    writer.writeln("}");

    writer.show().to_string()
}

pub(crate) fn generate_tuple_parameters(tab: usize, params: &[TupleFieldASTNode]) -> String {
    let mut writer = Writer::default();

    for param in params {
        let type_id = generate_type_id(&param.type_id);
        writer.writeln_tab(tab, &format!("{},", type_id));
    }

    writer.show().to_string()
}
