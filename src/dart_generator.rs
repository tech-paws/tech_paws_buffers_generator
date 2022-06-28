use crate::{
    parser::{ASTNode, EnumASTNode, FnASTNode, StructASTNode, StructFieldASTNode, TypeIDASTNode},
    writer::Writer,
};
use convert_case::{Case, Casing};

pub fn generate(ast: &[ASTNode], models: bool, buffers: bool, rpc: bool) -> String {
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
            ASTNode::Struct(node) => writer.writeln(&generate_struct_model(node)),
            ASTNode::Enum(node) => writer.writeln(&generate_enum_model(node)),
            ASTNode::Fn(_) => (),
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
            ASTNode::Fn(_) => (),
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
            ASTNode::Struct(_) => (),
            ASTNode::Enum(_) => (),
            ASTNode::Fn(node) => writer.writeln(&generate_rpc_method(node)),
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_struct_model(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln(&format!("class {} {{", node.id));

    if node.fields.is_empty() {
        writer.writeln("}");
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
    writer.writeln("}");

    writer.show().to_string()
}

pub fn generate_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size: _,
            signed: _,
        } => String::from("int"),
        TypeIDASTNode::Number { id: _, size } => {
            match size {
                4 => String::from("float"),
                8 => String::from("double"),
                _ => panic!("Unsupported size of number: {}", size),
            }
        }
        TypeIDASTNode::Bool { id: _ } => String::from("bool"),
        TypeIDASTNode::Char { id: _ } => String::from("int"),
        TypeIDASTNode::Other { id } => id.clone(),
    }
}

fn generate_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::new(2);

    let args_struct_id = format!("__{}_rpc_args__", node.id);

    let mut args_struct_fields = vec![];

    for (i, arg) in node.args.iter().enumerate() {
        args_struct_fields.push(StructFieldASTNode {
            position: i as u32,
            name: arg.id.clone(),
            type_id: arg.type_id.clone(),
        });
    }

    let args_struct = StructASTNode {
        id: args_struct_id.clone(),
        fields: args_struct_fields,
    };

    writer.writeln(&generate_struct_model(&args_struct));

    writer.writeln(&generate_struct_buffers(&args_struct));

    writer.show().to_string()
}

pub fn generate_enum_model(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln("}");
    writer.show().to_string()
}

pub fn generate_struct_buffers(node: &StructASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.show().to_string()
}

pub fn generate_enum_buffers(node: &EnumASTNode) -> String {
    let mut writer = Writer::new(2);

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
    #[ignore]
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
    #[ignore]
    fn generate_complex_enum_models() {
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
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
