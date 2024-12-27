use convert_case::{Case, Casing};

use crate::ast::{
    ASTNode, ConstBlockASTNode, ConstItemASTNode, EnumASTNode, EnumItemASTNode, StructASTNode,
    TypeIDASTNode,
};

use super::ir::{generate_type_id, KotlinIR};

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
        id: const_node.id.clone(),
        is_data_object: false,
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
            id: case_id.clone(),
            is_data_object: true,
            body: vec![
                KotlinIR::Fun {
                    id: String::from("readFromBuffers"),
                    is_override: false,
                    return_type_id: Some(Box::new(KotlinIR::Id(case_id.clone()))),
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
                            body: Box::new(KotlinIR::Id(case_id.clone())),
                        }],
                    })),
                },
                KotlinIR::Fun {
                    id: String::from("writeToBuffers"),
                    is_override: true,
                    return_type_id: None,
                    arguments: Some(Box::new(KotlinIR::List {
                        separator: ",",
                        new_line: false,
                        items: vec![KotlinIR::FunctionArgument {
                            id: "writer".to_string(),
                            type_id: Box::new(KotlinIR::Id("Long".to_string())),
                        }],
                    })),
                    body: Some(Box::new(KotlinIR::Statements { items: vec![] })),
                },
            ],
            extends: vec![KotlinIR::Id(enum_node.id.clone())],
        },
        EnumItemASTNode::Tuple { values, .. } => {
            let mut fields = vec![];
            let mut read_body = vec![];
            let mut write_body = vec![];
            let mut new_instance_body = vec![];

            for value in values.iter() {
                let field_id = format!("p{}", value.position);

                read_body.push(KotlinIR::ValDeclaration {
                    id: field_id.clone(),
                    is_const: false,
                    type_id: None,
                    value: Some(Box::new(generate_read(&value.type_id))),
                });
                write_body.push(generate_write(&value.type_id, &field_id));
                new_instance_body.push(KotlinIR::AssignArgument {
                    id: field_id.clone(),
                    value: Box::new(KotlinIR::Id(field_id.clone())),
                });

                fields.push(KotlinIR::Declaration {
                    separator: Some(","),
                    body: Box::new(KotlinIR::ValDeclaration {
                        id: field_id.clone(),
                        is_const: false,
                        type_id: Some(Box::new(KotlinIR::TypeId(value.type_id.clone()))),
                        value: None,
                    }),
                });
            }

            read_body.push(KotlinIR::Gap);
            read_body.push(KotlinIR::ReturnStatement {
                body: Box::new(KotlinIR::Call {
                    id: case_id.clone(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: new_instance_body,
                        separator: ",",
                        new_line: true,
                    })),
                }),
            });

            KotlinIR::Class {
                id: case_id.clone(),
                is_data_class: true,
                extends: vec![KotlinIR::Id(enum_node.id.clone())],
                fields,
                body: vec![
                    KotlinIR::CompanionObject {
                        body: vec![KotlinIR::Fun {
                            id: String::from("readFromBuffers"),
                            is_override: false,
                            return_type_id: Some(Box::new(KotlinIR::Id(case_id.clone()))),
                            arguments: Some(Box::new(KotlinIR::List {
                                separator: ",",
                                new_line: false,
                                items: vec![KotlinIR::FunctionArgument {
                                    id: "reader".to_string(),
                                    type_id: Box::new(KotlinIR::Id("Long".to_string())),
                                }],
                            })),
                            body: Some(Box::new(KotlinIR::Statements { items: read_body })),
                        }],
                    },
                    KotlinIR::Fun {
                        id: String::from("writeToBuffers"),
                        is_override: true,
                        return_type_id: None,
                        arguments: Some(Box::new(KotlinIR::List {
                            separator: ",",
                            new_line: false,
                            items: vec![KotlinIR::FunctionArgument {
                                id: "writer".to_string(),
                                type_id: Box::new(KotlinIR::Id("Long".to_string())),
                            }],
                        })),
                        body: Some(Box::new(KotlinIR::Statements { items: write_body })),
                    },
                ],
            }
        }
        EnumItemASTNode::Struct { fields, .. } => {
            let mut enum_fields = vec![];
            let mut read_body = vec![];
            let mut write_body = vec![];
            let mut new_instance_body = vec![];

            for field in fields {
                let field_id = field.name.to_case(Case::Camel).clone();

                read_body.push(KotlinIR::ValDeclaration {
                    id: field_id.clone(),
                    is_const: false,
                    type_id: None,
                    value: Some(Box::new(generate_read(&field.type_id))),
                });
                write_body.push(generate_write(&field.type_id, &field_id));
                new_instance_body.push(KotlinIR::AssignArgument {
                    id: field_id.clone(),
                    value: Box::new(KotlinIR::Id(field_id.clone())),
                });

                enum_fields.push(KotlinIR::Declaration {
                    separator: Some(","),
                    body: Box::new(KotlinIR::ValDeclaration {
                        id: field.name.clone(),
                        is_const: false,
                        type_id: Some(Box::new(KotlinIR::TypeId(field.type_id.clone()))),
                        value: None,
                    }),
                });
            }

            read_body.push(KotlinIR::Gap);
            read_body.push(KotlinIR::ReturnStatement {
                body: Box::new(KotlinIR::Call {
                    id: case_id.clone(),
                    arguments: Some(Box::new(KotlinIR::List {
                        items: new_instance_body,
                        separator: ",",
                        new_line: true,
                    })),
                }),
            });

            KotlinIR::Class {
                id: case_id.clone(),
                is_data_class: true,
                extends: vec![KotlinIR::Id(enum_node.id.clone())],
                fields: enum_fields,
                body: vec![
                    KotlinIR::CompanionObject {
                        body: vec![KotlinIR::Fun {
                            id: String::from("readFromBuffers"),
                            is_override: false,
                            return_type_id: Some(Box::new(KotlinIR::Id(case_id.clone()))),
                            arguments: Some(Box::new(KotlinIR::List {
                                separator: ",",
                                new_line: false,
                                items: vec![KotlinIR::FunctionArgument {
                                    id: "reader".to_string(),
                                    type_id: Box::new(KotlinIR::Id("Long".to_string())),
                                }],
                            })),
                            body: Some(Box::new(KotlinIR::Statements { items: read_body })),
                        }],
                    },
                    KotlinIR::Fun {
                        id: String::from("writeToBuffers"),
                        is_override: true,
                        return_type_id: None,
                        arguments: Some(Box::new(KotlinIR::List {
                            separator: ",",
                            new_line: false,
                            items: vec![KotlinIR::FunctionArgument {
                                id: "writer".to_string(),
                                type_id: Box::new(KotlinIR::Id("Long".to_string())),
                            }],
                        })),
                        body: Some(Box::new(KotlinIR::Statements { items: write_body })),
                    },
                ],
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
                    arguments.push(KotlinIR::AssignArgument {
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

    body.push(KotlinIR::CompanionObject {
        body: vec![
            create_default_method,
            generate_enum_read_from_buffers_method(node),
            generate_enum_skip_in_buffers_method(),
        ],
    });

    body.push(KotlinIR::Fun {
        id: "writeToBuffers".to_string(),
        is_override: false,
        arguments: Some(Box::new(KotlinIR::List {
            separator: ",",
            new_line: false,
            items: vec![KotlinIR::FunctionArgument {
                id: "writer".to_string(),
                type_id: Box::new(KotlinIR::Id("Long".to_string())),
            }],
        })),
        return_type_id: None,
        body: None,
    });

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

fn generate_enum_read_from_buffers_method(node: &EnumASTNode) -> KotlinIR {
    let mut method_statements = vec![];
    let case_value_var_name = "case".to_string();

    method_statements.push(KotlinIR::ValDeclaration {
        id: case_value_var_name.clone(),
        is_const: false,
        type_id: None,
        value: Some(Box::new(generate_read(&TypeIDASTNode::Integer {
            id: "u32".to_string(),
            size: 4,
            signed: false,
        }))),
    });

    let mut cases_statements = vec![];

    for case in &node.items {
        cases_statements.push(KotlinIR::WhenCase {
            item: Box::new(KotlinIR::Id(format!("{}U", case.position()))),
            body: Box::new(generate_read(&TypeIDASTNode::Other {
                id: format!("{}{}", node.id, case.id()),
            })),
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

    method_statements.push(KotlinIR::Gap);
    method_statements.push(KotlinIR::ReturnStatement {
        body: Box::new(KotlinIR::When {
            item: Box::new(KotlinIR::Id(case_value_var_name.clone())),
            body: Box::new(KotlinIR::Statements {
                items: cases_statements,
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
            items: method_statements,
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
            arguments.push(KotlinIR::AssignArgument {
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
            type_id: None,
            value: Some(Box::new(read_call)),
        });
    }

    if !read_body.is_empty() {
        method_statements.push(KotlinIR::Statements { items: read_body });
    }

    let mut new_instance_body = vec![];

    for field in &node.fields {
        new_instance_body.push(KotlinIR::AssignArgument {
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
            "Option" => KotlinIR::TrailingLambda {
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
            "Vec" => KotlinIR::TrailingLambda {
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
            "Option" => KotlinIR::TrailingLambda {
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
            "Vec" => KotlinIR::TrailingLambda {
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
    use super::*;
    use crate::{kotlin::ir::stringify_ir, lexer::Lexer, parser::parse};
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
}
