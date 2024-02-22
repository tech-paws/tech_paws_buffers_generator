use crate::ast::{ASTNode, ConstASTNode, ConstItemASTNode};

use super::generator::SwiftGeneratorToken;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::parse, swift::generator::stringify_tokens};
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
}
