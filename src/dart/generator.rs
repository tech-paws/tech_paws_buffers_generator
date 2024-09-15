use convert_case::{Case, Casing};

use crate::ast::{ASTNode, StructASTNode};

use super::ir::{
    ArgumentDeclarationIR, AssignIR, CallIR, ClassDartIR, DartIR, DefaultConstructorIR, ListIR,
    NamedConstructorIR, ShortFuncIR, VarDeclarationIR,
};

pub fn generate_models(ast: &[ASTNode]) -> Vec<DartIR> {
    let mut ir = vec![];

    for node in ast {
        match node {
            ASTNode::Struct(node) => ir.append(&mut generate_copy_struct_model(node)),
            _ => (),
        }
    }

    ir
}

fn generate_copy_struct_model(node: &StructASTNode) -> Vec<DartIR> {
    let mut ir = vec![];

    if node.fields.is_empty() {
        ir.push(DartIR::Class(ClassDartIR {
            id: node.id.clone(),
            body: vec![DartIR::DefaultConstructor(DefaultConstructorIR {
                id: node.id.clone(),
                is_const: true,
                fields: None,
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
                    fields: None,
                }),
                DartIR::ShortFunc(ShortFuncIR {
                    id: String::from("createDefault"),
                    return_type_id: Some(Box::new(DartIR::Id(node.id.clone()))),
                    is_override: true,
                    args: None,
                    body: Box::new(DartIR::Call(CallIR {
                        path: vec![DartIR::Id(node.id.clone())],
                        is_const: true,
                        args: None,
                    })),
                }),
            ],
            implements: vec![DartIR::Id(format!("BuffersFactory<{}>", node.id))],
        }));
    } else {
        let mut body = vec![];
        let mut default_constructor_fields = vec![];

        for field in node.fields.iter() {
            default_constructor_fields.push(DartIR::ArgumentDeclaration(ArgumentDeclarationIR {
                id: field.name.to_case(Case::Camel),
                is_required: true,
                is_this: true,
                type_id: None,
                assign: None,
            }));
        }

        body.push(DartIR::DefaultConstructor(DefaultConstructorIR {
            id: node.id.clone(),
            is_const: true,
            fields: Some(Box::new(DartIR::List(ListIR {
                items: default_constructor_fields,
                separator: ",",
                new_line: true,
            }))),
        }));

        let mut assigns = vec![];

        for field in node.fields.iter() {
            assigns.push(DartIR::Assign(AssignIR {
                left: Box::new(DartIR::Id(field.name.to_case(Case::Camel))),
                right: Box::new(DartIR::DefaultCopyValueForTypeID(field.type_id.clone())),
            }));
        }

        body.push(DartIR::NamedConstructor(NamedConstructorIR {
            id: node.id.clone(),
            name: String::from("createDefault"),
            is_const: true,
            fields: None,
            assigns,
        }));

        for field in node.fields.iter() {
            body.push(DartIR::VarDeclaration(VarDeclarationIR {
                id: field.name.clone().to_case(Case::Camel),
                type_id: Box::new(DartIR::CopyTypeId(field.type_id.clone())),
                is_final: true,
                assign: None,
            }));
        }

        ir.push(DartIR::Class(ClassDartIR {
            id: node.id.clone(),
            body,
            implements: vec![],
        }));

        let factory_id = format!("{}BuffersFactory", node.id);

        ir.push(DartIR::Class(ClassDartIR {
            id: factory_id.clone(),
            body: vec![
                DartIR::DefaultConstructor(DefaultConstructorIR {
                    id: factory_id.clone(),
                    is_const: true,
                    fields: None,
                }),
                DartIR::ShortFunc(ShortFuncIR {
                    id: String::from("createDefault"),
                    return_type_id: Some(Box::new(DartIR::Id(node.id.clone()))),
                    is_override: true,
                    args: None,
                    body: Box::new(DartIR::Call(CallIR {
                        path: vec![
                            DartIR::Id(node.id.clone()),
                            DartIR::Id(String::from("createDefault")),
                        ],
                        is_const: true,
                        args: None,
                    })),
                }),
            ],
            implements: vec![DartIR::Id(format!("BuffersFactory<{}>", node.id))],
        }));
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

    #[test]
    fn generate_struct_model_test_basic() {
        let src = fs::read_to_string("test_resources/struct_basic.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/struct_basic.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_types() {
        let src = fs::read_to_string("test_resources/struct_types.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/struct_types.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_with_positions() {
        let src = fs::read_to_string("test_resources/struct_with_positions.tpb").unwrap();
        let target = fs::read_to_string("test_resources/dart/struct_with_positions.dart").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }
}
