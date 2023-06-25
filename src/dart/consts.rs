use convert_case::{Case, Casing};

use crate::{
    ast::{ASTNode, ConstASTNode, ConstItemASTNode, TypeIDASTNode},
    dart_generator::generate_const_value,
    dart_generator::generate_type_id,
    writer::Writer,
};

pub fn generate_consts(ast: &[ASTNode]) -> String {
    let mut writer = Writer::new(2);

    for node in ast {
        if let ASTNode::Const(node) = node {
            writer.write(&generate_const_class_decls(node));
        }
    }

    for node in ast {
        if let ASTNode::Const(node) = node {
            writer.writeln("");
            writer.write(&generate_const_class_impl(node));
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_const_class_decls(node: &ConstASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(
        0,
        &format!(
            "const {} = _{}();",
            node.id.to_case(Case::Camel),
            node.id.to_case(Case::Pascal)
        ),
    );

    for item in &node.items {
        if let ConstItemASTNode::ConstNode { node } = item {
            writer.write(&generate_const_class_decls(node))
        }
    }

    writer.show().to_string()
}

pub fn generate_const_class_impl(node: &ConstASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln(&format!("class _{} {{", node.id.to_case(Case::Pascal)));
    writer.writeln_tab(1, &format!("const _{}();", node.id.to_case(Case::Pascal)));

    let mut added_space = false;

    for item in &node.items {
        if !added_space {
            added_space = true;
            writer.writeln("");
        }

        match item {
            ConstItemASTNode::Value { id, type_id, value } => {
                let type_id = match type_id {
                    TypeIDASTNode::Other { id } => match id.as_str() {
                        "GroupAddress" => String::from("int"),
                        "CommandsBufferAddress" => String::from("int"),
                        _ => generate_type_id(type_id),
                    },
                    _ => generate_type_id(type_id),
                };

                writer.writeln_tab(
                    1,
                    &format!(
                        "final {} {} = {};",
                        type_id,
                        id.to_case(Case::Camel),
                        generate_const_value(value),
                    ),
                );
            }
            ConstItemASTNode::ConstNode { node } => writer.writeln_tab(
                1,
                &format!(
                    "final {} = const _{}();",
                    node.id.to_case(Case::Camel),
                    node.id.to_case(Case::Pascal),
                ),
            ),
        }
    }

    writer.writeln("}");

    for item in &node.items {
        if let ConstItemASTNode::ConstNode { node } = item {
            writer.writeln("");
            writer.write(&generate_const_class_impl(node))
        }
    }

    writer.show().to_string()
}
