use convert_case::{Case, Casing};

use super::ir::SwiftGeneratorToken;

use crate::ast::{
    ASTNode, ConstASTNode, ConstItemASTNode, EnumASTNode, EnumItemASTNode, StructASTNode,
    TypeIDASTNode,
};

pub fn generate_consts(ast: &[ASTNode]) -> Vec<SwiftGeneratorToken> {
    let mut tokens = vec![];

    for node in ast {
        if let ASTNode::Const(node) = node {
            tokens.push(generate_const_block(node))
        }
    }

    tokens
}

pub fn generate_const_block(const_node: &ConstASTNode) -> SwiftGeneratorToken {
    let mut body = vec![];

    for item in &const_node.items {
        match &item {
            ConstItemASTNode::Value { id, type_id, value } => {
                body.push(SwiftGeneratorToken::StructConstField {
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

    SwiftGeneratorToken::Struct {
        id: const_node.id.clone(),
        body,
    }
}

pub fn generate_models(ast: &[ASTNode]) -> Vec<SwiftGeneratorToken> {
    let mut tokens = vec![];

    for node in ast {
        match node {
            ASTNode::Struct(node) => tokens.push(generate_struct_model(node, true)),
            ASTNode::Enum(node) => tokens.push(generate_enum_model(node)),
            ASTNode::Fn(_) => (),
            ASTNode::Directive(_) => (),
            ASTNode::Const(_) => (),
        }
    }

    tokens
}

fn generate_enum_model(node: &EnumASTNode) -> SwiftGeneratorToken {
    let mut body = vec![];

    for case in &node.items {
        let mut parameters = vec![];

        match case {
            EnumItemASTNode::Tuple { values, .. } => {
                for value in values {
                    parameters.push(SwiftGeneratorToken::EnumCaseType {
                        id: None,
                        type_id: value.type_id.clone(),
                    });
                }
            }
            EnumItemASTNode::Struct { fields, .. } => {
                for field in fields {
                    parameters.push(SwiftGeneratorToken::EnumCaseType {
                        id: Some(field.name.clone()),
                        type_id: field.type_id.clone(),
                    });
                }
            }
            _ => (),
        }

        body.push(SwiftGeneratorToken::EnumCase {
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

    method_body.push(SwiftGeneratorToken::ReturnStatement {
        body: Box::new(match first_case {
            EnumItemASTNode::Empty { .. } => SwiftGeneratorToken::FieldAccess {
                instance: None,
                field: first_case.id().to_string(),
            },
            EnumItemASTNode::Tuple { values, .. } => {
                let mut arguments = vec![];

                for value in values {
                    arguments.push(SwiftGeneratorToken::AssignArgument {
                        id: None,
                        value: None,
                        type_id: value.type_id.clone(),
                    });
                }

                SwiftGeneratorToken::Call {
                    id: format!(".{}", first_case.id().to_case(Case::Camel)),
                    arguments,
                }
            }
            EnumItemASTNode::Struct { fields, .. } => {
                let mut arguments = vec![];

                for field in fields {
                    arguments.push(SwiftGeneratorToken::AssignArgument {
                        id: Some(field.name.clone()),
                        value: None,
                        type_id: field.type_id.clone(),
                    });
                }

                SwiftGeneratorToken::Call {
                    id: format!(".{}", first_case.id().to_case(Case::Camel)),
                    arguments,
                }
            }
        }),
    });

    body.push(SwiftGeneratorToken::StructMethod {
        id: String::from("createDefault"),
        is_static: true,
        return_type_id: enum_type_id,
        body: method_body,
        arguments: vec![],
    });

    SwiftGeneratorToken::Enum {
        id: node.id.clone(),
        body,
    }
}

pub fn generate_struct_model(node: &StructASTNode, generate_default: bool) -> SwiftGeneratorToken {
    let mut body = vec![];

    for field in &node.fields {
        body.push(SwiftGeneratorToken::StructField {
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
            new_instance_body.push(SwiftGeneratorToken::AssignStructNamedArgument {
                id: field.name.clone(),
                type_id: field.type_id.clone(),
                value: None,
            });
        }

        method_body.push(SwiftGeneratorToken::ReturnStatement {
            body: Box::new(SwiftGeneratorToken::NewInstance {
                type_id: struct_type_id.clone(),
                body: new_instance_body,
            }),
        });

        body.push(SwiftGeneratorToken::StructMethod {
            id: String::from("createDefault"),
            is_static: true,
            return_type_id: struct_type_id,
            body: method_body,
            arguments: vec![],
        });
    }

    SwiftGeneratorToken::Struct {
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
