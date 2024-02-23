use convert_case::{Case, Casing};
use strum_macros::IntoStaticStr;

use crate::{
    ast::{ConstValueASTNode, TypeIDASTNode},
    lexer::Literal,
    writer::Writer,
};

#[derive(Clone, Debug, PartialEq, IntoStaticStr)]
pub enum KotlinGeneratorToken {
    Object {
        id: String,
        body: Vec<KotlinGeneratorToken>,
    },
    ConstValStatement {
        id: String,
        type_id: Box<KotlinGeneratorToken>,
        value: Box<KotlinGeneratorToken>,
    },
    TypeId(TypeIDASTNode),
    ConstValueExpr {
        type_id: TypeIDASTNode,
        value: ConstValueASTNode,
    },
}

pub fn stringify_tokens(tokens: &[KotlinGeneratorToken]) -> String {
    let mut writer = Writer::default();
    write_tokens(&mut writer, tokens);
    writer.show().to_string()
}

pub fn write_tokens(writer: &mut Writer, tokens: &[KotlinGeneratorToken]) {
    let mut last_token: Option<&KotlinGeneratorToken> = None;

    for token in tokens {
        let gaps_pairs = vec![("Object", "Object"), ("Object", "ConstValStatement")];

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

fn write_token(writer: &mut Writer, token: &KotlinGeneratorToken) {
    match token {
        KotlinGeneratorToken::Object { id, body } => {
            writer.writeln(&format!("object {} {{", id.to_case(Case::Pascal)));
            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();
            writer.writeln("}");
        }
        KotlinGeneratorToken::ConstValStatement { id, type_id, value } => {
            writer.write_tabs();
            writer.write(&format!("const val {}", id));
            writer.write(": ");
            write_token(writer, type_id);
            writer.write(" = ");
            write_token(writer, value);
            writer.new_line();
        }
        KotlinGeneratorToken::TypeId(type_id) => writer.write(&generate_type_id(type_id)),
        KotlinGeneratorToken::ConstValueExpr { type_id, value } => {
            writer.write(&generate_const_value(type_id, value))
        }
    }
}

pub fn generate_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer { size, signed, .. } => match size {
            1 if *signed => String::from("Byte"),
            4 if *signed => String::from("Int"),
            8 if *signed => String::from("Long"),
            1 if !*signed => String::from("UByte"),
            4 if !*signed => String::from("UInt"),
            8 if !*signed => String::from("ULong"),
            _ => panic!("Unsupported integer size, {}", size),
        },
        TypeIDASTNode::Number { size, .. } => match size {
            4 => String::from("Float"),
            8 => String::from("Double"),
            _ => panic!("Unsupported number size, {}", size),
        },
        TypeIDASTNode::Bool { .. } => String::from("Boolean"),
        TypeIDASTNode::Char { .. } => String::from("Char"),
        TypeIDASTNode::Other { id } => match id.as_str() {
            "GroupAddress" => String::from("ULong"),
            "CommandsBufferAddress" => String::from("ULong"),
            _ => id.clone(),
        },
        TypeIDASTNode::Generic { id, generics } => {
            format!(
                "{}<{}>",
                id,
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

pub fn generate_const_value(type_id: &TypeIDASTNode, node: &ConstValueASTNode) -> String {
    match node {
        ConstValueASTNode::Literal { literal, .. } => match literal {
            Literal::StringLiteral(value) => format!("\"{}\"", value),
            Literal::IntLiteral(value) => match type_id {
                TypeIDASTNode::Other { id } => match id.as_str() {
                    "GroupAddress" => format!("{}UL", value),
                    "CommandsBufferAddress" => format!("{}UL", value),
                    _ => panic!(
                        "Integer literal cannot have non integer type: {:?}",
                        type_id
                    ),
                },
                TypeIDASTNode::Integer { size, signed, .. } => match size {
                    1 if *signed => format!("{}", value),
                    4 if *signed => format!("{}", value),
                    8 if *signed => format!("{}L", value),
                    1 if !*signed => format!("{}U", value),
                    4 if !*signed => format!("{}U", value),
                    8 if !*signed => format!("{}UL", value),
                    _ => panic!("Unsupported integer size"),
                },
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

#[cfg(test)]
mod tests {
    use crate::{
        ast::{ConstValueASTNode, TypeIDASTNode},
        kotlin::ir::{generate_const_value, generate_type_id},
        lexer::Literal,
    };

    #[test]
    fn generate_type_id_test_signed_integers() {
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i8"),
                size: 1,
                signed: true,
            }),
            String::from("Byte")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i32"),
                size: 4,
                signed: true,
            }),
            String::from("Int")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("i64"),
                size: 8,
                signed: true,
            }),
            String::from("Long")
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
            String::from("UByte")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("u32"),
                size: 4,
                signed: false,
            }),
            String::from("UInt")
        );
        assert_eq!(
            generate_type_id(&TypeIDASTNode::Integer {
                id: String::from("u64"),
                size: 8,
                signed: false,
            }),
            String::from("ULong")
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
            String::from("Boolean")
        );
    }

    #[test]
    fn generate_const_value_test_string() {
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Other {
                    id: String::from("String")
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::StringLiteral(String::from("Hello World!")),
                    type_id: TypeIDASTNode::Other {
                        id: String::from("String")
                    }
                },
            ),
            String::from("\"Hello World!\"")
        );
    }

    #[test]
    fn generate_const_value_test_int8() {
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Integer {
                    id: String::from("i8"),
                    size: 1,
                    signed: true
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("i8"),
                        size: 1,
                        signed: true
                    }
                },
            ),
            String::from("13")
        );
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Integer {
                    id: String::from("u8"),
                    size: 1,
                    signed: false
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("u8"),
                        size: 1,
                        signed: false
                    }
                },
            ),
            String::from("13U")
        );
    }

    #[test]
    fn generate_const_value_test_int32() {
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Integer {
                    id: String::from("i32"),
                    size: 4,
                    signed: true
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("i32"),
                        size: 4,
                        signed: true
                    }
                },
            ),
            String::from("13")
        );
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Integer {
                    id: String::from("u32"),
                    size: 4,
                    signed: false
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("u32"),
                        size: 4,
                        signed: false
                    }
                },
            ),
            String::from("13U")
        );
    }

    #[test]
    fn generate_const_value_test_int64() {
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Integer {
                    id: String::from("i64"),
                    size: 8,
                    signed: true
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("i64"),
                        size: 8,
                        signed: true
                    }
                },
            ),
            String::from("13L")
        );
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Integer {
                    id: String::from("u64"),
                    size: 8,
                    signed: false
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("u64"),
                        size: 8,
                        signed: false
                    }
                },
            ),
            String::from("13UL")
        );
    }

    #[test]
    fn generate_const_value_test_float32() {
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Number {
                    id: String::from("f32"),
                    size: 4,
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::NumberLiteral(3.14),
                    type_id: TypeIDASTNode::Number {
                        id: String::from("f32"),
                        size: 4,
                    }
                },
            ),
            String::from("3.14f")
        );
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Number {
                    id: String::from("f32"),
                    size: 4,
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::NumberLiteral(314.0),
                    type_id: TypeIDASTNode::Number {
                        id: String::from("f32"),
                        size: 4,
                    }
                },
            ),
            String::from("314.0f")
        );
    }

    #[test]
    fn generate_const_value_test_float64() {
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Number {
                    id: String::from("f64"),
                    size: 8,
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::NumberLiteral(3.14),
                    type_id: TypeIDASTNode::Number {
                        id: String::from("f64"),
                        size: 8,
                    }
                },
            ),
            String::from("3.14")
        );
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Number {
                    id: String::from("f64"),
                    size: 8,
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::NumberLiteral(314.0),
                    type_id: TypeIDASTNode::Number {
                        id: String::from("f64"),
                        size: 8,
                    }
                },
            ),
            String::from("314.0")
        );
    }

    #[test]
    fn generate_const_value_test_boolean() {
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Bool {
                    id: String::from("Bool")
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::BoolLiteral(true),
                    type_id: TypeIDASTNode::Bool {
                        id: String::from("Bool")
                    }
                },
            ),
            String::from("true")
        );
        assert_eq!(
            generate_const_value(
                &TypeIDASTNode::Bool {
                    id: String::from("Bool")
                },
                &ConstValueASTNode::Literal {
                    literal: Literal::BoolLiteral(false),
                    type_id: TypeIDASTNode::Bool {
                        id: String::from("Bool")
                    }
                },
            ),
            String::from("false")
        );
    }
}
