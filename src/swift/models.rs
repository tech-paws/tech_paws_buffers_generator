use super::generator_tokens::SwiftGeneratorToken;

use crate::ast::{ASTNode, StructASTNode, TypeIDASTNode};

pub fn generate_models(ast: &[ASTNode]) -> Vec<SwiftGeneratorToken> {
    let mut tokens = vec![];

    for node in ast {
        match node {
            ASTNode::Struct(node) => tokens.push(generate_struct_model(node, true)),
            ASTNode::Enum(_) => (),
            ASTNode::Fn(_) => (),
            ASTNode::Directive(_) => (),
            ASTNode::Const(_) => (),
        }
    }

    tokens
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
            new_instance_body.push(SwiftGeneratorToken::StructAssignArgumentInConstructor {
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
    use crate::{lexer::Lexer, parser::parse, swift::generator_tokens::stringify_tokens};
    use std::fs;

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

    // #[test]
    // fn generate_enum_models() {
    //     let src = fs::read_to_string("test_resources/enum.tpb").unwrap();
    //     let target = fs::read_to_string("test_resources/rust/enum_models.rs").unwrap();
    //     let mut lexer = Lexer::tokenize(&src);
    //     let ast = parse(&mut lexer);
    //     let actual = generate_models(&ast);
    //     println!("{}", actual);
    //     assert_eq!(actual, target);
    // }
}
