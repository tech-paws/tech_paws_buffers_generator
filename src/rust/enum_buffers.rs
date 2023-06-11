use crate::{ast::{EnumASTNode, TypeIDASTNode, EnumItemASTNode}, writer::Writer, rust_generator::{generate_read, generate_write}};

pub fn generate_enum_buffers(node: &EnumASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!("impl IntoVMBuffers for {} {{", node.id));
    writer.write(&generate_enum_buffers_read_from_buffers(node));
    writer.writeln("");
    writer.write(&generate_enum_buffers_write_to_buffers(node));
    writer.writeln("");
    writer.write(&generate_enum_buffers_skip(node));
    writer.writeln("}");

    writer.show().to_string()
}

pub fn generate_enum_buffers_read_from_buffers(node: &EnumASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln_tab(
        1,
        "fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {",
    );

    writer.writeln_tab(
        2,
        &format!(
            "let value = {};",
            generate_read(&TypeIDASTNode::u32_type_id())
        ),
    );
    writer.writeln("");
    writer.writeln_tab(2, "match value {");

    for item in node.items.iter() {
        match item {
            EnumItemASTNode::Empty { position, id } => {
                writer.writeln_tab(3, &format!("{} => return {}::{},", position, node.id, id));
            }
            EnumItemASTNode::Tuple {
                position,
                id,
                values,
            } => {
                writer.writeln_tab(3, &format!("{} => return {}::{}(", position, node.id, id));

                for value in values {
                    writer.writeln_tab(4, &format!("{},", &generate_read(&value.type_id)));
                }

                writer.writeln_tab(3, "),");
            }
            EnumItemASTNode::Struct {
                position,
                id,
                fields,
            } => {
                writer.writeln_tab(3, &format!("{} => return {}::{} {{", position, node.id, id));

                for field in fields {
                    writer.writeln_tab(
                        4,
                        &format!("{}: {},", field.name, &generate_read(&field.type_id)),
                    );
                }

                writer.writeln_tab(3, "},");
            }
        }
    }

    writer.writeln_tab(3, "_ => panic!(\"Unsupported enum value: {}\", value),");
    writer.writeln_tab(2, "}");
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

pub fn generate_enum_buffers_write_to_buffers(node: &EnumASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln_tab(
        1,
        "fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {",
    );

    writer.writeln_tab(2, "match self {");

    for item in node.items.iter() {
        match item {
            EnumItemASTNode::Empty { position, id } => {
                writer.writeln_tab(3, &format!("{}::{} => {{", node.id, id));
                writer.writeln_tab(
                    4,
                    &generate_write(&TypeIDASTNode::u32_type_id(), &position.to_string(), false),
                );
                writer.writeln_tab(3, "},");
            }
            EnumItemASTNode::Tuple {
                position,
                id,
                values,
            } => {
                writer.writeln_tab(3, &format!("{}::{}(", node.id, id));

                for (i, _) in values.iter().enumerate() {
                    writer.writeln_tab(4, &format!("v{},", i));
                }

                writer.writeln_tab(3, ") => {");
                writer.writeln_tab(
                    4,
                    &generate_write(&TypeIDASTNode::u32_type_id(), &position.to_string(), false),
                );

                for (i, value) in values.iter().enumerate() {
                    writer
                        .writeln_tab(4, &generate_write(&value.type_id, &format!("v{}", i), true));
                }

                writer.writeln_tab(3, "},");
            }
            EnumItemASTNode::Struct {
                position,
                id,
                fields,
            } => {
                writer.writeln_tab(3, &format!("{}::{} {{", node.id, id));

                for field in fields {
                    writer.writeln_tab(4, &format!("{},", field.name));
                }

                writer.writeln_tab(3, "} => {");
                writer.writeln_tab(
                    4,
                    &generate_write(&TypeIDASTNode::u32_type_id(), &position.to_string(), false),
                );

                for field in fields {
                    writer.writeln_tab(4, &generate_write(&field.type_id, &field.name, true));
                }

                writer.writeln_tab(3, "},");
            }
        }
    }

    writer.writeln_tab(2, "}");
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

pub fn generate_enum_buffers_skip(node: &EnumASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln_tab(
        1,
        "fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {",
    );

    writer.writeln_tab(2, "for _ in 0..count {");

    writer.writeln_tab(
        3,
        &format!(
            "let value = {};",
            generate_read(&TypeIDASTNode::u32_type_id())
        ),
    );
    writer.writeln("");
    writer.writeln_tab(3, "match value {");

    for item in node.items.iter() {
        match item {
            EnumItemASTNode::Empty { position, id: _ } => {
                writer.writeln_tab(4, &format!("{} => (),", position));
            }
            EnumItemASTNode::Tuple {
                position,
                id: _,
                values,
            } => {
                writer.writeln_tab(4, &format!("{} => {{", position));

                for value in values {
                    writer.writeln_tab(5, &format!("{};", &generate_read(&value.type_id)));
                }

                writer.writeln_tab(4, "},");
            }
            EnumItemASTNode::Struct {
                position,
                id: _,
                fields,
            } => {
                writer.writeln_tab(4, &format!("{} => {{", position));

                for field in fields {
                    writer.writeln_tab(5, &format!("{};", &generate_read(&field.type_id)));
                }

                writer.writeln_tab(4, "},");
            }
        }
    }

    writer.writeln_tab(4, "_ => panic!(\"Unsupported enum value: {}\", value),");
    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "}");

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}
