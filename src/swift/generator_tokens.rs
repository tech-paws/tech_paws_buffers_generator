use convert_case::{Case, Casing};

use crate::{
    ast::{ConstValueASTNode, TypeIDASTNode},
    lexer::Literal,
    writer::Writer,
};

use super::types::generate_type_id;

#[derive(Clone, Debug, PartialEq)]
pub enum SwiftGeneratorToken {
    Struct {
        id: String,
        body: Vec<SwiftGeneratorToken>,
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
    StructAssignArgumentInConstructor {
        id: String,
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

pub fn write_tokens(writer: &mut Writer, tokens: &[SwiftGeneratorToken]) {
    let mut last_token: Option<&SwiftGeneratorToken> = None;

    for token in tokens {
        // NOTE(sysint64): New lines between blocks
        if let Some(last_token) = last_token {
            if let SwiftGeneratorToken::Struct { .. } = token {
                if let SwiftGeneratorToken::Struct { .. } = last_token {
                    writer.new_line();
                }

                if let SwiftGeneratorToken::StructConstField { .. } = last_token {
                    writer.new_line();
                }
            }

            if let SwiftGeneratorToken::StructMethod { .. } = token {
                if let SwiftGeneratorToken::StructMethod { .. } = last_token {
                    writer.new_line();
                }

                if let SwiftGeneratorToken::StructConstField { .. } = last_token {
                    writer.new_line();
                }

                if let SwiftGeneratorToken::StructField { .. } = last_token {
                    writer.new_line();
                }
            }

            //
            if let SwiftGeneratorToken::Struct { .. } = last_token {
                if let SwiftGeneratorToken::StructConstField { .. } = token {
                    writer.new_line();
                }
            }

            if let SwiftGeneratorToken::StructMethod { .. } = last_token {
                if let SwiftGeneratorToken::StructConstField { .. } = token {
                    writer.new_line();
                }

                if let SwiftGeneratorToken::StructField { .. } = token {
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
            arguments,
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
        SwiftGeneratorToken::FunctionArgument { id, named, type_id } => {}
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

                let mut it = body.iter().peekable();

                while let Some(token) = it.next() {
                    writer.write_tabs();
                    write_token(writer, token);

                    if it.peek().is_some() {
                        writer.write(",");
                    }
                    writer.new_line();
                }

                writer.pop_tab();
                writer.write_tabs();
                writer.write(")");
            }
        }
        SwiftGeneratorToken::StructAssignArgumentInConstructor { id, type_id, value } => {
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
        SwiftGeneratorToken::StaticCall { type_id, method } => {}
        SwiftGeneratorToken::Call { id, arguments } => {}
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
        TypeIDASTNode::Other { id } => format!("{}.createDefault()", id),
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
