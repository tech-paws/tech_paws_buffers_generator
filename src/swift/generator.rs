use convert_case::{Case, Casing};

use super::ir::SwiftIR;

use crate::ast::{
    ASTNode, ConstASTNode, ConstItemASTNode, EnumASTNode, EnumItemASTNode, StructASTNode,
    TypeIDASTNode,
};

pub fn generate_consts(ast: &[ASTNode]) -> Vec<SwiftIR> {
    let mut tokens = vec![];

    for node in ast {
        if let ASTNode::Const(node) = node {
            tokens.push(generate_const_block(node))
        }
    }

    tokens
}

pub fn generate_const_block(const_node: &ConstASTNode) -> SwiftIR {
    let mut body = vec![];

    for item in &const_node.items {
        match &item {
            ConstItemASTNode::Value { id, type_id, value } => {
                body.push(SwiftIR::StructConstField {
                    id: id.clone(),
                    type_id: type_id.clone(),
                    value: value.clone(),
                });
            }
            ConstItemASTNode::ConstNode { node } => {
                body.push(generate_const_block(node));
            }
        }
    }

    SwiftIR::Struct {
        id: const_node.id.clone(),
        body,
    }
}

pub fn generate_models(ast: &[ASTNode]) -> Vec<SwiftIR> {
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

fn generate_enum_model(node: &EnumASTNode) -> SwiftIR {
    let mut body = vec![];

    for case in &node.items {
        let mut parameters = vec![];

        match case {
            EnumItemASTNode::Tuple { values, .. } => {
                for value in values {
                    parameters.push(SwiftIR::EnumCaseType {
                        id: None,
                        type_id: value.type_id.clone(),
                    });
                }
            }
            EnumItemASTNode::Struct { fields, .. } => {
                for field in fields {
                    parameters.push(SwiftIR::EnumCaseType {
                        id: Some(field.name.clone()),
                        type_id: field.type_id.clone(),
                    });
                }
            }
            _ => (),
        }

        body.push(SwiftIR::EnumCase {
            id: case.id().to_string(),
            parameters,
        });
    }

    // createDefault method
    let enum_type_id = TypeIDASTNode::Other {
        id: node.id.clone(),
    };

    let mut method_body = vec![];
    let first_case = node.items.first().unwrap();

    method_body.push(SwiftIR::ReturnStatement {
        body: Box::new(match first_case {
            EnumItemASTNode::Empty { .. } => SwiftIR::FieldAccess {
                instance: None,
                field: first_case.id().to_string(),
            },
            EnumItemASTNode::Tuple { values, .. } => {
                let mut arguments = vec![];

                for value in values {
                    arguments.push(SwiftIR::AssignArgument {
                        id: None,
                        value: None,
                        type_id: value.type_id.clone(),
                    });
                }

                SwiftIR::Call {
                    id: format!(".{}", first_case.id().to_case(Case::Camel)),
                    arguments,
                }
            }
            EnumItemASTNode::Struct { fields, .. } => {
                let mut arguments = vec![];

                for field in fields {
                    arguments.push(SwiftIR::AssignArgument {
                        id: Some(field.name.clone()),
                        value: None,
                        type_id: field.type_id.clone(),
                    });
                }

                SwiftIR::Call {
                    id: format!(".{}", first_case.id().to_case(Case::Camel)),
                    arguments,
                }
            }
        }),
    });

    body.push(SwiftIR::StructMethod {
        id: String::from("createDefault"),
        is_static: true,
        return_type_id: enum_type_id,
        body: method_body,
        arguments: vec![],
    });

    SwiftIR::Enum {
        id: node.id.clone(),
        body,
    }
}

pub fn generate_struct_model(node: &StructASTNode, generate_default: bool) -> SwiftIR {
    let mut body = vec![];

    for field in &node.fields {
        body.push(SwiftIR::StructField {
            id: field.name.clone(),
            type_id: field.type_id.clone(),
        });
    }

    if generate_default {
        let struct_type_id = TypeIDASTNode::Other {
            id: node.id.clone(),
        };

        let mut method_body = vec![];
        let mut new_instance_body = vec![];

        for field in &node.fields {
            new_instance_body.push(SwiftIR::AssignStructNamedArgument {
                id: field.name.clone(),
                type_id: field.type_id.clone(),
                value: None,
            });
        }

        method_body.push(SwiftIR::ReturnStatement {
            body: Box::new(SwiftIR::NewInstance {
                type_id: struct_type_id.clone(),
                body: new_instance_body,
            }),
        });

        body.push(SwiftIR::StructMethod {
            id: String::from("createDefault"),
            is_static: true,
            return_type_id: struct_type_id,
            body: method_body,
            arguments: vec![],
        });
    }

    SwiftIR::Struct {
        id: node.id.clone(),
        body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::parse, swift::ir::stringify_tokens};
    use std::fs;

    #[test]
    fn generate_consts_test() {
        let src = fs::read_to_string("test_resources/consts.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/consts.swift").unwrap();
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
        let target = fs::read_to_string("test_resources/swift/struct_empty.swift").unwrap();
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
        let target = fs::read_to_string("test_resources/swift/struct_basic.swift").unwrap();
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
        let target =
            fs::read_to_string("test_resources/swift/struct_with_positions.swift").unwrap();
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
        let target = fs::read_to_string("test_resources/swift/struct_generics.swift").unwrap();
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
        let target = fs::read_to_string("test_resources/swift/struct_models.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }

    #[test]
    fn generate_enum_model_test() {
        let src = fs::read_to_string("test_resources/enum_extended.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/enum_models.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_tokens(&actual));

        assert_eq!(stringify_tokens(&actual), target);
    }
}
