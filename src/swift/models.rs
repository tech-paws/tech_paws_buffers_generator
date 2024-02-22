// use convert_case::{Case, Casing};

// use crate::{
//     ast::{ASTNode, StructASTNode, StructFieldASTNode},
//     writer::Writer,
// };

// use super::{types::generate_type_id, consts::generate_default_const_value};

// pub fn generate_models(ast: &[ASTNode]) -> String {
//     let mut writer = Writer::default();

//     for node in ast {
//         match node {
//             ASTNode::Struct(node) => writer.write(&generate_struct_model(node, true)),
//             // ASTNode::Enum(node) => writer.writeln(&generate_enum_model(node)),
//             ASTNode::Enum(_) => (),
//             ASTNode::Fn(_) => (),
//             ASTNode::Directive(_) => (),
//             ASTNode::Const(_) => (),
//         }
//     }

//     let mut res = writer.show().to_string();

//     if res.ends_with("\n\n") {
//         res.pop();
//     }

//     res
// }

// pub fn generate_struct_model(node: &StructASTNode, generate_default: bool) -> String {
//     let mut writer = Writer::default();

//     if node.fields.is_empty() {
//         writer.writeln(&format!("struct {} {{", node.id));

//         if generate_default {
//             writer.writeln_tab(1, &format!("static func createDefault() -> {} {{", node.id));
//             writer.writeln_tab(2, &format!("return {}()", node.id));
//             writer.writeln_tab(1, "}");
//         }

//         writer.writeln("}");
//     } else {
//         writer.writeln(&format!("struct {} {{", node.id));
//         writer.write(&generate_struct_parameters(1, &node.fields));

//         if generate_default {
//             writer.writeln("");
//             writer.write(&generate_struct_default(node));
//         }

//         writer.writeln("}");
//     }

//     writer.show().to_string()
// }

// pub fn generate_struct_parameters(tab: usize, params: &[StructFieldASTNode]) -> String {
//     let mut writer = Writer::default();

//     for param in params {
//         let type_id = generate_type_id(&param.type_id);
//         writer.writeln_tab(
//             tab,
//             &format!("var {}: {}", param.name.to_case(Case::Camel), type_id),
//         );
//     }

//     writer.show().to_string()
// }

// fn generate_struct_default(node: &StructASTNode) -> String {
//     let mut writer = Writer::default();

//     writer.writeln_tab(1, &format!("static func createDefault() -> {} {{", node.id));
//     writer.writeln_tab(2, &format!("return {}(", node.id));

//     let mut it = node.fields.iter().peekable();

//     while let Some(field) = it.next() {
//         if it.peek().is_none() {
//             writer.writeln_tab(
//                 3,
//                 &format!(
//                     "{}: {}",
//                     field.name.to_case(Case::Camel),
//                     generate_default_const_value(&field.type_id)
//                 ),
//             );
//         } else {
//             writer.writeln_tab(
//                 3,
//                 &format!(
//                     "{}: {},",
//                     field.name.to_case(Case::Camel),
//                     generate_default_const_value(&field.type_id)
//                 ),
//             );
//         }
//     }

//     writer.writeln_tab(2, ")");
//     writer.writeln_tab(1, "}");

//     writer.show().to_string()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{lexer::Lexer, parser::parse};
//     use std::fs;

//     #[test]
//     fn generate_struct_model_test_empty() {
//         let src = fs::read_to_string("test_resources/struct_empty.tpb").unwrap();
//         let target = fs::read_to_string("test_resources/swift/struct_empty.swift").unwrap();
//         let mut lexer = Lexer::tokenize(&src);
//         let ast = parse(&mut lexer);
//         let actual = generate_models(&ast);
//         println!("{}", actual);
//         assert_eq!(actual, target);
//     }

//     #[test]
//     fn generate_struct_model_test_basic() {
//         let src = fs::read_to_string("test_resources/struct_basic.tpb").unwrap();
//         let target = fs::read_to_string("test_resources/swift/struct_basic.swift").unwrap();
//         let mut lexer = Lexer::tokenize(&src);
//         let ast = parse(&mut lexer);
//         let actual = generate_models(&ast);
//         println!("{}", actual);
//         assert_eq!(actual, target);
//     }

//     #[test]
//     fn generate_struct_model_test_with_positions() {
//         let src = fs::read_to_string("test_resources/struct_with_positions.tpb").unwrap();
//         let target = fs::read_to_string("test_resources/swift/struct_with_positions.swift").unwrap();
//         let mut lexer = Lexer::tokenize(&src);
//         let ast = parse(&mut lexer);
//         let actual = generate_models(&ast);
//         println!("{}", actual);
//         assert_eq!(actual, target);
//     }

//     #[test]
//     fn generate_struct_model_test_generics() {
//         let src = fs::read_to_string("test_resources/struct_generics.tpb").unwrap();
//         let target = fs::read_to_string("test_resources/swift/struct_generics.swift").unwrap();
//         let mut lexer = Lexer::tokenize(&src);
//         let ast = parse(&mut lexer);
//         let actual = generate_models(&ast);
//         println!("{}", actual);
//         assert_eq!(actual, target);
//     }

//     // #[test]
//     // fn generate_enum_models() {
//     //     let src = fs::read_to_string("test_resources/enum.tpb").unwrap();
//     //     let target = fs::read_to_string("test_resources/rust/enum_models.rs").unwrap();
//     //     let mut lexer = Lexer::tokenize(&src);
//     //     let ast = parse(&mut lexer);
//     //     let actual = generate_models(&ast);
//     //     println!("{}", actual);
//     //     assert_eq!(actual, target);
//     // }
// }
