use crate::ast::{self, *};
use crate::rust::consts::generate_const_block;
use crate::rust::enum_buffers::generate_enum_buffers;
use crate::rust::enum_models::generate_enum_model;
use crate::rust::rpc::{generate_register_fn, generate_rpc_method};
use crate::rust::struct_buffers::generate_struct_buffers;
use crate::rust::struct_models::generate_struct_model;
use crate::{lexer::Literal, writer::Writer};

pub fn generate(ast: &[ASTNode]) -> String {
    let mut writer = Writer::default();

    writer.writeln("// GENERATED, DO NOT EDIT");
    writer.writeln("");

    let mut has_doc_comments = false;

    // Top level doc comments
    for node in ast {
        if let ASTNode::DocComments { comments } = node {
            for comment in comments.iter() {
                writer.writeln(&format!("//!{}", comment));
            }

            has_doc_comments = true;
        }
    }

    if has_doc_comments {
        writer.writeln("");
    }

    writer.writeln("#![allow(warnings)]");
    writer.writeln("#![allow(clippy)]");
    writer.writeln("#![allow(unknown_lints)]");
    writer.writeln("");

    let has_rpc = ast::contains_fn_nodes(ast);
    let has_buffers = ast::contains_buffers_nodes(ast);
    let has_consts = ast::contains_consts_nodes(ast);

    if has_buffers || has_rpc {
        writer.writeln("use tech_paws_buffers::memory::{BytesReader, BytesWriter, BuffersModel};");
    }

    if has_rpc {
        writer.writeln("use tech_paws_buffers::runtime_memory::{");
        writer.writeln("    RpcMethodAddress, RpcRuntimeMemory, RpcMethodBuffer,");
        writer.writeln("    RpcMethodPayloadSize, BuffersScopeId,");
        writer.writeln("};");
        writer.writeln(
            "use tech_paws_buffers::{RpcMethodHandler, RpcRuntime, RpcMethod, SignalRpcResult};",
        );
        writer.writeln("use uuid::uuid;");
    }

    let imports = ast::find_directive_group_values(ast, "rust", "use");

    for import in imports {
        let import = match import {
            ast::ConstValueASTNode::Literal {
                literal,
                type_id: _,
            } => match literal {
                Literal::StringLiteral(value) => value,
                _ => panic!("rust use should be a string literal"),
            },
        };
        writer.writeln(&format!("use {};", import));
    }

    if has_consts {
        writer.writeln("");
        writer.write(&generate_consts(ast));
    }

    if has_buffers {
        writer.writeln("");
        writer.write(&generate_models(ast));
        writer.writeln("");
        writer.write(&generate_buffers(ast));
    }

    if has_rpc {
        writer.writeln("");
        writer.write(&generate_rpc(ast));
    }

    let mut res = writer.show().to_string();

    while res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_models(ast: &[ASTNode]) -> String {
    let mut writer = Writer::default();

    for node in ast {
        match node {
            ASTNode::Struct(node) => writer.writeln(&generate_struct_model(node, true)),
            ASTNode::Enum(node) => writer.writeln(&generate_enum_model(node)),
            ASTNode::Fn(_) => (),
            ASTNode::Directive(_) => (),
            ASTNode::Const(_) => (),
            ASTNode::DocComments { .. } => (),
            ASTNode::Trait(_) => (),
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_buffers(ast: &[ASTNode]) -> String {
    let mut writer = Writer::default();

    for node in ast {
        match node {
            ASTNode::Struct(node) => writer.writeln(&generate_struct_buffers(node)),
            ASTNode::Enum(node) => writer.writeln(&generate_enum_buffers(node)),
            ASTNode::Fn(_) => (),
            ASTNode::Directive(_) => (),
            ASTNode::Const(_) => (),
            ASTNode::DocComments { .. } => (),
            ASTNode::Trait(_) => (),
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_rpc(ast: &[ASTNode]) -> String {
    let mut writer = Writer::default();

    for node in ast {
        if let ASTNode::Trait(node) = node {
            writer.writeln(&generate_register_fn(node));

            for method in &node.methods {
                writer.writeln(&generate_rpc_method(node, method));
            }

            break;
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_consts(ast: &[ASTNode]) -> String {
    let mut writer = Writer::default();

    for node in ast {
        if let ASTNode::Const(node) = node {
            writer.writeln(&generate_const_block(0, node))
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id,
            size: _,
            signed: _,
        } => id.clone(),
        TypeIDASTNode::Number { id, size: _ } => id.clone(),
        TypeIDASTNode::Bool { id } => id.clone(),
        TypeIDASTNode::Char { id } => id.clone(),
        TypeIDASTNode::Other { id } => id.clone(),
        TypeIDASTNode::Generic { id, generics } => {
            format!(
                "{}<{}>",
                id,
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

pub fn generate_const_value(node: &ConstValueASTNode) -> String {
    match node {
        ConstValueASTNode::Literal {
            literal,
            type_id: _,
        } => match literal {
            Literal::StringLiteral(value) => format!("\"{}\"", value),
            Literal::IntLiteral(value) => format!("{}", value),
            Literal::NumberLiteral(value) => format!("{}", value),
            Literal::BoolLiteral(value) => format!("{}", value),
        },
    }
}

pub fn generate_read(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id,
            size: _,
            signed: _,
        } => format!("bytes_reader.read_{}()", id),
        TypeIDASTNode::Number { id, size: _ } => format!("bytes_reader.read_{}()", id),
        TypeIDASTNode::Bool { id } => format!("bytes_reader.read_{}()", id),
        TypeIDASTNode::Char { id } => format!("bytes_reader.read_{}()", id),
        TypeIDASTNode::Other { id } => format!("{}::read_from_buffers(bytes_reader)", id),
        TypeIDASTNode::Generic { id, generics } => {
            format!(
                "{}::<{}>::read_from_buffers(bytes_reader)",
                id,
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

pub fn generate_write(type_id: &TypeIDASTNode, accessor: &str, deref: bool) -> String {
    let deref_accessor = format!("*{}", accessor);
    let primitive_accessor = if deref { &deref_accessor } else { accessor };

    match type_id {
        TypeIDASTNode::Integer {
            id,
            size: _,
            signed: _,
        } => format!("bytes_writer.write_{}({});", id, primitive_accessor),
        TypeIDASTNode::Number { id, size: _ } => {
            format!("bytes_writer.write_{}({});", id, primitive_accessor)
        }
        TypeIDASTNode::Bool { id } => format!("bytes_writer.write_{}({});", id, primitive_accessor),
        TypeIDASTNode::Char { id } => format!("bytes_writer.write_{}({});", id, primitive_accessor),
        TypeIDASTNode::Other { id: _ } => {
            format!("{}.write_to_buffers(bytes_writer);", accessor)
        }
        TypeIDASTNode::Generic { id: _, generics: _ } => {
            format!("{}.write_to_buffers(bytes_writer);", accessor)
        }
    }
}

pub fn generate_default_const(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size: _,
            signed: _,
        } => String::from("0"),
        TypeIDASTNode::Number { id: _, size: _ } => String::from("0.0"),
        TypeIDASTNode::Bool { id: _ } => String::from("false"),
        TypeIDASTNode::Char { id: _ } => String::from("0"),
        TypeIDASTNode::Other { id } => format!("{}::default()", id),
        TypeIDASTNode::Generic { id, generics } => {
            format!(
                "{}::<{}>::default()",
                id,
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serial_test::serial;

    use crate::{lexer::Lexer, parser::{init_mock_uuid, parse}};

    use super::*;

    #[test]
    fn generate_empty_file() {
        let src = fs::read_to_string("test_resources/empty.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/empty.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_struct_model() {
        let src = fs::read_to_string("test_resources/struct.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/struct_models.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_struct_macro() {
        let src = fs::read_to_string("test_resources/struct_macro.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/struct_macro.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_enum_models() {
        let src = fs::read_to_string("test_resources/enum.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/enum_models.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_struct_buffers() {
        let src = fs::read_to_string("test_resources/struct.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/struct_buffers.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_buffers(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_enum_buffers() {
        let src = fs::read_to_string("test_resources/enum.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/enum_buffers.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_buffers(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_consts_test() {
        let src = fs::read_to_string("test_resources/consts.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/consts.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_consts(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    #[serial]
    fn generate_rpc_sync_methods() {
        let src = fs::read_to_string("test_resources/rpc_sync_methods.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/rpc_sync_methods.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_rpc(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    #[serial]
    fn generate_rpc_signal_methods() {
        init_mock_uuid();

        let src = fs::read_to_string("test_resources/rpc_signal_methods.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/rpc_signal_methods.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_rpc(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    #[serial]
    fn generate_doc_comments() {
        let src = fs::read_to_string("test_resources/doc_comments.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/doc_comments.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }
}
