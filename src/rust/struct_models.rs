use crate::{
    ast::{self, StructASTNode, StructFieldASTNode},
    rust_generator::{generate_default_const, generate_type_id},
    writer::Writer,
};

pub fn generate_struct_model(node: &StructASTNode, generate_default: bool) -> String {
    let mut writer = Writer::default();
    let mut derives = vec![
        "Debug".to_string(),
        "Clone".to_string(),
        "PartialEq".to_string(),
    ];

    for directive in &node.directives {
        match directive {
            ast::DirectiveASTNode::Value { .. } => {
                continue;
            }
            ast::DirectiveASTNode::Group { group_id, values } => {
                if group_id != "derive" {
                    continue;
                }

                for value in values {
                    if !derives.contains(&value.id) {
                        derives.push(value.id.clone());
                    }
                }
            }
        }
    }

    for comment in &node.doc_comments {
        writer.writeln(&format!("///{}", comment));
    }

    writer.writeln(&format!("#[derive({})]", derives.join(", ")));

    if node.fields.is_empty() {
        writer.writeln(&format!("pub struct {};", node.id));

        if generate_default {
            writer.writeln("");
            writer.writeln(&format!("impl Default for {} {{", node.id));
            writer.writeln_tab(1, "fn default() -> Self {");
            writer.writeln_tab(2, "Self");
            writer.writeln_tab(1, "}");
            writer.writeln("}");
        }
    } else {
        writer.writeln(&format!("pub struct {} {{", node.id));
        writer.write(&generate_struct_parameters(1, true, &node.fields));
        writer.writeln("}");

        if generate_default {
            writer.writeln("");
            writer.writeln(&generate_struct_default(node));
        }
    }

    writer.show().to_string()
}

fn generate_struct_default(node: &StructASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!("impl Default for {} {{", node.id));
    writer.writeln_tab(1, "fn default() -> Self {");
    writer.writeln_tab(2, "Self {");

    for field in node.fields.iter() {
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
    writer.writeln_tab(1, "}");
    writer.write("}");

    writer.show().to_string()
}

pub fn generate_struct_parameters(
    tab: usize,
    is_pub: bool,
    params: &[StructFieldASTNode],
) -> String {
    let mut writer = Writer::default();
    let mut has_doc_comments = false;

    for param in params {
        if !param.doc_comments.is_empty() {
            has_doc_comments = true;
            break;
        }
    }

    for (idx, param) in params.iter().enumerate() {
        let type_id = generate_type_id(&param.type_id);

        for comment in &param.doc_comments {
            writer.writeln_tab(tab, &format!("///{}", comment));
        }

        if is_pub {
            writer.writeln_tab(tab, &format!("pub {}: {},", param.name, type_id));
        } else {
            writer.writeln_tab(tab, &format!("{}: {},", param.name, type_id));
        }

        if has_doc_comments && idx < params.len() - 1 {
            writer.new_line();
        }
    }

    writer.show().to_string()
}
