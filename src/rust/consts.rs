use crate::{
    ast::{ConstASTNode, ConstItemASTNode, TypeIDASTNode},
    rust_generator::generate_const_value,
    rust_generator::generate_type_id,
    writer::Writer,
};

pub fn generate_const_block(tab: usize, const_node: &ConstASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln_tab(tab, &format!("mod {} {{", const_node.id));

    let mut is_first = true;
    let mut is_value = false;

    for item in &const_node.items {
        match &item {
            ConstItemASTNode::Value { id, type_id, value } => {
                if !is_value && !is_first {
                    writer.writeln("");
                }

                is_value = true;

                let const_value = generate_const_value(value);
                let generated_type_id = generate_type_id(type_id);

                let (type_id, value) = match type_id {
                    TypeIDASTNode::Other { id } => match id.as_str() {
                        "GroupAddress" => (
                            String::from("tech_paws_runtime::GroupAddress"),
                            format!("tech_paws_runtime::GroupAddress({})", const_value),
                        ),
                        "CommandsBufferAddress" => (
                            String::from("tech_paws_runtime::CommandsBufferAddress"),
                            format!("tech_paws_runtime::CommandsBufferAddress({})", const_value),
                        ),
                        _ => (generated_type_id, const_value),
                    },
                    _ => (generated_type_id, const_value),
                };

                writer.writeln_tab(
                    tab + 1,
                    &format!("pub const {}: {} = {};", id, type_id, value),
                );
            }
            ConstItemASTNode::ConstNode { node } => {
                if is_value && !is_first {
                    writer.writeln("");
                }

                writer.write(&generate_const_block(tab + 1, node));
                is_value = false;
            }
        }

        is_first = false;
    }

    writer.writeln_tab(tab, "}");

    writer.show().to_string()
}
