use convert_case::{Case, Casing};

use crate::{
    dart_generator::{
        generate_read, generate_read_emplace, generate_read_skip, generate_read_skip_emplace,
        generate_write, generate_write_emplace,
    },
    parser::{StructASTNode, TypeIDASTNode},
    writer::Writer,
};

pub fn generate_struct_emplace_buffers(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln(&format!(
        "class {}EmplaceToBuffers implements EmplaceToBuffers<{}> {{",
        node.id, node.id
    ));

    writer.writeln_tab(1, &format!("const {}EmplaceToBuffers();", node.id));
    writer.writeln("");

    writer.writeln(&generate_struct_emplace_buffers_read(node));
    writer.writeln(&generate_struct_emplace_buffers_write(node));
    writer.write(&generate_struct_emplace_buffers_skip(node));

    writer.writeln("}");

    writer.show().to_string()
}

pub fn generate_struct_emplace_buffers_read(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "@override");
    writer.writeln_tab(
        1,
        &format!("void read(BytesReader reader, {} model) {{", node.id),
    );

    for field in node.fields.iter() {
        writer.writeln_tab(
            2,
            &generate_read_emplace(
                &field.type_id,
                &format!("model.{}", field.name.to_case(Case::Camel),),
            ),
        );
    }

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

pub fn generate_struct_emplace_buffers_write(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "@override");
    writer.writeln_tab(
        1,
        &format!("void write(BytesWriter writer, {} model) {{", node.id),
    );

    for field in node.fields.iter() {
        writer.writeln_tab(
            2,
            &generate_write_emplace(
                &field.type_id,
                &format!("model.{}", field.name.to_case(Case::Camel)),
            ),
        );
    }

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

pub fn generate_struct_emplace_buffers_skip(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "@override");
    writer.writeln_tab(1, "void skip(BytesReader reader, int count) {");

    let mut break_line = false;

    for field in node.fields.iter() {
        if let TypeIDASTNode::Other { id: _ } = field.type_id {
            writer.writeln_tab(2, &generate_read_skip_emplace(&field.type_id));
            break_line = true;
        }
    }

    if break_line {
        writer.writeln("");
    }

    writer.writeln_tab(2, "for (int i = 0; i < count; i += 1) {");

    for field in node.fields.iter() {
        if let TypeIDASTNode::Other { id: _ } = field.type_id {
            // Do nothing
        } else {
            writer.writeln_tab(3, &generate_read_skip_emplace(&field.type_id));
        }
    }

    writer.writeln_tab(2, "}");
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}
