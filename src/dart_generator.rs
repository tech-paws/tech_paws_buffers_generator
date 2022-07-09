use crate::ast::*;
use crate::{
    dart::{
        enum_emplace_buffers::generate_enum_emplace_buffers,
        enum_into_buffers::generate_enum_into_buffers,
        enum_models::create_enum_item_struct_ast_node, rpc::generate_rpc_method,
        struct_emplace_to_buffers::generate_struct_emplace_buffers,
        struct_into_buffers::generate_struct_into_buffers,
    },
    writer::Writer,
};

use convert_case::{Case, Casing};

pub fn generate(_ast: &[ASTNode], _models: bool, buffers: bool, _rpc: bool) -> String {
    let mut writer = Writer::new(2);

    writer.writeln("// GENERATED, DO NOT EDIT");
    writer.writeln("");

    if buffers {
        writer.writeln("import 'package:buffers/buffers.dart';");
    }

    writer.show().to_string()
}

pub fn generate_models(ast: &[ASTNode]) -> String {
    let mut writer = Writer::new(2);

    for node in ast {
        match node {
            ASTNode::Struct(node) => writer.writeln(&generate_struct_model(node, "", true)),
            ASTNode::Enum(node) => writer.write(&generate_enum_model(node)),
            _ => (),
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_buffers(ast: &[ASTNode]) -> String {
    let mut writer = Writer::new(2);

    for node in ast {
        match node {
            ASTNode::Struct(node) => writer.writeln(&generate_struct_buffers(node)),
            ASTNode::Enum(node) => writer.writeln(&generate_enum_buffers(node)),
            _ => (),
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_rpc(ast: &[ASTNode]) -> String {
    let mut writer = Writer::new(2);

    for node in ast {
        match node {
            ASTNode::Fn(node) => writer.writeln(&generate_rpc_method(node)),
            _ => (),
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_struct_model(node: &StructASTNode, def: &str, generate_default: bool) -> String {
    let mut writer = Writer::new(2);

    writer.writeln(&format!("class {}{} {{", node.id, def));

    if node.fields.is_empty() {
        writer.writeln_tab(1, &format!("const {}();", node.id));
        writer.writeln("}");

        if generate_default {
            writer.writeln("");
            writer.writeln(&format!(
                "class {}BuffersFactory implements BuffersFactory<{}> {{",
                node.id, node.id
            ));
            writer.writeln_tab(1, &format!("const {}BuffersFactory();", node.id));
            writer.writeln("");
            writer.writeln_tab(
                1,
                &format!("{} createDefault() => const {}();", node.id, node.id),
            );
            writer.writeln("}");
        }

        return writer.show().to_string();
    }

    for param in node.fields.iter() {
        let type_id = generate_type_id(&param.type_id);
        writer.writeln_tab(
            1,
            &format!("{} {};", type_id, param.name.to_case(Case::Camel)),
        );
    }

    writer.writeln("");

    writer.writeln_tab(1, &format!("{}({{", node.id));

    for param in node.fields.iter() {
        writer.writeln_tab(
            2,
            &format!("required this.{},", param.name.to_case(Case::Camel)),
        );
    }

    writer.writeln_tab(1, "});");

    // Create Default
    if generate_default {
        writer.writeln("");
        writer.writeln_tab(1, &format!("{}.createDefault()", node.id));
        writer.write_tab(3, ": ");

        for (idx, field) in node.fields.iter().enumerate() {
            writer.write(&format!(
                "{} = {}",
                field.name.to_case(Case::Camel),
                &generate_default_const(&field.type_id)
            ));

            if idx == node.fields.len() - 1 {
                writer.writeln(";");
            } else {
                writer.writeln(",");
                writer.write_tab(3, "  ");
            }
        }

        writer.writeln("}");

        // Create Factory class

        writer.writeln("");
        writer.writeln(&format!(
            "class {}BuffersFactory implements BuffersFactory<{}> {{",
            node.id, node.id
        ));
        writer.writeln_tab(1, &format!("const {}BuffersFactory();", node.id));
        writer.writeln("");
        writer.writeln_tab(
            1,
            &format!(
                "{} createDefault() => {}.createDefault();",
                node.id, node.id
            ),
        );
        writer.writeln("}");
    } else {
        writer.writeln("}");
    }

    writer.show().to_string()
}

pub fn generate_struct_buffers(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    if node.emplace_buffers {
        writer.writeln(&generate_struct_emplace_buffers(node));
    }

    if node.into_buffers {
        writer.writeln(&generate_struct_into_buffers(node));
    }

    writer.show().to_string()
}

pub fn generate_enum_model(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);

    // Enum value
    writer.writeln(&format!("enum {}Value {{", node.id));

    for item in node.items.iter() {
        writer.writeln_tab(1, &format!("{},", item.id().to_case(Case::Camel)));
    }

    writer.writeln("}");
    writer.writeln("");

    // Union type
    writer.writeln(&format!("class {}Union {{", node.id));
    let default_union_value = node
        .items
        .first()
        .expect("At least one item should be presented in enum");

    writer.writeln_tab(
        1,
        &format!(
            "{}Value value = {}Value.{};",
            node.id,
            node.id,
            default_union_value.id().to_case(Case::Snake)
        ),
    );

    for item in node.items.iter() {
        let id = item.id();
        let factory = match item {
            EnumItemASTNode::Empty { position: _, id } => format!("const {}{}()", node.id, id),
            EnumItemASTNode::Tuple {
                position: _,
                id,
                values: _,
            } => format!("{}{}.createDefault()", node.id, id),
            EnumItemASTNode::Struct {
                position: _,
                id,
                fields: _,
            } => format!("{}{}.createDefault()", node.id, id),
        };

        writer.writeln_tab(
            1,
            &format!(
                "{}{} {} = {};",
                node.id,
                id,
                id.to_case(Case::Camel),
                &factory
            ),
        );
    }

    writer.writeln("}");
    writer.writeln("");

    // Create default for union
    writer.writeln(&format!(
        "class {}UnionBuffersFactory implements BuffersFactory<{}Union> {{",
        node.id, node.id
    ));
    writer.writeln_tab(1, &format!("const {}UnionBuffersFactory();", node.id));
    writer.writeln("");
    writer.writeln_tab(
        1,
        &format!("{}Union createDefault() => {}Union();", node.id, node.id),
    );
    writer.writeln("}");
    writer.writeln("");

    // enum

    writer.writeln(&format!("abstract class {} {{", node.id));

    for (item_idx, item) in node.items.iter().enumerate() {
        match item {
            EnumItemASTNode::Empty { position: _, id } => {
                writer.writeln_tab(
                    1,
                    &format!(
                        "static const {} = {}{}();",
                        id.to_case(Case::Camel),
                        node.id,
                        id
                    ),
                );
            }
            EnumItemASTNode::Tuple {
                position: _,
                id,
                values,
            } => {
                writer.writeln_tab(
                    1,
                    &format!("static {}{} {}({{", node.id, id, id.to_case(Case::Camel)),
                );

                for (i, value) in values.iter().enumerate() {
                    let type_id = generate_type_id(&value.type_id);
                    writer.writeln_tab(2, &format!("required {} v{},", type_id, i));
                }

                writer.writeln_tab(1, "}) =>");
                writer.writeln_tab(3, &format!("{}{}(", node.id, id));

                for (i, _) in values.iter().enumerate() {
                    writer.writeln_tab(4, &format!("v{}: v{},", i, i));
                }

                writer.writeln_tab(3, ");");
            }
            EnumItemASTNode::Struct {
                position: _,
                id,
                fields,
            } => {
                writer.writeln_tab(
                    1,
                    &format!("static {}{} {}({{", node.id, id, id.to_case(Case::Camel)),
                );
                for field in fields {
                    let type_id = generate_type_id(&field.type_id);
                    writer.writeln_tab(
                        2,
                        &format!("required {} {},", type_id, field.name.to_case(Case::Camel)),
                    );
                }
                writer.writeln_tab(1, "}) =>");
                writer.writeln_tab(3, &format!("{}{}(", node.id, id));

                for field in fields {
                    writer.writeln_tab(4, &format!("{}: {},", field.name, field.name));
                }

                writer.writeln_tab(3, ");");
            }
        }

        if item_idx != node.items.len() - 1 {
            writer.writeln("");
        }
    }

    writer.writeln("}");
    writer.writeln("");

    // Generate enum default

    writer.writeln(&format!(
        "class {}BuffersFactory implements BuffersFactory<{}> {{",
        node.id, node.id
    ));
    writer.writeln_tab(1, &format!("const {}BuffersFactory();", node.id));
    writer.writeln("");
    writer.writeln_tab(
        1,
        &format!(
            "{} createDefault() => const {}{}BuffersFactory().createDefault();",
            node.id,
            node.id,
            node.items.first().unwrap().id()
        ),
    );
    writer.writeln("}");
    writer.writeln("");

    for (item_idx, item) in node.items.iter().enumerate() {
        let enum_class = create_enum_item_struct_ast_node(node, item);
        writer.write(&generate_struct_model(
            &enum_class,
            &format!(" implements {}", node.id),
            true,
        ));

        if item_idx != node.items.len() - 1 {
            writer.writeln("");
        }
    }

    writer.show().to_string()
}

pub fn generate_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size: _,
            signed: _,
        } => String::from("int"),
        TypeIDASTNode::Number { id: _, size: _ } => String::from("double"),
        TypeIDASTNode::Bool { id: _ } => String::from("bool"),
        TypeIDASTNode::Char { id: _ } => String::from("int"),
        TypeIDASTNode::Other { id } => id.clone(),
    }
}

pub fn generate_read(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size,
            signed: _,
        } => {
            match size {
                1 => String::from("reader.readInt8()"),
                4 => String::from("reader.readInt32()"),
                8 => String::from("reader.readInt64()"),
                _ => panic!("Unsupported size of int: {}", size),
            }
        }
        TypeIDASTNode::Number { id: _, size } => {
            match size {
                4 => String::from("reader.readFloat()"),
                8 => String::from("reader.readDouble()"),
                _ => panic!("Unsupported size of number: {}", size),
            }
        }
        TypeIDASTNode::Bool { id: _ } => String::from("reader.readBool()"),
        TypeIDASTNode::Char { id: _ } => String::from("reader.readInt8()"),
        TypeIDASTNode::Other { id } => {
            format!(
                "const {}IntoBuffers().read(reader)",
                id.to_case(Case::Pascal)
            )
        }
    }
}

pub fn generate_default_const(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size: _,
            signed: _,
        } => String::from(""),
        TypeIDASTNode::Number { id: _, size: _ } => String::from("0.0"),
        TypeIDASTNode::Bool { id: _ } => String::from("false"),
        TypeIDASTNode::Char { id: _ } => String::from("0"),
        TypeIDASTNode::Other { id } => {
            format!(
                "const {}BuffersFactory().createDefault()",
                id.to_case(Case::Pascal)
            )
        }
    }
}

pub fn generate_read_emplace(type_id: &TypeIDASTNode, accessor: &str) -> String {
    match type_id {
        TypeIDASTNode::Other { id } => {
            format!(
                "const {}EmplaceToBuffers().read(reader, {});",
                id.to_case(Case::Pascal),
                accessor,
            )
        }
        _ => format!("{} = {};", accessor, generate_read(type_id)),
    }
}

pub fn generate_read_skip(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Other { id } => {
            format!(
                "const {}IntoBuffers().skip(reader, count);",
                id.to_case(Case::Pascal),
            )
        }
        _ => format!("{};", &generate_read(type_id)),
    }
}

pub fn generate_read_skip_emplace(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Other { id } => {
            format!(
                "const {}EmplaceToBuffers().skip(reader, count);",
                id.to_case(Case::Pascal),
            )
        }
        _ => format!("{};", &generate_read(type_id)),
    }
}

pub fn generate_write(type_id: &TypeIDASTNode, accessor: &str) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size,
            signed: _,
        } => {
            match size {
                1 => format!("writer.writeInt8({});", accessor),
                4 => format!("writer.writeInt32({});", accessor),
                8 => format!("writer.writeInt64({});", accessor),
                _ => panic!("Unsupported size of int: {}", size),
            }
        }
        TypeIDASTNode::Number { id: _, size } => {
            match size {
                4 => format!("writer.writeFloat({});", accessor),
                8 => format!("writer.writeDouble({});", accessor),
                _ => panic!("Unsupported size of number: {}", size),
            }
        }
        TypeIDASTNode::Bool { id: _ } => format!("writer.writeBool({});", accessor),
        TypeIDASTNode::Char { id: _ } => format!("writer.writeInt8({});", accessor),
        TypeIDASTNode::Other { id } => {
            format!(
                "const {}IntoBuffers().write(writer, {});",
                id.to_case(Case::Pascal),
                accessor
            )
        }
    }
}

pub fn generate_write_emplace(type_id: &TypeIDASTNode, accessor: &str) -> String {
    match type_id {
        TypeIDASTNode::Other { id } => {
            format!(
                "const {}EmplaceToBuffers().write(writer, {});",
                id.to_case(Case::Pascal),
                accessor
            )
        }
        _ => generate_write(type_id, accessor),
    }
}

pub fn generate_enum_buffers(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln(&generate_enum_into_buffers(node));
    writer.write(&generate_enum_emplace_buffers(node));

    writer.show().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{lexer::Lexer, parser::parse};

    use super::*;

    #[test]
    fn generate_empty_file() {
        let src = fs::read_to_string("test_resources/empty.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/empty.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast, true, true, true);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_empty_struct_model() {
        let src = fs::read_to_string("test_resources/empty_struct.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/empty_struct_models.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_struct_with_parameters_model() {
        let src = fs::read_to_string("test_resources/struct_with_parameters.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/dart/struct_with_parameters_models.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_two_structs_models() {
        let src = fs::read_to_string("test_resources/two_empty_structs.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/dart/two_empty_structs_models.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_struct_with_external_type_models() {
        let src = fs::read_to_string("test_resources/struct_with_external_type.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/dart/struct_with_external_type_models.dart")
                .unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_enum_models() {
        let src = fs::read_to_string("test_resources/enum.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/enum_models.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    #[ignore]
    fn generate_empty_struct_buffers() {
        let src = fs::read_to_string("test_resources/empty_struct.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/empty_struct_buffers.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_buffers(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_struct_with_parameters_buffer() {
        let src = fs::read_to_string("test_resources/struct_with_parameters.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/dart/struct_with_parameters_buffers.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_buffers(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_struct_with_external_type_buffer() {
        let src = fs::read_to_string("test_resources/struct_with_external_type.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/dart/struct_with_external_type_buffers.dart")
                .unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_buffers(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_enum_buffers() {
        let src = fs::read_to_string("test_resources/enum.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/enum_buffers.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_buffers(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    #[ignore]
    fn generate_rpc_method() {
        let src = fs::read_to_string("test_resources/rpc_method.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/rpc_method.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_rpc(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    #[ignore]
    fn generate_rpc_method_without_ret() {
        let src = fs::read_to_string("test_resources/rpc_method_without_ret.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/rpc_method_without_ret.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_rpc(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }
}
