use convert_case::{Case, Casing};

use crate::ast::{ASTNode, ConstASTNode, ConstItemASTNode, StructASTNode, TypeIDASTNode, EnumASTNode};

use super::ir::KotlinIR;

pub fn generate_consts(ast: &[ASTNode]) -> Vec<KotlinIR> {
    let mut tokens = vec![];

    for node in ast {
        if let ASTNode::Const(node) = node {
            tokens.push(generate_const_block(node))
        }
    }

    tokens
}

pub fn generate_const_block(const_node: &ConstASTNode) -> KotlinIR {
    let mut body = vec![];

    for item in &const_node.items {
        match &item {
            ConstItemASTNode::Value { id, type_id, value } => {
                body.push(KotlinIR::Declaration {
                    separator: None,
                    body: Box::new(KotlinIR::ValDeclaration {
                        id: id.clone(),
                        is_const: true,
                        type_id: Box::new(KotlinIR::TypeId(type_id.clone())),
                        value: Some(Box::new(KotlinIR::ConstValueExpr {
                            type_id: type_id.clone(),
                            value: value.clone(),
                        })),
                    }),
                });
            }
            ConstItemASTNode::ConstNode { node } => {
                body.push(generate_const_block(node));
            }
        }
    }

    KotlinIR::Object {
        id: const_node.id.clone(),
        body,
    }
}

pub fn generate_models(ast: &[ASTNode]) -> Vec<KotlinIR> {
    let mut tokens = vec![];

    for node in ast {
        match node {
            ASTNode::Struct(node) => tokens.push(generate_struct_model(node, true)),
            ASTNode::Enum(node) => tokens.push(generate_enum_model(node)),
            _ => (),
        }
    }

    tokens
}

pub fn generate_enum_model(_node: &EnumASTNode) -> KotlinIR {
    todo!();
}

pub fn generate_struct_model(node: &StructASTNode, generate_default: bool) -> KotlinIR {
    let mut body = vec![];
    let mut fields = vec![];

    for field in &node.fields {
        fields.push(KotlinIR::Declaration {
            separator: Some(","),
            body: Box::new(KotlinIR::ValDeclaration {
                id: field.name.to_case(Case::Camel).clone(),
                is_const: false,
                type_id: Box::new(KotlinIR::TypeId(field.type_id.clone())),
                value: None,
            }),
        });
    }

    let struct_type_id = TypeIDASTNode::Other {
        id: node.id.clone(),
    };

    if generate_default {
        let mut arguments = vec![];

        for field in &node.fields {
            arguments.push(KotlinIR::Declaration {
                separator: Some(","),
                body: Box::new(KotlinIR::AssignArgument {
                    id: field.name.clone(),
                    value: Box::new(KotlinIR::DefaulConstValueExpr(field.type_id.clone())),
                }),
            });
        }

        let create_default_method = KotlinIR::FunInline {
            id: String::from("createDefault"),
            arguments: vec![],
            return_type_id: Box::new(KotlinIR::TypeId(struct_type_id.clone())),
            body: Box::new(KotlinIR::Call {
                id: node.id.clone(),
                arguments,
            }),
        };

        body.push(KotlinIR::CompanionObject {
            body: vec![create_default_method],
        });
    }

    KotlinIR::Class {
        id: node.id.clone(),
        is_data_class: !fields.is_empty(),
        fields,
        body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{kotlin::ir::stringify_tokens, lexer::Lexer, parser::parse};
    use std::fs;

    #[test]
    fn generate_consts_test() {
        let src = fs::read_to_string("test_resources/consts.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/consts.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_consts(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_empty() {
        let src = fs::read_to_string("test_resources/struct_empty.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/struct_empty.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_basic() {
        let src = fs::read_to_string("test_resources/struct_basic.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/struct_basic.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_with_positions() {
        let src = fs::read_to_string("test_resources/struct_with_positions.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/struct_with_positions.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_generics() {
        let src = fs::read_to_string("test_resources/struct_generics.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/struct_generics.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_gaps() {
        let src = fs::read_to_string("test_resources/struct.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/struct_models.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    // #[test]
    // fn generate_enum_model_test() {
    //     let src = fs::read_to_string("test_resources/enum_extended.tpb").unwrap();
    //     let target = fs::read_to_string("test_resources/kotlin/enum_models.kt").unwrap();
    //     let mut lexer = Lexer::tokenize(&src);
    //     let ast = parse(&mut lexer);
    //     let actual = generate_models(&ast);

    //     println!("{:?}", actual);
    //     println!("{}", stringify_tokens(&actual));

    //     assert_eq!(stringify_tokens(&actual), target);
    // }
}
