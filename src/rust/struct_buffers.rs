use crate::{
    ast::StructASTNode,
    rust_generator::{generate_read, generate_write},
    writer::Writer,
};

pub fn generate_struct_buffers(node: &StructASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!("impl IntoVMBuffers for {} {{", node.id));

    writer.write(&generate_struct_buffers_read_from_buffers(node));
    writer.writeln("");
    writer.write(&generate_struct_buffers_write_to_buffers(node));
    writer.writeln("");
    writer.write(&generate_struct_buffers_skip(node));

    writer.writeln("}");

    writer.show().to_string()
}

pub fn generate_struct_buffers_read_from_buffers(node: &StructASTNode) -> String {
    let mut writer = Writer::default();

    if node.fields.is_empty() {
        writer.writeln_tab(1, "fn read_from_buffers(_: &mut BytesReader) -> Self {");
        writer.writeln_tab(2, &node.id);
        writer.writeln_tab(1, "}");
    } else {
        writer.writeln_tab(
            1,
            "fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {",
        );

        writer.writeln_tab(2, "Self {");

        for field in node.fields.iter() {
            writer.writeln_tab(
                3,
                &format!("{}: {},", field.name, generate_read(&field.type_id)),
            );
        }

        writer.writeln_tab(2, "}");
        writer.writeln_tab(1, "}");
    }

    writer.show().to_string()
}

pub fn generate_struct_buffers_write_to_buffers(node: &StructASTNode) -> String {
    let mut writer = Writer::default();

    if node.fields.is_empty() {
        writer.writeln_tab(1, "fn write_to_buffers(&self, _: &mut BytesWriter) {}");
    } else {
        writer.writeln_tab(
            1,
            "fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {",
        );

        for field in node.fields.iter() {
            writer.writeln_tab(
                2,
                &generate_write(&field.type_id, &format!("self.{}", field.name), false),
            );
        }

        writer.writeln_tab(1, "}");
    }

    writer.show().to_string()
}

pub fn generate_struct_buffers_skip(node: &StructASTNode) -> String {
    let mut writer = Writer::default();

    if node.fields.is_empty() {
        writer.writeln_tab(1, "fn skip_in_buffers(_: &mut BytesReader, _: u64) {}");
    } else {
        writer.writeln_tab(
            1,
            "fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {",
        );
        writer.writeln_tab(2, "for _ in 0..count {");

        for field in node.fields.iter() {
            writer.writeln_tab(3, &format!("{};", generate_read(&field.type_id)));
        }

        writer.writeln_tab(2, "}");
        writer.writeln_tab(1, "}");
    }

    writer.show().to_string()
}
