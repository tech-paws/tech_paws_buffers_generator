use convert_case::{Case, Casing};

use crate::{
    dart_generator::{generate_read, generate_write},
    parser::StructASTNode,
    writer::Writer,
};

pub fn generate_struct_into_buffers(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln(&format!(
        "class {}IntoToBuffers implements IntoToBuffers<{}> {{",
        node.id, node.id
    ));

    writer.writeln_tab(1, &format!("const {}IntoToBuffers()", node.id));
    writer.writeln("");

    writer.writeln(&generate_struct_into_buffers_read(node));
    writer.writeln(&generate_struct_into_buffers_write(node));
    writer.write(&generate_struct_into_buffers_skip(node));

    writer.write("}");

    writer.show().to_string()
}

pub fn generate_struct_into_buffers_read(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "@override");
    writer.writeln_tab(1, &format!("{} read(BytesReader reader) {{", node.id));

    for field in node.fields.iter() {
        writer.writeln_tab(
            2,
            &format!(
                "final {} = {};",
                field.name.to_case(Case::Camel),
                &generate_read(&field.type_id)
            ),
        );
    }

    writer.writeln("");

    writer.writeln_tab(2, &format!("return {}(", node.id));

    for field in node.fields.iter() {
        writer.writeln_tab(
            3,
            &format!(
                "{}: {};",
                field.name.to_case(Case::Camel),
                field.name.to_case(Case::Camel)
            ),
        );
    }

    writer.writeln_tab(2, ");");
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

pub fn generate_struct_into_buffers_write(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "@override");
    writer.writeln_tab(
        1,
        &format!("void write(BytesWriter writer, {} model) {{", node.id),
    );

    for field in node.fields.iter() {
        writer.writeln_tab(
            2,
            &generate_write(
                &field.type_id,
                &format!("model.{}", field.name.to_case(Case::Camel)),
            ),
        );
    }

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

pub fn generate_struct_into_buffers_skip(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "@override");
    writer.writeln_tab(1, "void skip(BytesReader reader, int count) {");

    writer.writeln_tab(2, "for (int i = 0; i < count; i += 1) {");

    for field in node.fields.iter() {
        writer.writeln_tab(3, &format!("{};", &generate_read(&field.type_id)));
    }

    writer.writeln_tab(2, "}");
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}
