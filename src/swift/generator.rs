use convert_case::{Case, Casing};

use super::ir::{generate_default_const_value, generate_type_id, SwiftIR};

use crate::{
    ast::{
        self, ASTNode, ConstBlockASTNode, ConstItemASTNode, EnumASTNode, EnumItemASTNode,
        FnASTNode, StructASTNode, TypeIDASTNode,
    },
    lexer::Literal,
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

fn generate_const_block(const_node: &ConstBlockASTNode) -> SwiftIR {
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
            ConstItemASTNode::ConstsBlock { node } => {
                body.push(generate_const_block(node));
            }
        }
    }

    SwiftIR::Struct {
        id: const_node.id.to_case(Case::Pascal),
        extends: vec![],
        body,
    }
}

pub fn generate_models(ast: &[ASTNode]) -> Vec<SwiftIR> {
    let mut tokens = vec![];

    for node in ast {
        match node {
            ASTNode::Struct(node) => tokens.push(generate_struct_model(node)),
            ASTNode::Enum(node) => tokens.push(generate_enum_model(node)),
            _ => (),
        }
    }

    tokens
}

pub fn generate_rpc(ast: &[ASTNode]) -> Vec<SwiftIR> {
    let mut statements = vec![];

    let namespace = ast::find_directive_value(ast, "namespace").expect("namespace is required");
    let namespace = match namespace {
        ast::ConstValueASTNode::Literal {
            literal,
            type_id: _,
        } => match literal {
            Literal::StringLiteral(value) => value,
            _ => panic!("namespace should be a string literal"),
        },
    };

    let scope_id = ast::find_directive_value(ast, "id").expect("id is required");
    let scope_id = match scope_id {
        ast::ConstValueASTNode::Literal {
            literal,
            type_id: _,
        } => match literal {
            Literal::StringLiteral(value) => value,
            _ => panic!("id should be a string literal"),
        },
    };

    statements.push(SwiftIR::TopLevelDeclarations {
        items: vec![SwiftIR::StaticVarDeclaration {
            id: "scopeId".to_string(),
            is_const: true,
            is_private: true,
            is_private_set: false,
            type_id: None,
            value: Some(Box::new(SwiftIR::Id(format!("\"{scope_id}\"")))),
        }],
    });

    let mut stream_subjects = vec![];

    for node in ast {
        match node {
            ASTNode::Fn(node) if node.is_signal => {
                if let Some(return_type_id) = &node.return_type_id {
                    stream_subjects.push(SwiftIR::StaticVarDeclaration {
                        id: format!("{}Current", node.id.to_case(Case::Camel)),
                        is_const: false,
                        is_private: true,
                        is_private_set: true,
                        type_id: Some(Box::new(SwiftIR::TypeId(return_type_id.clone()))),
                        value: Some(Box::new(SwiftIR::Id(generate_default_const_value(
                            return_type_id,
                        )))),
                    });
                }

                stream_subjects.push(SwiftIR::StaticVarDeclaration {
                    id: format!("{}Subject", node.id.to_case(Case::Camel)),
                    is_const: true,
                    is_private: true,
                    is_private_set: false,
                    type_id: None,
                    value: Some(Box::new(SwiftIR::Call {
                        id: format!(
                            "PassthroughSubject<{}, Never>",
                            node.return_type_id
                                .as_ref()
                                .map_or("Void".to_string(), generate_type_id)
                        ),
                        arguments: None,
                    })),
                });
            }
            _ => (),
        }
    }

    if !stream_subjects.is_empty() {
        statements.push(SwiftIR::TopLevelDeclarations {
            items: stream_subjects,
        });

        for node in ast {
            match node {
                ASTNode::Fn(node) if node.is_signal => {
                    statements.push(SwiftIR::TopLevelDeclarations {
                        items: vec![SwiftIR::StaticVarDeclaration {
                            id: node.id.to_case(Case::Camel),
                            is_const: false,
                            is_private: false,
                            is_private_set: false,
                            type_id: Some(Box::new(SwiftIR::NamedBlock {
                                id: format!(
                                    "AnyPublisher<{}, Never>",
                                    node.return_type_id
                                        .as_ref()
                                        .map_or("Void".to_string(), generate_type_id)
                                ),
                                body: Box::new(SwiftIR::Statements {
                                    items: vec![SwiftIR::ReturnStatement {
                                        body: Box::new(SwiftIR::ChainCalls {
                                            items: vec![
                                                SwiftIR::Id(format!(
                                                    "{}Subject",
                                                    node.id.to_case(Case::Camel)
                                                )),
                                                SwiftIR::Call {
                                                    id: ".receive".to_string(),
                                                    arguments: Some(Box::new(
                                                        SwiftIR::AssignStructNamedArgument {
                                                            id: "on".to_string(),
                                                            value: Some(Box::new(SwiftIR::Id(
                                                                "DispatchQueue.main".to_string(),
                                                            ))),
                                                            default_value_type_id: None,
                                                        },
                                                    )),
                                                },
                                                SwiftIR::Call {
                                                    id: ".eraseToAnyPublisher".to_string(),
                                                    arguments: None,
                                                },
                                            ],
                                        }),
                                    }],
                                }),
                            })),
                            value: None,
                        }],
                    });
                }
                _ => (),
            }
        }

        statements.push(generate_consume_streams_method(ast));
    }

    for node in ast {
        match node {
            ASTNode::Fn(node) if !node.is_signal => statements.push(generate_sync_rpc_method(node)),
            _ => (),
        }
    }

    if ast::contains_fn_nodes(ast) {
        vec![SwiftIR::Struct {
            id: format!("{}Rpc", namespace.to_case(Case::Pascal)),
            body: statements,
            extends: vec![],
        }]
    } else {
        vec![]
    }
}

fn generate_consume_streams_method(ast: &[ASTNode]) -> SwiftIR {
    let mut statements = vec![];

    for node in ast {
        match node {
            ASTNode::Fn(node) if node.is_signal => {
                let mut consume_result_body_statements = vec![];

                if let Some(type_id) = &node.return_type_id {
                    consume_result_body_statements.push(SwiftIR::VarDeclaration {
                        id: "value".to_string(),
                        is_const: true,
                        type_id: None,
                        value: Some(Box::new(generate_read(type_id))),
                    });
                }

                let value_id = if node.return_type_id.is_some() {
                    "value"
                } else {
                    "()"
                };

                consume_result_body_statements.push(SwiftIR::SetVar {
                    id: format!("{}Current", node.id.to_case(Case::Camel)),
                    value: Box::new(SwiftIR::Id("value".to_string())),
                });
                consume_result_body_statements.push(SwiftIR::Call {
                    id: format!("{}Subject.send", node.id.to_case(Case::Camel)),
                    arguments: Some(Box::new(SwiftIR::Id(value_id.to_string()))),
                });

                statements.push(SwiftIR::TrailingCall {
                    id: "runtime.consumeResult".to_string(),
                    arguments: Some(Box::new(SwiftIR::List {
                        items: vec![
                            SwiftIR::AssignStructNamedArgument {
                                id: "scopeId".to_string(),
                                value: Some(Box::new(SwiftIR::Id("scopeId".to_string()))),
                                default_value_type_id: None,
                            },
                            SwiftIR::AssignStructNamedArgument {
                                id: "methodId".to_string(),
                                value: Some(Box::new(SwiftIR::Id(node.position.to_string()))),
                                default_value_type_id: None,
                            },
                        ],
                        separator: ",",
                        new_line: true,
                    })),
                    input: Some(Box::new(SwiftIR::Id("bytesReader".to_string()))),
                    body: Box::new(SwiftIR::Statements {
                        items: consume_result_body_statements,
                    }),
                });
            }
            _ => (),
        }
    }

    SwiftIR::Func {
        id: "consumeStreams".to_string(),
        is_static: true,
        return_type_id: None,
        arguments: Some(Box::new(SwiftIR::FunctionArgument {
            named: false,
            id: "runtime".to_string(),
            type_id: Box::new(SwiftIR::Id("TechPawsBuffersStream".to_string())),
        })),
        body: Some(Box::new(SwiftIR::Statements { items: statements })),
    }
}

fn generate_sync_rpc_method(node: &FnASTNode) -> SwiftIR {
    let mut arguments = vec![];
    let mut rpc_body_statements = vec![];
    let mut write_body_statements = vec![];

    for argument in &node.args {
        let id = argument.id.to_case(Case::Camel);

        arguments.push(SwiftIR::FunctionArgument {
            named: true,
            id: id.clone(),
            type_id: Box::new(SwiftIR::TypeId(argument.type_id.clone())),
        });

        write_body_statements.push(generate_write(&argument.type_id, &id));
    }

    if !node.args.is_empty() {
        rpc_body_statements.push(SwiftIR::TrailingCall {
            id: "runtime.writeArgs".to_string(),
            arguments: None,
            input: Some(Box::new(SwiftIR::Id("bytesWriter".to_string()))),
            body: Box::new(SwiftIR::Statements {
                items: write_body_statements,
            }),
        });
    }

    rpc_body_statements.push(SwiftIR::Call {
        id: "runtime.callRpc".to_string(),
        arguments: None,
    });

    if let Some(return_type_id) = &node.return_type_id {
        rpc_body_statements.push(SwiftIR::Gap);
        rpc_body_statements.push(SwiftIR::ReturnStatement {
            body: Box::new(SwiftIR::TrailingCall {
                id: "runtime.readResult".to_string(),
                arguments: None,
                input: Some(Box::new(SwiftIR::Id("bytesReader".to_string()))),
                body: Box::new(SwiftIR::Statements {
                    items: vec![SwiftIR::ReturnStatement {
                        body: Box::new(generate_read(return_type_id)),
                    }],
                }),
            }),
        });
    }

    SwiftIR::Func {
        id: node.id.to_case(Case::Camel).clone(),
        is_static: true,
        return_type_id: node
            .return_type_id
            .clone()
            .map(|type_id| Box::new(SwiftIR::TypeId(type_id))),
        arguments: Some(Box::new(SwiftIR::List {
            separator: ",",
            new_line: true,
            items: arguments,
        })),
        body: Some(Box::new(SwiftIR::Statements {
            items: vec![SwiftIR::TrailingCall {
                id: "TechPawsBuffersRpc.rpc".to_string(),
                arguments: Some(Box::new(SwiftIR::List {
                    items: vec![
                        SwiftIR::AssignStructNamedArgument {
                            id: "scopeId".to_string(),
                            value: Some(Box::new(SwiftIR::Id("scopeId".to_string()))),
                            default_value_type_id: None,
                        },
                        SwiftIR::AssignStructNamedArgument {
                            id: "methodId".to_string(),
                            value: Some(Box::new(SwiftIR::Id(node.position.to_string()))),
                            default_value_type_id: None,
                        },
                    ],
                    separator: ",",
                    new_line: true,
                })),
                input: Some(Box::new(SwiftIR::Id("runtime".to_string()))),
                body: Box::new(SwiftIR::Statements {
                    items: rpc_body_statements,
                }),
            }],
        })),
    }
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
    let mut method_statements = vec![];
    let first_case = node.items.first().unwrap();

    method_statements.push(SwiftIR::ReturnStatement {
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
                        default_value_type_id: value.type_id.clone(),
                    });
                }

                SwiftIR::Call {
                    id: format!(".{}", first_case.id().to_case(Case::Camel)),
                    arguments: Some(Box::new(SwiftIR::List {
                        items: arguments,
                        separator: ",",
                        new_line: true,
                    })),
                }
            }
            EnumItemASTNode::Struct { fields, .. } => {
                let mut arguments = vec![];

                for field in fields {
                    arguments.push(SwiftIR::AssignArgument {
                        id: Some(field.name.to_case(Case::Camel).clone()),
                        value: None,
                        default_value_type_id: field.type_id.clone(),
                    });
                }

                SwiftIR::Call {
                    id: format!(".{}", first_case.id().to_case(Case::Camel)),
                    arguments: Some(Box::new(SwiftIR::List {
                        items: arguments,
                        separator: ",",
                        new_line: true,
                    })),
                }
            }
        }),
    });

    body.push(SwiftIR::Func {
        id: String::from("createBuffersDefault"),
        is_static: true,
        return_type_id: Some(Box::new(SwiftIR::Id(node.id.clone()))),
        body: Some(Box::new(SwiftIR::Statements {
            items: method_statements,
        })),
        arguments: None,
    });

    body.push(generate_enum_read_from_buffers_method(node));
    body.push(generate_enum_skip_in_buffers_method(node));
    body.push(generate_enum_write_to_buffers_method(node));

    SwiftIR::Enum {
        id: node.id.clone(),
        extends: vec![SwiftIR::Id(String::from("TechPawsBuffersModel"))],
        body,
    }
}

fn generate_enum_read_from_buffers_method(node: &EnumASTNode) -> SwiftIR {
    let mut method_statements = vec![];
    let case_value_var_name = "caseValue".to_string();

    method_statements.push(SwiftIR::VarDeclaration {
        id: case_value_var_name.clone(),
        is_const: true,
        type_id: None,
        value: Some(Box::new(generate_read(&TypeIDASTNode::Integer {
            id: "u32".to_string(),
            size: 4,
            signed: false,
        }))),
    });

    let mut cases_statements = vec![];

    for case in &node.items {
        let case_id = format!(".{}", case.id().to_case(Case::Camel));

        let body: Vec<SwiftIR> = match case {
            EnumItemASTNode::Empty { .. } => vec![SwiftIR::ReturnStatement {
                body: Box::new(SwiftIR::Id(case_id)),
            }],
            EnumItemASTNode::Tuple {
                doc_comments: _,
                position: _,
                id: _,
                values,
            } => {
                let mut body: Vec<SwiftIR> = vec![];
                let mut new_instance_body = vec![];

                for value in values {
                    let read_call = generate_read(&value.type_id);
                    let field_id = format!("p{}", value.position);

                    body.push(SwiftIR::VarDeclaration {
                        id: field_id.clone(),
                        is_const: true,
                        type_id: None,
                        value: Some(Box::new(read_call)),
                    });

                    new_instance_body.push(SwiftIR::AssignArgument {
                        id: None,
                        value: Some(Box::new(SwiftIR::Id(field_id.clone()))),
                        default_value_type_id: value.type_id.clone(),
                    });
                }

                body.push(SwiftIR::Gap);
                body.push(SwiftIR::ReturnStatement {
                    body: Box::new(SwiftIR::Call {
                        id: case_id,
                        arguments: Some(Box::new(SwiftIR::List {
                            items: new_instance_body,
                            separator: ",",
                            new_line: true,
                        })),
                    }),
                });

                body
            }
            EnumItemASTNode::Struct {
                doc_comments: _,
                position: _,
                id: _,
                fields,
            } => {
                let mut body: Vec<SwiftIR> = vec![];
                let mut new_instance_body = vec![];

                for field in fields {
                    let field_id = field.name.to_case(Case::Camel).clone();
                    let read_call = generate_read(&field.type_id);

                    body.push(SwiftIR::VarDeclaration {
                        id: field_id.clone(),
                        is_const: true,
                        type_id: None,
                        value: Some(Box::new(read_call)),
                    });

                    new_instance_body.push(SwiftIR::AssignArgument {
                        id: None,
                        value: Some(Box::new(SwiftIR::Id(field_id.clone()))),
                        default_value_type_id: field.type_id.clone(),
                    });
                }

                body.push(SwiftIR::Gap);
                body.push(SwiftIR::ReturnStatement {
                    body: Box::new(SwiftIR::Call {
                        id: case_id,
                        arguments: Some(Box::new(SwiftIR::List {
                            items: new_instance_body,
                            separator: ",",
                            new_line: true,
                        })),
                    }),
                });

                body
            }
        };

        cases_statements.push(SwiftIR::Case {
            item: Box::new(SwiftIR::Id(case.position().to_string())),
            body: Box::new(SwiftIR::Statements { items: body }),
        });
    }

    cases_statements.push(SwiftIR::DefaultCase {
        body: Box::new(SwiftIR::Statements {
            items: vec![SwiftIR::Call {
                id: "fatalError".to_string(),
                arguments: Some(Box::new(SwiftIR::Id(
                    "\"Invalid value: \\(caseValue)\"".to_string(),
                ))),
            }],
        }),
    });

    method_statements.push(SwiftIR::Gap);
    method_statements.push(SwiftIR::Switch {
        item: Box::new(SwiftIR::Id(case_value_var_name.clone())),
        body: Box::new(SwiftIR::Statements {
            items: cases_statements,
        }),
    });

    SwiftIR::Func {
        id: String::from("readFromBuffers"),
        is_static: true,
        return_type_id: Some(Box::new(SwiftIR::Id("Self".to_string()))),
        arguments: Some(Box::new(SwiftIR::List {
            separator: ",",
            new_line: true,
            items: vec![SwiftIR::FunctionArgument {
                named: false,
                id: "bytesReader".to_string(),
                type_id: Box::new(SwiftIR::Id("TechPawsBuffersBytesReader".to_string())),
            }],
        })),
        body: Some(Box::new(SwiftIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_enum_skip_in_buffers_method(node: &EnumASTNode) -> SwiftIR {
    let mut method_statements = vec![];
    let case_value_var_name = "caseValue".to_string();

    let mut cases_statements = vec![];

    for case in &node.items {
        let body: Vec<SwiftIR> = match case {
            EnumItemASTNode::Empty { .. } => vec![SwiftIR::Continue],
            EnumItemASTNode::Tuple {
                doc_comments: _,
                position: _,
                id: _,
                values,
            } => {
                let mut body: Vec<SwiftIR> = vec![];

                for value in values {
                    let read_call = generate_read(&value.type_id);

                    body.push(SwiftIR::VarDeclaration {
                        id: "_".to_string(),
                        is_const: true,
                        type_id: None,
                        value: Some(Box::new(read_call)),
                    });
                }

                body
            }
            EnumItemASTNode::Struct {
                doc_comments: _,
                position: _,
                id: _,
                fields,
            } => {
                let mut body: Vec<SwiftIR> = vec![];

                for field in fields {
                    let read_call = generate_read(&field.type_id);

                    body.push(SwiftIR::VarDeclaration {
                        id: "_".to_string(),
                        is_const: true,
                        type_id: None,
                        value: Some(Box::new(read_call)),
                    });
                }

                body
            }
        };

        cases_statements.push(SwiftIR::Case {
            item: Box::new(SwiftIR::Id(case.position().to_string())),
            body: Box::new(SwiftIR::Statements { items: body }),
        });
    }

    cases_statements.push(SwiftIR::DefaultCase {
        body: Box::new(SwiftIR::Statements {
            items: vec![SwiftIR::Call {
                id: "fatalError".to_string(),
                arguments: Some(Box::new(SwiftIR::Id(
                    "\"Invalid value: \\(caseValue)\"".to_string(),
                ))),
            }],
        }),
    });

    method_statements.push(SwiftIR::ForLoop {
        item: None,
        collection_expr: Box::new(SwiftIR::Range {
            from: Box::new(SwiftIR::Id("1".to_string())),
            to: Box::new(SwiftIR::Id("count".to_string())),
        }),
        body: Box::new(SwiftIR::Statements {
            items: vec![
                SwiftIR::VarDeclaration {
                    id: case_value_var_name.clone(),
                    is_const: true,
                    type_id: None,
                    value: Some(Box::new(generate_read(&TypeIDASTNode::Integer {
                        id: "u32".to_string(),
                        size: 4,
                        signed: false,
                    }))),
                },
                SwiftIR::Gap,
                SwiftIR::Switch {
                    item: Box::new(SwiftIR::Id(case_value_var_name.clone())),
                    body: Box::new(SwiftIR::Statements {
                        items: cases_statements,
                    }),
                },
            ],
        }),
    });

    SwiftIR::Func {
        id: String::from("skipInBuffers"),
        is_static: true,
        return_type_id: None,
        arguments: Some(Box::new(SwiftIR::List {
            separator: ",",
            new_line: true,
            items: vec![
                SwiftIR::FunctionArgument {
                    named: false,
                    id: "bytesReader".to_string(),
                    type_id: Box::new(SwiftIR::Id("TechPawsBuffersBytesReader".to_string())),
                },
                SwiftIR::FunctionArgument {
                    named: false,
                    id: "count".to_string(),
                    type_id: Box::new(SwiftIR::Id("UInt64".to_string())),
                },
            ],
        })),
        body: Some(Box::new(SwiftIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_enum_write_to_buffers_method(node: &EnumASTNode) -> SwiftIR {
    let mut method_statements = vec![];
    let mut cases_statements = vec![];

    for case in &node.items {
        let write_type_ir = generate_write(
            &TypeIDASTNode::Integer {
                id: "u32".to_string(),
                size: 4,
                signed: false,
            },
            &case.position().to_string(),
        );

        let case_id = format!(".{}", case.id().to_case(Case::Camel));

        let (case_ir, body) = match case {
            EnumItemASTNode::Empty { .. } => (SwiftIR::Id(case_id), vec![write_type_ir]),
            EnumItemASTNode::Tuple {
                doc_comments: _,
                position: _,
                id: _,
                values,
            } => {
                let mut body = vec![write_type_ir];
                let mut case_arguments = vec![];

                for value in values {
                    let field_name = format!("p{}", value.position);
                    body.push(generate_write(&value.type_id, &field_name));
                    case_arguments.push(SwiftIR::VarDeclaration {
                        id: field_name,
                        is_const: true,
                        type_id: None,
                        value: None,
                    });
                }

                (
                    SwiftIR::Call {
                        id: case_id,
                        arguments: Some(Box::new(SwiftIR::List {
                            items: case_arguments,
                            separator: ",",
                            new_line: true,
                        })),
                    },
                    body,
                )
            }
            EnumItemASTNode::Struct {
                doc_comments: _,
                position: _,
                id: _,
                fields,
            } => {
                let mut body: Vec<SwiftIR> = vec![write_type_ir];
                let mut case_arguments = vec![];

                for field in fields {
                    let field_name = field.name.to_case(Case::Camel);
                    body.push(generate_write(&field.type_id, &field_name));
                    case_arguments.push(SwiftIR::VarDeclaration {
                        id: field_name,
                        is_const: true,
                        type_id: None,
                        value: None,
                    });
                }

                (
                    SwiftIR::Call {
                        id: case_id,
                        arguments: Some(Box::new(SwiftIR::List {
                            items: case_arguments,
                            separator: ",",
                            new_line: true,
                        })),
                    },
                    body,
                )
            }
        };

        cases_statements.push(SwiftIR::Case {
            item: Box::new(case_ir),
            body: Box::new(SwiftIR::Statements { items: body }),
        });
    }

    method_statements.push(SwiftIR::Gap);
    method_statements.push(SwiftIR::Switch {
        item: Box::new(SwiftIR::Id("self".to_string())),
        body: Box::new(SwiftIR::Statements {
            items: cases_statements,
        }),
    });

    SwiftIR::Func {
        id: String::from("writeToBuffers"),
        is_static: false,
        return_type_id: None,
        arguments: Some(Box::new(SwiftIR::List {
            separator: ",",
            new_line: true,
            items: vec![SwiftIR::FunctionArgument {
                named: false,
                id: "bytesWriter".to_string(),
                type_id: Box::new(SwiftIR::Id("TechPawsBuffersBytesWriter".to_string())),
            }],
        })),
        body: Some(Box::new(SwiftIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_struct_model(node: &StructASTNode) -> SwiftIR {
    let mut body = vec![];

    for field in &node.fields {
        body.push(SwiftIR::StructField {
            id: field.name.clone(),
            type_id: field.type_id.clone(),
        });
    }

    let mut method_statements = vec![];
    let mut new_instance_body = vec![];

    for field in &node.fields {
        new_instance_body.push(SwiftIR::AssignStructNamedArgument {
            id: field.name.clone(),
            default_value_type_id: Some(field.type_id.clone()),
            value: None,
        });
    }

    method_statements.push(SwiftIR::ReturnStatement {
        body: Box::new(SwiftIR::Call {
            id: node.id.clone(),
            arguments: Some(Box::new(SwiftIR::List {
                items: new_instance_body,
                separator: ",",
                new_line: true,
            })),
        }),
    });

    body.push(SwiftIR::Func {
        id: String::from("createBuffersDefault"),
        is_static: true,
        return_type_id: Some(Box::new(SwiftIR::Id("Self".to_string()))),
        body: Some(Box::new(SwiftIR::Statements {
            items: method_statements,
        })),
        arguments: None,
    });

    body.push(generate_struct_read_from_buffers_method(node));
    body.push(generate_struct_skip_in_buffers_method(node));
    body.push(generate_struct_write_to_buffers_method(node));

    SwiftIR::Struct {
        id: node.id.clone(),
        extends: vec![SwiftIR::Id(String::from("TechPawsBuffersModel"))],
        body,
    }
}

fn generate_struct_read_from_buffers_method(node: &StructASTNode) -> SwiftIR {
    let mut method_statements = vec![];
    let mut read_body = vec![];

    for field in &node.fields {
        let read_call = generate_read(&field.type_id);

        read_body.push(SwiftIR::VarDeclaration {
            id: field.name.to_case(Case::Camel).clone(),
            is_const: true,
            type_id: None,
            value: Some(Box::new(read_call)),
        });
    }

    if !read_body.is_empty() {
        method_statements.push(SwiftIR::Statements { items: read_body });
    }

    let mut new_instance_body = vec![];

    for field in &node.fields {
        new_instance_body.push(SwiftIR::AssignStructNamedArgument {
            id: field.name.clone(),
            default_value_type_id: Some(field.type_id.clone()),
            value: Some(Box::new(SwiftIR::Id(
                field.name.to_case(Case::Camel).clone(),
            ))),
        });
    }

    if !node.fields.is_empty() {
        method_statements.push(SwiftIR::Gap);
    }

    method_statements.push(SwiftIR::ReturnStatement {
        body: Box::new(SwiftIR::Call {
            id: node.id.clone(),
            arguments: Some(Box::new(SwiftIR::List {
                items: new_instance_body,
                separator: ",",
                new_line: true,
            })),
        }),
    });

    SwiftIR::Func {
        id: String::from("readFromBuffers"),
        is_static: true,
        return_type_id: Some(Box::new(SwiftIR::Id("Self".to_string()))),
        arguments: Some(Box::new(SwiftIR::List {
            separator: ",",
            new_line: true,
            items: vec![SwiftIR::FunctionArgument {
                named: false,
                id: "bytesReader".to_string(),
                type_id: Box::new(SwiftIR::Id("TechPawsBuffersBytesReader".to_string())),
            }],
        })),
        body: Some(Box::new(SwiftIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_struct_skip_in_buffers_method(node: &StructASTNode) -> SwiftIR {
    let mut method_statements = vec![];
    let mut read_body = vec![];

    for field in &node.fields {
        let read_call = generate_read(&field.type_id);

        read_body.push(SwiftIR::VarDeclaration {
            id: "_".to_string(),
            is_const: true,
            type_id: None,
            value: Some(Box::new(read_call)),
        });
    }

    if !read_body.is_empty() {
        method_statements.push(SwiftIR::ForLoop {
            item: None,
            collection_expr: Box::new(SwiftIR::Range {
                from: Box::new(SwiftIR::Id("1".to_string())),
                to: Box::new(SwiftIR::Id("count".to_string())),
            }),
            body: Box::new(SwiftIR::Statements { items: read_body }),
        });
    }

    SwiftIR::Func {
        id: String::from("skipInBuffers"),
        is_static: true,
        return_type_id: None,
        arguments: Some(Box::new(SwiftIR::List {
            separator: ",",
            new_line: true,
            items: vec![
                SwiftIR::FunctionArgument {
                    named: false,
                    id: "bytesReader".to_string(),
                    type_id: Box::new(SwiftIR::Id("TechPawsBuffersBytesReader".to_string())),
                },
                SwiftIR::FunctionArgument {
                    named: false,
                    id: "count".to_string(),
                    type_id: Box::new(SwiftIR::Id("UInt64".to_string())),
                },
            ],
        })),
        body: Some(Box::new(SwiftIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_struct_write_to_buffers_method(node: &StructASTNode) -> SwiftIR {
    let mut method_statements = vec![];

    for field in &node.fields {
        let write_call = generate_write(&field.type_id, &field.name.to_case(Case::Camel));

        method_statements.push(write_call);
    }

    SwiftIR::Func {
        id: String::from("writeToBuffers"),
        is_static: false,
        return_type_id: None,
        arguments: Some(Box::new(SwiftIR::List {
            separator: ",",
            new_line: true,
            items: vec![SwiftIR::FunctionArgument {
                named: false,
                id: "bytesWriter".to_string(),
                type_id: Box::new(SwiftIR::Id("TechPawsBuffersBytesWriter".to_string())),
            }],
        })),
        body: Some(Box::new(SwiftIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_read(type_id: &TypeIDASTNode) -> SwiftIR {
    match type_id {
        TypeIDASTNode::Generic { .. } => SwiftIR::Call {
            id: format!("{}.readFromBuffers", generate_type_id(type_id)),
            arguments: Some(Box::new(SwiftIR::Id("bytesReader".to_string()))),
        },
        TypeIDASTNode::Other { id } => SwiftIR::Call {
            id: format!("{id}.readFromBuffers"),
            arguments: Some(Box::new(SwiftIR::Id("bytesReader".to_string()))),
        },
        _ => SwiftIR::Call {
            id: format!("bytesReader.read{}", generate_type_id(type_id)),
            arguments: None,
        },
    }
}

fn generate_write(type_id: &TypeIDASTNode, accessor: &str) -> SwiftIR {
    match type_id {
        TypeIDASTNode::Generic { .. } => SwiftIR::Call {
            id: format!("{}.writeToBuffers", accessor),
            arguments: Some(Box::new(SwiftIR::Id("bytesWriter".to_string()))),
        },
        TypeIDASTNode::Other { .. } => SwiftIR::Call {
            id: format!("{}.writeToBuffers", accessor),
            arguments: Some(Box::new(SwiftIR::Id("bytesWriter".to_string()))),
        },
        _ => SwiftIR::Call {
            id: format!("bytesWriter.write{}", generate_type_id(type_id)),
            arguments: Some(Box::new(SwiftIR::Id(accessor.to_string()))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::parse, swift::ir::stringify_ir};
    use std::fs;

    #[test]
    fn generate_consts_test() {
        let src = fs::read_to_string("test_resources/consts.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/consts.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_consts(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_empty() {
        let src = fs::read_to_string("test_resources/struct_empty.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/struct_empty.swift").unwrap();
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
        let target = fs::read_to_string("test_resources/swift/struct_basic.swift").unwrap();
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
        let target = fs::read_to_string("test_resources/swift/struct_types.swift").unwrap();
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
        let target =
            fs::read_to_string("test_resources/swift/struct_with_positions.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_enum_model_basic_test() {
        let src = fs::read_to_string("test_resources/enum_basic.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/enum_basic.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_enum_model_with_positions_test() {
        let src = fs::read_to_string("test_resources/enum_with_positions.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/enum_with_positions.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_enum_model_with_many_default_arguments_test() {
        let src =
            fs::read_to_string("test_resources/enum_with_many_default_arguments.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/swift/enum_with_many_default_arguments.swift")
                .unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_enum_model_with_named_default_arguments_test() {
        let src =
            fs::read_to_string("test_resources/enum_with_named_default_arguments.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/swift/enum_with_named_default_arguments.swift")
                .unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_enum_model_extended_test() {
        let src = fs::read_to_string("test_resources/enum_extended.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/enum_extended.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_rpc_sync_methods_test() {
        let src = fs::read_to_string("test_resources/rpc_sync_methods.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/rpc_sync_methods.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_rpc(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_rpc_stream_methods_test() {
        let src = fs::read_to_string("test_resources/rpc_stream_methods.tpb").unwrap();
        let target = fs::read_to_string("test_resources/swift/rpc_stream_methods.swift").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_rpc(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }
}
