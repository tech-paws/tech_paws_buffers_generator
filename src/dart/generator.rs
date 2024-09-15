use crate::ast::{ASTNode, StructASTNode};

use super::ir::{CallIR, ClassDartIR, DartIR, DefaultConstructorIR, ShortFuncIR};

pub fn generate_models(ast: &[ASTNode]) -> Vec<DartIR> {
    let mut ir = vec![];

    for node in ast {
        match node {
            ASTNode::Struct(node) => ir.append(&mut generate_struct_model(node)),
            _ => (),
        }
    }

    ir
}

fn generate_struct_model(node: &StructASTNode) -> Vec<DartIR> {
    let mut ir = vec![];

    if node.fields.is_empty() {
        ir.push(DartIR::Class(ClassDartIR {
            id: node.id.clone(),
            body: vec![DartIR::DefaultConstructor(DefaultConstructorIR {
                id: node.id.clone(),
                is_const: true,
                fields: vec![],
            })],
            implements: vec![],
        }));

        let factory_id = format!("{}BuffersFactory", node.id);

        ir.push(DartIR::Class(ClassDartIR {
            id: factory_id.clone(),
            body: vec![
                DartIR::DefaultConstructor(DefaultConstructorIR {
                    id: factory_id.clone(),
                    is_const: true,
                    fields: vec![],
                }),
                DartIR::ShortFunc(ShortFuncIR {
                    id: String::from("createDefault"),
                    return_type_id: Some(Box::new(DartIR::Id(node.id.clone()))),
                    is_override: true,
                    args: None,
                    body: Box::new(DartIR::Call(CallIR {
                        id: node.id.clone(),
                        is_const: true,
                        args: None,
                    })),
                }),
            ],
            implements: vec![DartIR::Id(format!("BuffersFactory<{}>", node.id))],
        }));
    } else {
        todo!()
    }

    ir
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{dart::ir::stringify_ir, lexer::Lexer, parser::parse};

    use super::*;

    #[test]
    fn generate_struct_model_test_empty() {
        let src = fs::read_to_string("test_resources/struct_empty.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/struct_empty.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }
}
