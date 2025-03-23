use convert_case::{Case, Casing};

use crate::ast::{
    self, ASTNode, ConstBlockASTNode, ConstItemASTNode, EnumASTNode, EnumItemASTNode, FnASTNode,
    StructASTNode, TraitASTNode, TypeIDASTNode,
};

use super::ir::{generate_default_const_value, generate_type_id, KotlinIR};

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
            ConstItemASTNode::Value {
                id, type_id, value, ..
            } => {
                body.push(KotlinIR::Declaration {
                    separator: None,
                    body: Box::new(KotlinIR::ValDeclaration {
                        id: id.clone(),
                        is_const: true,
                        is_private: false,
                        is_private_set: false,
                        type_id: Some(Box::new(KotlinIR::TypeId(type_id.clone()))),
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
        id: const_node.id.to_case(Case::Pascal),
        is_data_object: false,
        extends: vec![],
        body,
    }
}

pub fn generate_models(ast: &[ASTNode]) -> Vec<KotlinIR> {
    let mut ir = vec![];

    for node in ast {
        match node {
            ASTNode::Struct(node) => ir.push(generate_struct_model(node, true)),
            ASTNode::Enum(node) => ir.append(&mut generate_enum_model(node)),
            _ => (),
        }
    }

    ir
}

pub fn generate_rpc(node: &TraitASTNode) -> KotlinIR {
    let mut statements = vec![];

    let scope_id = ast::get_rpc_scope_id(node);

    statements.push(KotlinIR::TopLevelDeclarations {
        items: vec![KotlinIR::ValDeclaration {
            id: "SCOPE_ID".to_string(),
            is_const: true,
            is_private: true,
            is_private_set: false,
            type_id: None,
            value: Some(Box::new(KotlinIR::Id(format!("\"{scope_id}\"")))),
        }],
    });

    let mut signal_flows = vec![];

    for method in &node.methods {
        if method.is_signal {
            let current_id = if method.return_type_id.is_some() {
                format!("{}Current", method.id.to_case(Case::Camel))
            } else {
                "Unit".to_string()
            };

            let flow_id = format!("{}Flow", method.id.to_case(Case::Camel));

            if let Some(return_type_id) = &method.return_type_id {
                signal_flows.push(KotlinIR::VarDeclaration {
                    id: current_id.clone(),
                    is_const: false,
                    is_private: false,
                    is_private_set: true,
                    type_id: Some(Box::new(KotlinIR::TypeId(return_type_id.clone()))),
                    value: Some(Box::new(KotlinIR::Id(generate_default_const_value(
                        return_type_id,
                    )))),
                });
            }

            signal_flows.push(KotlinIR::ValDeclaration {
                id: flow_id.clone(),
                is_const: false,
                is_private: true,
                is_private_set: false,
                type_id: None,
                value: Some(Box::new(KotlinIR::Call {
                    id: "MutableStateFlow".to_string(),
                    arguments: Some(Box::new(KotlinIR::Id(current_id.clone()))),
                })),
            });
        }
    }

    if !signal_flows.is_empty() {
        statements.push(KotlinIR::TopLevelDeclarations {
            items: signal_flows,
        });

        for method in &node.methods {
            if method.is_signal {
                statements.push(KotlinIR::TopLevelDeclarations {
                    items: vec![KotlinIR::ValGetterDeclaration {
                        id: method.id.to_case(Case::Camel),
                        is_private: false,
                        is_private_set: false,
                        type_id: Some(Box::new(KotlinIR::Id(format!(
                            "StateFlow<{}>",
                            method
                                .return_type_id
                                .as_ref()
                                .map_or("Unit".to_string(), generate_type_id)
                        )))),
                        value: Some(Box::new(KotlinIR::Id(format!(
                            "{}Flow",
                            method.id.to_case(Case::Camel)
                        )))),
                    }],
                });
            }
        }

        statements.push(generate_consume_streams_method(node));
    }

    for method in &node.methods {
        if !method.is_signal {
            statements.push(generate_sync_rpc_method(method));
        }
    }

    KotlinIR::Object {
        id: node.id.clone(),
        is_data_object: false,
        body: statements,
        extends: vec![],
    }
}

pub fn generate_traits(ast: &[ASTNode]) -> Vec<KotlinIR> {
    let mut ir = vec![];

    for node in ast {
        if let ASTNode::Trait(node) = node {
            ir.push(generate_rpc(node))
        }
    }

    ir
}

fn generate_consume_streams_method(node: &TraitASTNode) -> KotlinIR {
    let mut statements = vec![];

    for method in &node.methods {
        if method.is_signal {
            let mut consume_result_body_statements = vec![];

            if let Some(type_id) = &method.return_type_id {
                consume_result_body_statements.push(KotlinIR::ValDeclaration {
                    id: "value".to_string(),
                    is_const: false,
                    is_private: false,
                    is_private_set: false,
                    type_id: None,
                    value: Some(Box::new(generate_read(type_id))),
                });
            }

            let value_id = if method.return_type_id.is_some() {
                "value"
            } else {
                "Unit"
            };

            if method.return_type_id.is_some() {
                consume_result_body_statements.push(KotlinIR::Assign {
                    id: format!("{}Current", method.id.to_case(Case::Camel)),
                    value: Box::new(KotlinIR::Id("value".to_string())),
                });
            }

            consume_result_body_statements.push(KotlinIR::Call {
                id: format!("{}Flow.tryEmit", method.id.to_case(Case::Camel)),
                arguments: Some(Box::new(KotlinIR::Id(value_id.to_string()))),
            });

            statements.push(KotlinIR::TrailingBlock {
                arguments: Some(Box::new(KotlinIR::Id("reader".to_string()))),
                call: Box::new(KotlinIR::Call {
                    id: "runtime.consumeResult".to_string(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: vec![
                            KotlinIR::Assign {
                                id: "scopeId".to_string(),
                                value: Box::new(KotlinIR::Id("SCOPE_ID".to_string())),
                            },
                            KotlinIR::Assign {
                                id: "methodId".to_string(),
                                value: Box::new(KotlinIR::Id(format!("{}U", method.position))),
                            },
                        ],
                        separator: ",",
                        new_line: true,
                    })),
                }),
                body: Box::new(KotlinIR::Statements {
                    items: consume_result_body_statements,
                }),
            });
        }
    }

    KotlinIR::Fun {
        id: "consumeSignals".to_string(),
        return_type_id: None,
        is_override: false,
        arguments: Some(Box::new(KotlinIR::FunctionArgument {
            id: "runtime".to_string(),
            type_id: Box::new(KotlinIR::Id("TechPawsBuffersRpcSignalRuntime".to_string())),
        })),
        body: Some(Box::new(KotlinIR::Statements { items: statements })),
    }
}

fn generate_sync_rpc_method(node: &FnASTNode) -> KotlinIR {
    let mut arguments = vec![];
    let mut rpc_body_statements = vec![];
    let mut write_body_statements = vec![];

    for argument in &node.args {
        let id = argument.id.to_case(Case::Camel);

        arguments.push(KotlinIR::FunctionArgument {
            id: id.clone(),
            type_id: Box::new(KotlinIR::TypeId(argument.type_id.clone())),
        });

        write_body_statements.push(generate_write(&argument.type_id, &id));
    }

    if !node.args.is_empty() {
        rpc_body_statements.push(KotlinIR::TrailingBlock {
            call: Box::new(KotlinIR::Id("runtime.writeArgs".to_string())),
            arguments: Some(Box::new(KotlinIR::Id("writer".to_string()))),
            body: Box::new(KotlinIR::Statements {
                items: write_body_statements,
            }),
        });
    }

    rpc_body_statements.push(KotlinIR::Call {
        id: "runtime.callRpc".to_string(),
        arguments: None,
    });

    if let Some(return_type_id) = &node.return_type_id {
        rpc_body_statements.push(KotlinIR::Gap);
        rpc_body_statements.push(KotlinIR::TrailingBlock {
            arguments: Some(Box::new(KotlinIR::Id("reader".to_string()))),
            call: Box::new(KotlinIR::Id("runtime.readResult".to_string())),
            body: Box::new(KotlinIR::Statements {
                items: vec![generate_read(return_type_id)],
            }),
        });
    }

    let mut call = KotlinIR::Call {
        id: "TechPawsBuffersRpcRuntime.rpc".to_string(),
        arguments: Some(Box::new(KotlinIR::List {
            items: vec![
                KotlinIR::Assign {
                    id: "scopeId".to_string(),
                    value: Box::new(KotlinIR::Id("SCOPE_ID".to_string())),
                },
                KotlinIR::Assign {
                    id: "methodId".to_string(),
                    value: Box::new(KotlinIR::Id(format!("{}U", node.position))),
                },
            ],
            separator: ",",
            new_line: true,
        })),
    };

    if node.return_type_id.is_some() {
        call = KotlinIR::ReturnStatement {
            body: Box::new(call),
        }
    }

    KotlinIR::Fun {
        id: node.id.to_case(Case::Camel).clone(),
        is_override: false,
        return_type_id: node
            .return_type_id
            .clone()
            .map(|type_id| Box::new(KotlinIR::TypeId(type_id))),
        arguments: Some(Box::new(KotlinIR::List {
            separator: ",",
            new_line: true,
            items: arguments,
        })),
        body: Some(Box::new(KotlinIR::Statements {
            items: vec![KotlinIR::TrailingBlock {
                arguments: Some(Box::new(KotlinIR::Id("runtime".to_string()))),
                call: Box::new(call),
                body: Box::new(KotlinIR::Statements {
                    items: rpc_body_statements,
                }),
            }],
        })),
    }
}

pub fn generate_enum_model(node: &EnumASTNode) -> Vec<KotlinIR> {
    vec![generate_enum_interface(node)]
}

pub fn generate_enum_case(enum_node: &EnumASTNode, case_node: &EnumItemASTNode) -> KotlinIR {
    let case_id = case_node.id().to_string();

    match case_node {
        EnumItemASTNode::Empty { .. } => KotlinIR::Object {
            id: case_id.clone(),
            is_data_object: true,
            body: vec![],
            extends: vec![KotlinIR::Id(enum_node.id.clone())],
        },
        EnumItemASTNode::Tuple { values, .. } => {
            let mut fields = vec![];

            for value in values.iter() {
                let field_id = format!("p{}", value.position);

                fields.push(KotlinIR::Declaration {
                    separator: Some(","),
                    body: Box::new(KotlinIR::ValDeclaration {
                        id: field_id.clone(),
                        is_const: false,
                        is_private: false,
                        is_private_set: false,
                        type_id: Some(Box::new(KotlinIR::TypeId(value.type_id.clone()))),
                        value: None,
                    }),
                });
            }

            KotlinIR::Class {
                id: case_id.clone(),
                is_data_class: true,
                extends: vec![KotlinIR::Id(enum_node.id.clone())],
                fields,
                body: vec![],
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
                        is_private: false,
                        is_private_set: false,
                        type_id: Some(Box::new(KotlinIR::TypeId(field.type_id.clone()))),
                        value: None,
                    }),
                });
            }

            KotlinIR::Class {
                id: case_id.clone(),
                is_data_class: true,
                extends: vec![KotlinIR::Id(enum_node.id.clone())],
                fields: enum_fields,
                body: vec![],
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
    let first_case_id = first_case.id().to_string();

    let create_default_method = KotlinIR::FunInline {
        id: String::from("createDefault"),
        arguments: vec![],
        return_type_id: Box::new(KotlinIR::TypeId(enum_type_id.clone())),
        body: Box::new(match first_case {
            EnumItemASTNode::Empty { .. } => KotlinIR::Id(first_case_id),
            EnumItemASTNode::Tuple { values, .. } => {
                let mut arguments = vec![];

                for value in values {
                    arguments.push(KotlinIR::DefaultConstValueExpr(value.type_id.clone()));
                }

                KotlinIR::Call {
                    id: first_case_id,
                    arguments: Some(Box::new(KotlinIR::List {
                        separator: ",",
                        items: arguments,
                        new_line: true,
                    })),
                }
            }
            EnumItemASTNode::Struct { fields, .. } => {
                let mut arguments = vec![];

                for field in fields {
                    arguments.push(KotlinIR::Assign {
                        id: field.name.clone(),
                        value: Box::new(KotlinIR::DefaultConstValueExpr(field.type_id.clone())),
                    });
                }

                KotlinIR::Call {
                    id: first_case_id,
                    arguments: Some(Box::new(KotlinIR::List {
                        separator: ",",
                        items: arguments,
                        new_line: true,
                    })),
                }
            }
        }),
    };

    let mut body = vec![];

    for case in &node.items {
        body.push(generate_enum_case(node, case));
    }

    body.push(KotlinIR::CompanionObject {
        body: vec![
            create_default_method,
            generate_enum_read_from_buffers_method(node),
            generate_enum_skip_in_buffers_method(),
        ],
    });

    body.push(generate_enum_write_to_buffers_method(node));

    KotlinIR::Interface {
        id: node.id.clone(),
        is_sealed: true,
        body,
    }
}

fn generate_enum_skip_in_buffers_method() -> KotlinIR {
    KotlinIR::Fun {
        id: String::from("skipInBuffers"),
        is_override: false,
        return_type_id: None,
        arguments: Some(Box::new(KotlinIR::List {
            separator: ",",
            new_line: false,
            items: vec![
                KotlinIR::FunctionArgument {
                    id: "reader".to_string(),
                    type_id: Box::new(KotlinIR::Id("Long".to_string())),
                },
                KotlinIR::FunctionArgument {
                    id: "count".to_string(),
                    type_id: Box::new(KotlinIR::Id("Int".to_string())),
                },
            ],
        })),
        body: Some(Box::new(KotlinIR::Statements {
            items: vec![KotlinIR::ForLoop {
                item: None,
                collection_expr: Box::new(KotlinIR::Range {
                    inclusive: false,
                    from: Box::new(KotlinIR::Id("0".to_string())),
                    to: Box::new(KotlinIR::Id("count".to_string())),
                }),
                body: Box::new(KotlinIR::Statements {
                    items: vec![KotlinIR::Call {
                        id: "readFromBuffers".to_string(),
                        arguments: Some(Box::new(KotlinIR::Id("reader".to_string()))),
                    }],
                }),
            }],
        })),
    }
}

fn generate_enum_write_to_buffers_method(node: &EnumASTNode) -> KotlinIR {
    let mut cases_statements = vec![];
    let position_type = TypeIDASTNode::Integer {
        id: "u64".to_string(),
        size: 4,
        signed: false,
    };

    for case in &node.items {
        let body = match case {
            EnumItemASTNode::Empty {
                doc_comments: _,
                position,
                id: _,
            } => KotlinIR::Block {
                body: Some(Box::new(KotlinIR::Statements {
                    items: vec![generate_write(&position_type, &format!("{position}U"))],
                })),
            },
            EnumItemASTNode::Tuple {
                doc_comments: _,
                position,
                id: _,
                values,
            } => {
                let mut write_body = vec![generate_write(&position_type, &format!("{position}U"))];

                for value in values.iter() {
                    let field_id = format!("p{}", value.position);
                    write_body.push(generate_write(&value.type_id, &field_id));
                }

                KotlinIR::Block {
                    body: Some(Box::new(KotlinIR::Statements { items: write_body })),
                }
            }
            EnumItemASTNode::Struct {
                doc_comments: _,
                position,
                id: _,
                fields,
            } => {
                let mut write_body = vec![generate_write(&position_type, &format!("{position}U"))];

                for field in fields {
                    let field_id = field.name.to_case(Case::Camel);
                    write_body.push(generate_write(&field.type_id, &field_id));
                }

                KotlinIR::Block {
                    body: Some(Box::new(KotlinIR::Statements { items: write_body })),
                }
            }
        };

        let item = match case {
            EnumItemASTNode::Empty {
                doc_comments: _,
                position: _,
                id,
            } => KotlinIR::Id(id.to_string()),
            EnumItemASTNode::Tuple {
                doc_comments: _,
                position: _,
                id,
                values: _,
            } => KotlinIR::Id(format!("is {}", id)),
            EnumItemASTNode::Struct {
                doc_comments: _,
                position: _,
                id,
                fields: _,
            } => KotlinIR::Id(format!("is {}", id)),
        };

        cases_statements.push(KotlinIR::WhenCase {
            item: Box::new(item),
            body: Box::new(body),
        });
    }

    KotlinIR::Fun {
        id: String::from("writeToBuffers"),
        is_override: false,
        return_type_id: None,
        arguments: Some(Box::new(KotlinIR::List {
            separator: ",",
            new_line: false,
            items: vec![KotlinIR::FunctionArgument {
                id: "writer".to_string(),
                type_id: Box::new(KotlinIR::Id("Long".to_string())),
            }],
        })),
        body: Some(Box::new(KotlinIR::Statements {
            items: vec![KotlinIR::When {
                item: Box::new(KotlinIR::Id("this".to_string())),
                body: Box::new(KotlinIR::Statements {
                    items: cases_statements,
                }),
            }],
        })),
    }
}

fn generate_enum_read_from_buffers_method(node: &EnumASTNode) -> KotlinIR {
    let mut cases_statements = vec![];

    for case in &node.items {
        let body = match case {
            EnumItemASTNode::Empty { .. } => KotlinIR::Id(case.id().to_string()),
            EnumItemASTNode::Tuple {
                doc_comments: _,
                position: _,
                id: _,
                values,
            } => {
                let mut read_body = vec![];
                let mut new_instance_body = vec![];

                for value in values.iter() {
                    let field_id = format!("p{}", value.position);
                    read_body.push(KotlinIR::ValDeclaration {
                        id: field_id.clone(),
                        is_const: false,
                        is_private: false,
                        is_private_set: false,
                        type_id: None,
                        value: Some(Box::new(generate_read(&value.type_id))),
                    });
                    new_instance_body.push(KotlinIR::Assign {
                        id: field_id.clone(),
                        value: Box::new(KotlinIR::Id(field_id.clone())),
                    });
                }

                read_body.push(KotlinIR::Gap);
                read_body.push(KotlinIR::Call {
                    id: case.id().to_string(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: new_instance_body,
                        separator: ",",
                        new_line: true,
                    })),
                });

                KotlinIR::Block {
                    body: Some(Box::new(KotlinIR::Statements { items: read_body })),
                }
            }
            EnumItemASTNode::Struct {
                doc_comments: _,
                position: _,
                id: _,
                fields,
            } => {
                let mut read_body = vec![];
                let mut new_instance_body = vec![];

                for field in fields {
                    let field_id = field.name.to_case(Case::Camel).clone();
                    read_body.push(KotlinIR::ValDeclaration {
                        id: field_id.clone(),
                        is_const: false,
                        is_private: false,
                        is_private_set: false,
                        type_id: None,
                        value: Some(Box::new(generate_read(&field.type_id))),
                    });
                    new_instance_body.push(KotlinIR::Assign {
                        id: field_id.clone(),
                        value: Box::new(KotlinIR::Id(field_id.clone())),
                    });
                }

                read_body.push(KotlinIR::Gap);
                read_body.push(KotlinIR::Call {
                    id: case.id().to_string(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: new_instance_body,
                        separator: ",",
                        new_line: true,
                    })),
                });

                KotlinIR::Block {
                    body: Some(Box::new(KotlinIR::Statements { items: read_body })),
                }
            }
        };

        cases_statements.push(KotlinIR::WhenCase {
            item: Box::new(KotlinIR::Id(format!("{}U", case.position()))),
            body: Box::new(body),
        });
    }

    cases_statements.push(KotlinIR::WhenCase {
        item: Box::new(KotlinIR::Id("else".to_string())),
        body: Box::new(KotlinIR::Throw {
            body: Box::new(KotlinIR::Call {
                id: "IllegalArgumentException".to_string(),
                arguments: Some(Box::new(KotlinIR::Id(
                    "\"Invalid enum value: $case\"".to_string(),
                ))),
            }),
        }),
    });

    KotlinIR::Fun {
        id: String::from("readFromBuffers"),
        is_override: false,
        return_type_id: Some(Box::new(KotlinIR::Id(node.id.clone()))),
        arguments: Some(Box::new(KotlinIR::List {
            separator: ",",
            new_line: false,
            items: vec![KotlinIR::FunctionArgument {
                id: "reader".to_string(),
                type_id: Box::new(KotlinIR::Id("Long".to_string())),
            }],
        })),
        body: Some(Box::new(KotlinIR::Statements {
            items: vec![KotlinIR::ReturnStatement {
                body: Box::new(KotlinIR::When {
                    item: Box::new(KotlinIR::ValDeclaration {
                        id: "case".to_string(),
                        is_const: false,
                        is_private: false,
                        is_private_set: false,
                        type_id: None,
                        value: Some(Box::new(generate_read(&TypeIDASTNode::Integer {
                            id: "u32".to_string(),
                            size: 4,
                            signed: false,
                        }))),
                    }),
                    body: Box::new(KotlinIR::Statements {
                        items: cases_statements,
                    }),
                }),
            }],
        })),
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
                is_private: false,
                is_private_set: false,
                type_id: Some(Box::new(KotlinIR::TypeId(field.type_id.clone()))),
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
            arguments.push(KotlinIR::Assign {
                id: field.name.clone(),
                value: Box::new(KotlinIR::DefaultConstValueExpr(field.type_id.clone())),
            });
        }

        let create_default_method = KotlinIR::FunInline {
            id: String::from("createDefault"),
            arguments: vec![],
            return_type_id: Box::new(KotlinIR::TypeId(struct_type_id.clone())),
            body: Box::new(KotlinIR::Call {
                id: node.id.clone(),
                arguments: Some(Box::new(KotlinIR::List {
                    items: arguments,
                    separator: ",",
                    new_line: true,
                })),
            }),
        };

        body.push(KotlinIR::CompanionObject {
            body: vec![
                create_default_method,
                generate_struct_read_from_buffers_method(node),
                generate_struct_skip_in_buffers_method(node),
            ],
        });
    }

    body.push(generate_struct_write_to_buffers_method(node));

    KotlinIR::Class {
        id: node.id.clone(),
        is_data_class: !fields.is_empty(),
        extends: vec![],
        fields,
        body,
    }
}

fn generate_struct_read_from_buffers_method(node: &StructASTNode) -> KotlinIR {
    let mut method_statements = vec![];
    let mut read_body = vec![];

    for field in &node.fields {
        let read_call = generate_read(&field.type_id);

        read_body.push(KotlinIR::ValDeclaration {
            id: field.name.to_case(Case::Camel).clone(),
            is_const: false,
            is_private: false,
            is_private_set: false,
            type_id: None,
            value: Some(Box::new(read_call)),
        });
    }

    if !read_body.is_empty() {
        method_statements.push(KotlinIR::Statements { items: read_body });
    }

    let mut new_instance_body = vec![];

    for field in &node.fields {
        new_instance_body.push(KotlinIR::Assign {
            id: field.name.clone(),
            value: Box::new(KotlinIR::Id(field.name.to_case(Case::Camel).clone())),
        });
    }

    if !node.fields.is_empty() {
        method_statements.push(KotlinIR::Gap);
    }

    method_statements.push(KotlinIR::ReturnStatement {
        body: Box::new(KotlinIR::Call {
            id: node.id.clone(),
            arguments: Some(Box::new(KotlinIR::List {
                items: new_instance_body,
                separator: ",",
                new_line: true,
            })),
        }),
    });

    KotlinIR::Fun {
        id: String::from("readFromBuffers"),
        is_override: false,
        return_type_id: Some(Box::new(KotlinIR::Id(node.id.clone()))),
        arguments: Some(Box::new(KotlinIR::List {
            separator: ",",
            new_line: false,
            items: vec![KotlinIR::FunctionArgument {
                id: "reader".to_string(),
                type_id: Box::new(KotlinIR::Id("Long".to_string())),
            }],
        })),
        body: Some(Box::new(KotlinIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_struct_skip_in_buffers_method(node: &StructASTNode) -> KotlinIR {
    let mut method_statements = vec![];
    let mut read_body = vec![];

    for field in &node.fields {
        read_body.push(generate_read(&field.type_id));
    }

    if !read_body.is_empty() {
        method_statements.push(KotlinIR::ForLoop {
            item: None,
            collection_expr: Box::new(KotlinIR::Range {
                inclusive: false,
                from: Box::new(KotlinIR::Id("0".to_string())),
                to: Box::new(KotlinIR::Id("count".to_string())),
            }),
            body: Box::new(KotlinIR::Statements { items: read_body }),
        });
    }

    KotlinIR::Fun {
        id: String::from("skipInBuffers"),
        is_override: false,
        return_type_id: None,
        arguments: Some(Box::new(KotlinIR::List {
            separator: ",",
            new_line: false,
            items: vec![
                KotlinIR::FunctionArgument {
                    id: "reader".to_string(),
                    type_id: Box::new(KotlinIR::Id("Long".to_string())),
                },
                KotlinIR::FunctionArgument {
                    id: "count".to_string(),
                    type_id: Box::new(KotlinIR::Id("Int".to_string())),
                },
            ],
        })),
        body: Some(Box::new(KotlinIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_struct_write_to_buffers_method(node: &StructASTNode) -> KotlinIR {
    let mut method_statements = vec![];

    for field in &node.fields {
        let write_call = generate_write(&field.type_id, &field.name.to_case(Case::Camel));

        method_statements.push(write_call);
    }

    KotlinIR::Fun {
        id: String::from("writeToBuffers"),
        is_override: false,
        return_type_id: None,
        arguments: Some(Box::new(KotlinIR::List {
            separator: ",",
            new_line: false,
            items: vec![KotlinIR::FunctionArgument {
                id: "writer".to_string(),
                type_id: Box::new(KotlinIR::Id("Long".to_string())),
            }],
        })),
        body: Some(Box::new(KotlinIR::Statements {
            items: method_statements,
        })),
    }
}

fn generate_read(type_id: &TypeIDASTNode) -> KotlinIR {
    let reader_id = "reader".to_string();

    match type_id {
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Option" => KotlinIR::TrailingBlock {
                call: Box::new(KotlinIR::Call {
                    id: "readFromBuffersOptional".to_string(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: vec![KotlinIR::Id(reader_id.clone())],
                        separator: ",",
                        new_line: false,
                    })),
                }),
                arguments: None,
                body: Box::new(KotlinIR::Statements {
                    items: vec![generate_read(
                        generics.first().expect("Optional type cannot be empty"),
                    )],
                }),
            },
            "Vec" => KotlinIR::TrailingBlock {
                call: Box::new(KotlinIR::Call {
                    id: "readFromBuffersList".to_string(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: vec![KotlinIR::Id(reader_id.clone())],
                        separator: ",",
                        new_line: false,
                    })),
                }),
                arguments: None,
                body: Box::new(KotlinIR::Statements {
                    items: vec![generate_read(
                        generics.first().expect("Vec type cannot be empty"),
                    )],
                }),
            },
            _ => KotlinIR::Call {
                id: format!("{}.readFromBuffers", generate_type_id(type_id)),
                arguments: Some(Box::new(KotlinIR::Id(reader_id.clone()))),
            },
        },
        TypeIDASTNode::Other { id } => KotlinIR::Call {
            id: format!("{id}.readFromBuffers"),
            arguments: Some(Box::new(KotlinIR::Id(reader_id.clone()))),
        },
        _ => KotlinIR::Call {
            id: format!("{}.readFromBuffers", generate_type_id(type_id)),
            arguments: Some(Box::new(KotlinIR::Id(reader_id.clone()))),
        },
    }
}

fn generate_write(type_id: &TypeIDASTNode, accessor: &str) -> KotlinIR {
    let writer_id = "writer".to_string();

    match type_id {
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Option" => KotlinIR::TrailingBlock {
                call: Box::new(KotlinIR::Call {
                    id: "writeToBuffersOptional".to_string(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: vec![
                            KotlinIR::Id(writer_id.clone()),
                            KotlinIR::Id(accessor.to_string()),
                        ],
                        separator: ",",
                        new_line: false,
                    })),
                }),
                arguments: Some(Box::new(KotlinIR::Id(format!("{accessor}Item")))),
                body: Box::new(KotlinIR::Statements {
                    items: vec![generate_write(
                        generics.first().expect("Optional type cannot be empty"),
                        &format!("{accessor}Item"),
                    )],
                }),
            },
            "Vec" => KotlinIR::TrailingBlock {
                call: Box::new(KotlinIR::Call {
                    id: "writeToBuffersList".to_string(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: vec![
                            KotlinIR::Id(writer_id.clone()),
                            KotlinIR::Id(accessor.to_string()),
                        ],
                        separator: ",",
                        new_line: false,
                    })),
                }),
                arguments: Some(Box::new(KotlinIR::Id(format!("{accessor}Item")))),
                body: Box::new(KotlinIR::Statements {
                    items: vec![generate_write(
                        generics.first().expect("Vec type cannot be empty"),
                        &format!("{accessor}Item"),
                    )],
                }),
            },
            _ => KotlinIR::Call {
                id: format!("{}.writeToBuffers", accessor),
                arguments: Some(Box::new(KotlinIR::Id(writer_id.clone()))),
            },
        },
        TypeIDASTNode::Other { .. } => KotlinIR::Call {
            id: format!("{}.writeToBuffers", accessor),
            arguments: Some(Box::new(KotlinIR::Id(writer_id.clone()))),
        },
        _ => KotlinIR::Call {
            id: format!("{}.writeToBuffers", accessor),
            arguments: Some(Box::new(KotlinIR::Id(writer_id.clone()))),
        },
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::{
        kotlin::ir::stringify_ir,
        lexer::Lexer,
        parser::{init_mock_uuid, parse},
    };
    use std::fs;

    #[test]
    fn generate_consts_test() {
        let src = fs::read_to_string("test_resources/consts.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/consts.kt").unwrap();
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
        let target = fs::read_to_string("test_resources/kotlin/struct_empty.kt").unwrap();
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
        let target = fs::read_to_string("test_resources/kotlin/struct_basic.kt").unwrap();
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
        let target = fs::read_to_string("test_resources/kotlin/struct_with_positions.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_generics() {
        let src = fs::read_to_string("test_resources/struct_generics.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/struct_generics.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn generate_struct_model_test_gaps() {
        let src = fs::read_to_string("test_resources/struct.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/struct_models.kt").unwrap();
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
        let target = fs::read_to_string("test_resources/kotlin/enum_basic.kt").unwrap();
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
        let target = fs::read_to_string("test_resources/kotlin/struct_types.kt").unwrap();
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
        let target = fs::read_to_string("test_resources/kotlin/enum_with_positions.kt").unwrap();
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
            fs::read_to_string("test_resources/kotlin/enum_with_many_default_arguments.kt")
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
            fs::read_to_string("test_resources/kotlin/enum_with_named_default_arguments.kt")
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
        let target = fs::read_to_string("test_resources/kotlin/enum_extended.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    #[serial]
    fn generate_rpc_sync_methods_test() {
        init_mock_uuid();

        let src = fs::read_to_string("test_resources/rpc_sync_methods.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/rpc_sync_methods.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_traits(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    #[serial]
    fn generate_rpc_signal_methods_test() {
        init_mock_uuid();

        let src = fs::read_to_string("test_resources/rpc_signal_methods.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/rpc_signal_methods.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_traits(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }

    #[test]
    fn regression_struct_abbreviation_in_name_test() {
        let src = fs::read_to_string("test_resources/regression/struct_abbreviation_in_name.tpb")
            .unwrap();
        let target =
            fs::read_to_string("test_resources/kotlin/regression/struct_abbreviation_in_name.kt")
                .unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);

        println!("{:?}", actual);
        println!("{}", stringify_ir(&actual));

        assert_eq!(stringify_ir(&actual), target);
    }
}
