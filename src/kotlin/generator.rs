use convert_case::{Case, Casing};

use crate::ast::{
    ASTNode, ConstBlockASTNode, ConstItemASTNode, EnumASTNode, EnumItemASTNode, StructASTNode,
    TypeIDASTNode,
};

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

pub fn generate_const_block(const_node: &ConstBlockASTNode) -> KotlinIR {
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
            ConstItemASTNode::ConstsBlock { node } => {
                body.push(generate_const_block(node));
            }
        }
    }

    KotlinIR::Object {
        id: const_node.id.clone(),
        extends: vec![],
        body,
    }
}

pub fn generate_models(ast: &[ASTNode]) -> Vec<KotlinIR> {
    let mut tokens = vec![];

    for node in ast {
        match node {
            ASTNode::Struct(node) => tokens.push(generate_struct_model(node, true)),
            ASTNode::Enum(node) => tokens.append(&mut generate_enum_model(node)),
            _ => (),
        }
    }

    tokens
}

pub fn generate_enum_model(node: &EnumASTNode) -> Vec<KotlinIR> {
    let mut items = vec![];

    items.push(generate_enum_interface(node));

    for case in &node.items {
        items.push(generate_enum_case(node, case));
    }

    items
}

pub fn generate_enum_case(enum_node: &EnumASTNode, case_node: &EnumItemASTNode) -> KotlinIR {
    let case_id = format!("{}{}", enum_node.id, case_node.id());

    match case_node {
        EnumItemASTNode::Empty { .. } => KotlinIR::Object {
            id: case_id,
            body: vec![],
            extends: vec![KotlinIR::Id(enum_node.id.clone())],
        },
        EnumItemASTNode::Tuple { values, .. } => {
            let mut fields = vec![];

            for value in values.iter() {
                fields.push(KotlinIR::Declaration {
                    separator: Some(","),
                    body: Box::new(KotlinIR::ValDeclaration {
                        id: format!("p{}", value.position),
                        is_const: false,
                        type_id: Box::new(KotlinIR::TypeId(value.type_id.clone())),
                        value: None,
                    }),
                });
            }

            KotlinIR::Class {
                id: case_id,
                is_data_class: true,
                extends: vec![KotlinIR::Id(enum_node.id.clone())],
                body: vec![],
                fields,
            }
        }
        EnumItemASTNode::Struct { fields, .. } => {
            let mut enum_fields = vec![];

            for field in fields {
                enum_fields.push(KotlinIR::Declaration {
                    separator: Some(","),
                    body: Box::new(KotlinIR::ValDeclaration {
                        id: field.name.clone(),
                        is_const: false,
                        type_id: Box::new(KotlinIR::TypeId(field.type_id.clone())),
                        value: None,
                    }),
                });
            }

            KotlinIR::Class {
                id: case_id,
                is_data_class: true,
                extends: vec![KotlinIR::Id(enum_node.id.clone())],
                body: vec![],
                fields: enum_fields,
            }
        }
    }
}

#[allow(clippy::vec_init_then_push)]
pub fn generate_enum_interface(node: &EnumASTNode) -> KotlinIR {
    let enum_type_id = TypeIDASTNode::Other {
        id: node.id.clone(),
    };

    let first_case = node.items.first().unwrap();
    let first_case_id = format!("{}{}", node.id, first_case.id());

    let create_default_method = KotlinIR::FunInline {
        id: String::from("createDefault"),
        arguments: vec![],
        return_type_id: Box::new(KotlinIR::TypeId(enum_type_id.clone())),
        body: Box::new(match first_case {
            EnumItemASTNode::Empty { .. } => KotlinIR::Id(first_case_id),
            EnumItemASTNode::Tuple { values, .. } => {
                let mut arguments = vec![];

                for value in values {
                    arguments.push(KotlinIR::Declaration {
                        separator: Some(","),
                        body: Box::new(KotlinIR::DefaulConstValueExpr(value.type_id.clone())),
                    });
                }

                KotlinIR::Call {
                    id: first_case_id,
                    arguments,
                }
            }
            EnumItemASTNode::Struct { fields, .. } => {
                let mut arguments = vec![];

                for field in fields {
                    arguments.push(KotlinIR::Declaration {
                        separator: Some(","),
                        body: Box::new(KotlinIR::AssignArgument {
                            id: field.name.clone(),
                            value: Box::new(KotlinIR::DefaulConstValueExpr(field.type_id.clone())),
                        }),
                    });
                }

                KotlinIR::Call {
                    id: first_case_id,
                    arguments,
                }
            }
        }),
    };

    let mut body = vec![];

    body.push(KotlinIR::CompanionObject {
        body: vec![create_default_method],
    });

    KotlinIR::Interface {
        id: node.id.clone(),
        is_sealed: true,
        body,
    }
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

    if generate_default {
        let struct_type_id = TypeIDASTNode::Other {
            id: node.id.clone(),
        };

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
        extends: vec![],
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

    #[test]
    fn generate_enum_model_basic_test() {
        let src = fs::read_to_string("test_resources/enum_basic.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/enum_basic.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_enum_model_with_positions_test() {
        let src = fs::read_to_string("test_resources/enum_with_positions.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/enum_with_positions.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_enum_model_with_many_default_arguments_test() {
        let src =
            fs::read_to_string("test_resources/enum_with_many_default_arguments.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/kotlin/enum_with_many_default_arguments.kt")
                .unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_enum_model_with_named_default_arguments_test() {
        let src =
            fs::read_to_string("test_resources/enum_with_named_default_arguments.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/kotlin/enum_with_named_default_arguments.kt")
                .unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_enum_model_extended_test() {
        let src = fs::read_to_string("test_resources/enum_extended.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/enum_extended.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }
}
