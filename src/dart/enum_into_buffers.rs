use convert_case::{Case, Casing};

use crate::ast::*;
use crate::writer::Writer;

pub fn generate_enum_into_buffers(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln(&format!(
        "class {}IntoBuffers implements IntoBuffers<{}> {{",
        node.id, node.id
    ));

    writer.writeln_tab(1, &format!("const {}IntoBuffers();", node.id));
    writer.writeln("");
    writer.writeln(&generate_read(node));
    writer.writeln(&generate_write(node));
    writer.write(&generate_skip(node));
    writer.writeln("}");

    writer.show().to_string()
}

fn generate_read(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);
    writer.writeln_tab(1, "@override");
    writer.writeln_tab(1, &format!("{} read(BytesReader reader) {{", node.id));

    writer.writeln_tab(2, "final value = reader.readInt32();");
    writer.writeln("");
    writer.writeln_tab(2, "switch (value) {");

    for item in node.items.iter() {
        writer.writeln_tab(3, &format!("case {}:", item.position()));
        writer.writeln_tab(4, &format!("final model = {}();", node.id));
        writer.writeln_tab(
            4,
            &format!(
                "model.value = {}Value.{};",
                node.id,
                item.id().to_case(Case::Camel)
            ),
        );
        writer.writeln_tab(
            4,
            &format!(
                "const {}{}EmplaceToBuffers().read(reader, model.{});",
                node.id,
                item.id(),
                item.id().to_case(Case::Camel),
            ),
        );
        writer.writeln_tab(4, "return model;");
        writer.writeln("");
    }

    // Default case
    writer.writeln_tab(3, "default:");
    writer.writeln_tab(4, "throw StateError('Unsupported enum value: $value');");
    writer.writeln_tab(2, "}");

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_write(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "@override");
    writer.writeln_tab(
        1,
        &format!("void write(BytesWriter writer, {} model) {{", node.id),
    );
    writer.writeln_tab(2, "switch (model.value) {");

    for (idx, item) in node.items.iter().enumerate() {
        writer.writeln_tab(
            3,
            &format!("case {}Value.{}:", node.id, item.id().to_case(Case::Camel)),
        );
        writer.writeln_tab(4, &format!("writer.writeInt32({});", item.position()));
        writer.writeln_tab(
            4,
            &format!(
                "const {}{}EmplaceToBuffers().write(writer, model.{});",
                node.id,
                item.id(),
                item.id().to_case(Case::Camel),
            ),
        );
        writer.writeln_tab(4, "return;");

        if idx != node.items.len() - 1 {
            writer.writeln("");
        }
    }

    writer.writeln_tab(2, "}");
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_skip(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "@override");
    writer.writeln_tab(1, "void skip(BytesReader reader, int count) {");

    writer.writeln_tab(2, "for (int i = 0; i < count; i += 1) {");
    writer.writeln_tab(3, "final value = reader.readInt32();");
    writer.writeln("");
    writer.writeln_tab(3, "switch (value) {");

    for item in node.items.iter() {
        writer.writeln_tab(4, &format!("case {}:", item.position()));
        writer.writeln_tab(
            5,
            &format!(
                "const {}{}EmplaceToBuffers().skip(reader, 1);",
                node.id,
                item.id(),
            ),
        );
        writer.writeln_tab(5, "break;");
        writer.writeln("");
    }

    // Default case
    writer.writeln_tab(4, "default:");
    writer.writeln_tab(5, "throw StateError('Unsupported enum value: $value');");
    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "}");
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}
