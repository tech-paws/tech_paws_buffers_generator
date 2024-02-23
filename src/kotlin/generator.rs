use crate::ast::{ASTNode, ConstASTNode, ConstItemASTNode};

use super::ir::KotlinGeneratorToken;

pub fn generate_consts(ast: &[ASTNode]) -> Vec<KotlinGeneratorToken> {
    let mut tokens = vec![];

    for node in ast {
        if let ASTNode::Const(node) = node {
            tokens.push(generate_const_block(node))
        }
    }

    tokens
}

pub fn generate_const_block(const_node: &ConstASTNode) -> KotlinGeneratorToken {
    let mut body = vec![];

    for item in &const_node.items {
        match &item {
            ConstItemASTNode::Value { id, type_id, value } => {
                body.push(KotlinGeneratorToken::ConstValStatement {
                    id: id.clone(),
                    type_id: Box::new(KotlinGeneratorToken::TypeId(type_id.clone())),
                    value: Box::new(KotlinGeneratorToken::ConstValueExpr {
                        type_id: type_id.clone(),
                        value: value.clone(),
                    }),
                });
            }
            ConstItemASTNode::ConstNode { node } => {
                body.push(generate_const_block(node));
            }
        }
    }

    KotlinGeneratorToken::Object {
        id: const_node.id.clone(),
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
}
