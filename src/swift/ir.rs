use convert_case::{Case, Casing};
use strum_macros::IntoStaticStr;

use crate::{
    ast::{ConstValueASTNode, TypeIDASTNode},
    lexer::Literal,
    writer::Writer,
};

#[derive(Clone, Debug, PartialEq, IntoStaticStr)]
pub enum SwiftGeneratorToken {
    Struct {
        id: String,
        body: Vec<SwiftGeneratorToken>,
    },
    Enum {
        id: String,
        body: Vec<SwiftGeneratorToken>,
    },
    EnumCase {
        id: String,
        parameters: Vec<SwiftGeneratorToken>,
    },
    EnumCaseType {
        id: Option<String>,
        type_id: TypeIDASTNode,
    },
    FieldAccess {
        instance: Option<Box<SwiftGeneratorToken>>,
        field: String,
    },
    StructConstField {
        id: String,
        type_id: TypeIDASTNode,
        value: ConstValueASTNode,
    },
    StructField {
        id: String,
        type_id: TypeIDASTNode,
    },
    StructMethod {
        id: String,
        is_static: bool,
        return_type_id: TypeIDASTNode,
        arguments: Vec<SwiftGeneratorToken>,
        body: Vec<SwiftGeneratorToken>,
    },
    FunctionArgument {
        id: String,
        named: bool,
        type_id: TypeIDASTNode,
    },
    ReturnStatement {
        body: Box<SwiftGeneratorToken>,
    },
    NewInstance {
        type_id: TypeIDASTNode,
        body: Vec<SwiftGeneratorToken>,
    },
    AssignStructNamedArgument {
        id: String,
        type_id: TypeIDASTNode,
        value: Option<Box<SwiftGeneratorToken>>,
    },
    AssignArgument {
        id: Option<String>,
        type_id: TypeIDASTNode,
        value: Option<Box<SwiftGeneratorToken>>,
    },
    StaticCall {
        type_id: TypeIDASTNode,
        method: Box<SwiftGeneratorToken>,
    },
    Call {
        id: String,
        arguments: Vec<SwiftGeneratorToken>,
    },
}

pub fn stringify_tokens(tokens: &[SwiftGeneratorToken]) -> String {
    let mut writer = Writer::default();
    write_tokens(&mut writer, tokens);
    writer.show().to_string()
}

pub fn write_tokens_comma_separated(writer: &mut Writer, tokens: &[SwiftGeneratorToken]) {
    let mut it = tokens.iter().peekable();

    while let Some(token) = it.next() {
        writer.write_tabs();
        write_token(writer, token);

        if it.peek().is_some() {
            writer.write(",");
        }
        writer.new_line();
    }
}

pub fn write_tokens(writer: &mut Writer, tokens: &[SwiftGeneratorToken]) {
    let mut last_token: Option<&SwiftGeneratorToken> = None;

    for token in tokens {
        let gaps_pairs = vec![
            ("Struct", "Struct"),
            ("Struct", "Enum"),
            ("Enum", "Enum"),
            ("Struct", "StructConstField"),
            ("Struct", "StructField"),
            ("StructMethod", "StructMethod"),
            ("StructMethod", "StructConstField"),
            ("StructMethod", "StructField"),
            ("StructMethod", "EnumCase"),
        ];

        if let Some(last_token) = last_token {
            for (left, right) in gaps_pairs {
                let last_token: &'static str = last_token.into();
                let token: &'static str = token.into();

                if (left == last_token && right == token) || (right == last_token && left == token)
                {
                    writer.new_line();
                }
            }
        }

        last_token = Some(token);
        write_token(writer, token);
    }
}

fn write_token(writer: &mut Writer, token: &SwiftGeneratorToken) {
    match token {
        SwiftGeneratorToken::Struct { id, body } => {
            writer.writeln(&format!("struct {} {{", id.to_case(Case::Pascal)));
            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();
            writer.writeln("}");
        }
        SwiftGeneratorToken::StructConstField { id, type_id, value } => {
            writer.writeln(&format!(
                "static let {}: {} = {}",
                id.to_case(Case::Camel),
                generate_type_id(type_id),
                generate_const_value(value, type_id)
            ));
        }
        SwiftGeneratorToken::StructField { id, type_id } => writer.writeln(&format!(
            "var {}: {}",
            id.to_case(Case::Camel),
            generate_type_id(type_id),
        )),
        SwiftGeneratorToken::StructMethod {
            id,
            is_static,
            return_type_id,
            arguments: _,
            body,
        } => {
            if *is_static {
                writer.writeln(&format!(
                    "static func {}() -> {} {{",
                    id.to_case(Case::Camel),
                    generate_type_id(return_type_id),
                ));
            } else {
                writer.writeln(&format!(
                    "func {}() -> {} {{",
                    id.to_case(Case::Camel),
                    generate_type_id(return_type_id),
                ));
            }

            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();
            writer.writeln("}");
        }
        SwiftGeneratorToken::FunctionArgument {
            id: _,
            named: _,
            type_id: _,
        } => {}
        SwiftGeneratorToken::ReturnStatement { body } => {
            writer.write_tabs();
            writer.write("return ");
            write_token(writer, body);
            writer.new_line();
        }
        SwiftGeneratorToken::NewInstance { type_id, body } => {
            writer.write(&generate_type_id(type_id));

            if body.is_empty() {
                writer.write("()");
            } else {
                writer.write("(");
                writer.new_line();
                writer.push_tab();
                write_tokens_comma_separated(writer, body);
                writer.pop_tab();
                writer.write_tabs();
                writer.write(")");
            }
        }
        SwiftGeneratorToken::AssignStructNamedArgument { id, type_id, value } => {
            if let Some(value) = value {
                writer.write(&format!("{}: ", id.to_case(Case::Camel),));
                write_token(writer, value);
            } else {
                writer.write(&format!(
                    "{}: {}",
                    id.to_case(Case::Camel),
                    generate_default_const_value(type_id)
                ));
            }
        }
        SwiftGeneratorToken::AssignArgument { id, type_id, value } => {
            if let Some(id) = id {
                writer.write(&format!("/* {} */ ", id.to_case(Case::Camel),));

                if let Some(value) = value {
                    write_token(writer, value);
                } else {
                    writer.write(&generate_default_const_value(type_id));
                }
            } else if let Some(value) = value {
                write_token(writer, value);
            } else {
                writer.write(&generate_default_const_value(type_id));
            }
        }
        SwiftGeneratorToken::StaticCall {
            type_id: _,
            method: _,
        } => {}
        SwiftGeneratorToken::Call { id, arguments } => {
            writer.write(id);

            if arguments.is_empty() {
                writer.write("()");
            } else {
                writer.write("(");
                writer.new_line();
                writer.push_tab();
                write_tokens_comma_separated(writer, arguments);
                writer.pop_tab();
                writer.write_tabs();
                writer.write(")");
            }
        }
        SwiftGeneratorToken::Enum { id, body } => {
            writer.writeln(&format!("enum {} {{", id.to_case(Case::Pascal)));
            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();
            writer.writeln("}");
        }
        SwiftGeneratorToken::EnumCase { id, parameters } => {
            if parameters.is_empty() {
                writer.writeln(&format!("case {}", id.to_case(Case::Camel)));
            } else {
                writer.writeln(&format!("case {}(", id.to_case(Case::Camel)));
                writer.push_tab();
                write_tokens_comma_separated(writer, parameters);
                writer.pop_tab();
                writer.writeln(")");
            }
        }
        SwiftGeneratorToken::EnumCaseType { id, type_id } => {
            if let Some(id) = id {
                writer.write(&format!("/* {} */ {}", id, generate_type_id(type_id)));
            } else {
                writer.write(&generate_type_id(type_id));
            }
        }
        SwiftGeneratorToken::FieldAccess { instance, field } => {
            if let Some(instance) = instance {
                write_token(writer, instance);
                writer.write(&format!(".{}", field.to_case(Case::Camel)));
            } else {
                writer.write(&format!(".{}", field.to_case(Case::Camel)));
            }
        }
    }
}

pub fn generate_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer { size, signed, .. } => match size {
            1 if *signed => String::from("Int8"),
            4 if *signed => String::from("Int32"),
            8 if *signed => String::from("Int64"),
            1 if !*signed => String::from("UInt8"),
            4 if !*signed => String::from("UInt32"),
            8 if !*signed => String::from("UInt64"),
            _ => panic!("Unsupported integer size, {}", size),
        },
        TypeIDASTNode::Number { size, .. } => match size {
            4 => String::from("Float"),
            8 => String::from("Double"),
            _ => panic!("Unsupported number size, {}", size),
        },
        TypeIDASTNode::Bool { .. } => String::from("Bool"),
        TypeIDASTNode::Char { id } => id.clone(),
        TypeIDASTNode::Other { id } => match id.as_str() {
            "GroupAddress" => String::from("UInt64"),
            "CommandsBufferAddress" => String::from("UInt64"),
            _ => id.clone(),
        },
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Vec" => format!(
                "[{}]",
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => format!(
                "{}<{}>",
                id,
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        },
    }
}

pub fn generate_const_value(node: &ConstValueASTNode, type_id: &TypeIDASTNode) -> String {
    match node {
        ConstValueASTNode::Literal { literal, .. } => match literal {
            Literal::StringLiteral(value) => format!("\"{}\"", value),
            Literal::IntLiteral(value) => match type_id {
                TypeIDASTNode::Other { id } => match id.as_str() {
                    "GroupAddress" => format!("{}", value),
                    "CommandsBufferAddress" => format!("{}", value),
                    _ => panic!(
                        "Integer literal cannot have non integer type: {:?}",
                        type_id
                    ),
                },
                TypeIDASTNode::Integer { .. } => format!("{}", value),
                _ => panic!(
                    "Integer literal cannot have non integer type: {:?}",
                    type_id
                ),
            },
            Literal::NumberLiteral(value) => {
                if let TypeIDASTNode::Number { size, .. } = type_id {
                    match size {
                        4 if value.floor() == *value => format!("{}.0f", value),
                        4 => format!("{}f", value),
                        8 if value.floor() == *value => format!("{}.0", value),
                        8 => format!("{}", value),
                        _ => panic!("Unsupported integer size"),
                    }
                } else {
                    panic!("Integer literal cannot have non integer type")
                }
            }
            Literal::BoolLiteral(value) => format!("{}", value),
        },
    }
}

pub fn generate_default_const_value(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size: _,
            signed: _,
        } => String::from("0"),
        TypeIDASTNode::Number { id: _, size: _ } => String::from("0"),
        TypeIDASTNode::Bool { id: _ } => String::from("false"),
        TypeIDASTNode::Char { id: _ } => String::from("0"),
        TypeIDASTNode::Other { id } => match id.as_str() {
            "String" => String::from("\"\""),
            "Vec" => String::from("[]"),
            _ => format!("{}.createDefault()", id),
        },
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Vec" => String::from("[]"),
            _ => format!(
                "{}<{}>.createDefault()",
                id,
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_type_id_test_signed_integers() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i8"),
                size: 1,
                signed: true,
            }),
            String::from("Int8")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i32"),
                size: 4,
                signed: true,
            }),
            String::from("Int32")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i64"),
                size: 8,
                signed: true,
            }),
            String::from("Int64")
        );
    }

    #[test]
    fn generate_type_id_test_unsigned_integers() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("u8"),
                size: 1,
                signed: false,
            }),
            String::from("UInt8")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("u32"),
                size: 4,
                signed: false,
            }),
            String::from("UInt32")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("u64"),
                size: 8,
                signed: false,
            }),
            String::from("UInt64")
        );
    }

    #[test]
    fn generate_type_id_test_numbers() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Number {
                id: String::from("f32"),
                size: 4,
            }),
            String::from("Float")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Number {
                id: String::from("f64"),
                size: 8,
            }),
            String::from("Double")
        );
    }

    #[test]
    fn generate_type_id_test_boolean() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Bool {
                id: String::from("bool")
            }),
            String::from("Bool")
        );
    }

    #[test]
    fn generate_const_value_test_string() {
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::StringLiteral(String::from("Hello World!")),
                    type_id: TypeIDASTNode::Other {
                        id: String::from("String")
                    }
                },
                &TypeIDASTNode::Other {
                    id: String::from("String")
                }
            ),
            String::from("\"Hello World!\"")
        );
    }

    #[test]
    fn generate_const_value_test_int() {
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("i64"),
                        size: 8,
                        signed: true
                    }
                },
                &TypeIDASTNode::Integer {
                    id: String::from("i64"),
                    size: 8,
                    signed: true
                }
            ),
            String::from("13")
        );
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("u64"),
                        size: 8,
                        signed: false
                    }
                },
                &TypeIDASTNode::Integer {
                    id: String::from("u64"),
                    size: 8,
                    signed: false
                }
            ),
            String::from("13")
        );
    }

    #[test]
    fn generate_const_value_test_float() {
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::NumberLiteral(3.14),
                    type_id: TypeIDASTNode::Number {
                        id: String::from("f64"),
                        size: 8,
                    }
                },
                &TypeIDASTNode::Number {
                    id: String::from("f64"),
                    size: 8,
                }
            ),
            String::from("3.14")
        );
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::NumberLiteral(314.0),
                    type_id: TypeIDASTNode::Number {
                        id: String::from("f64"),
                        size: 8,
                    }
                },
                &TypeIDASTNode::Number {
                    id: String::from("f64"),
                    size: 8,
                }
            ),
            String::from("314.0")
        );
    }

    #[test]
    fn generate_const_value_test_boolean() {
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::BoolLiteral(true),
                    type_id: TypeIDASTNode::Bool {
                        id: String::from("Bool")
                    }
                },
                &TypeIDASTNode::Bool {
                    id: String::from("Bool")
                }
            ),
            String::from("true")
        );
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::BoolLiteral(false),
                    type_id: TypeIDASTNode::Bool {
                        id: String::from("Bool")
                    }
                },
                &TypeIDASTNode::Bool {
                    id: String::from("Bool")
                }
            ),
            String::from("false")
        );
    }
}
