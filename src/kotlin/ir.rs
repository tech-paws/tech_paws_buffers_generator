use convert_case::{Case, Casing};
use strum_macros::IntoStaticStr;

use crate::{
    ast::{ConstValueASTNode, TypeIDASTNode},
    lexer::Literal,
    writer::Writer,
};

#[derive(Clone, Debug, PartialEq, IntoStaticStr)]
pub enum KotlinIR {
    Object {
        id: String,
        body: Vec<KotlinIR>,
        extends: Vec<KotlinIR>,
    },
    Class {
        id: String,
        is_data_class: bool,
        extends: Vec<KotlinIR>,
        fields: Vec<KotlinIR>,
        body: Vec<KotlinIR>,
    },
    Interface {
        id: String,
        is_sealed: bool,
        body: Vec<KotlinIR>,
    },
    Id(String),
    ValDeclaration {
        id: String,
        is_const: bool,
        type_id: Box<KotlinIR>,
        value: Option<Box<KotlinIR>>,
    },
    Declaration {
        body: Box<KotlinIR>,
        separator: Option<&'static str>,
    },
    TypeId(TypeIDASTNode),
    ConstValueExpr {
        type_id: TypeIDASTNode,
        value: ConstValueASTNode,
    },
    DefaulConstValueExpr(TypeIDASTNode),
    CompanionObject {
        body: Vec<KotlinIR>,
    },
    Gap,
    FunInline {
        id: String,
        arguments: Vec<KotlinIR>,
        return_type_id: Box<KotlinIR>,
        body: Box<KotlinIR>,
    },
    Fun {
        id: String,
        arguments: Vec<KotlinIR>,
        return_type_id: Box<KotlinIR>,
        body: Vec<KotlinIR>,
    },
    Call {
        id: String,
        arguments: Vec<KotlinIR>,
    },
    AssignArgument {
        id: String,
        value: Box<KotlinIR>,
    },
}

pub fn stringify_tokens(tokens: &[KotlinIR]) -> String {
    let mut writer = Writer::default();
    write_tokens(&mut writer, tokens);
    writer.show().to_string()
}

pub fn write_tokens_separated(writer: &mut Writer, tokens: &[KotlinIR], separator: &'static str) {
    let mut it = tokens.iter().peekable();

    while let Some(token) = it.next() {
        write_token(writer, token);

        if it.peek().is_some() {
            writer.write(separator);
        }
    }
}

pub fn write_tokens(writer: &mut Writer, tokens: &[KotlinIR]) {
    let mut last_token: Option<&KotlinIR> = None;

    for token in tokens {
        let gaps_pairs = vec![
            ("Object", "Object"),
            ("Object", "Declaration"),
            ("Object", "Class"),
            ("Class", "Declaration"),
            ("Class", "Class"),
            ("Class", "Interface"),
            ("Object", "Interface"),
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

fn write_token(writer: &mut Writer, token: &KotlinIR) {
    match token {
        KotlinIR::Gap => writer.new_line(),
        KotlinIR::Id(id) => writer.write(id),
        KotlinIR::Object { id, extends, body } => {
            writer.write_tabs();
            writer.write(&format!("object {}", id.to_case(Case::Pascal)));

            if !extends.is_empty() {
                writer.write(" : ");
                write_tokens_separated(writer, extends, ", ");
            }

            if !body.is_empty() {
                writer.write(" {");
                writer.new_line();
                writer.push_tab();
                write_tokens(writer, body);
                writer.pop_tab();
                writer.writeln("}");
            } else {
                writer.new_line();
            }
            // writer.write(" {")

            // writer.push_tab();
            // write_tokens(writer, body);
            // writer.pop_tab();
            // writer.writeln("}");
        }
        KotlinIR::Declaration { body, separator } => {
            writer.write_tabs();
            write_token(writer, body);

            if let Some(separator) = separator {
                writer.write(separator);
            }

            writer.new_line();
        }
        KotlinIR::ValDeclaration {
            id,
            is_const,
            type_id,
            value,
        } => {
            if *is_const {
                writer.write("const ");
            }

            writer.write(&format!("val {}", id));
            writer.write(": ");
            write_token(writer, type_id);

            if let Some(value) = value {
                writer.write(" = ");
                write_token(writer, value);
            }
        }
        KotlinIR::TypeId(type_id) => writer.write(&generate_type_id(type_id)),
        KotlinIR::ConstValueExpr { type_id, value } => {
            writer.write(&generate_const_value(type_id, value))
        }
        KotlinIR::DefaulConstValueExpr(type_id) => {
            writer.write(&generate_default_const_value(type_id));
        }
        KotlinIR::Class {
            id,
            is_data_class,
            extends,
            fields,
            body,
        } => {
            writer.write_tabs();

            if *is_data_class {
                writer.write("data ");
            }

            writer.write(&format!("class {}(", id.to_case(Case::Pascal)));

            if fields.is_empty() {
                writer.write(")");
            } else {
                writer.new_line();
                writer.push_tab();
                write_tokens(writer, fields);
                writer.pop_tab();
                writer.write(")");
            }

            if !extends.is_empty() {
                writer.write(" : ");
                write_tokens_separated(writer, extends, ", ");
            }

            if !body.is_empty() {
                writer.write(" {");
                writer.new_line();
                writer.push_tab();
                write_tokens(writer, body);
                writer.pop_tab();
                writer.writeln("}");
            } else {
                writer.new_line();
            }
        }
        KotlinIR::Interface {
            id,
            is_sealed,
            body,
        } => {
            writer.write_tabs();

            if *is_sealed {
                writer.write("sealed ");
            }

            writer.writeln(&format!("interface {} {{", id));

            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();
            writer.writeln("}");
        }
        KotlinIR::CompanionObject { body } => {
            writer.writeln("companion object {");
            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();
            writer.writeln("}");
        }
        KotlinIR::FunInline {
            id,
            arguments,
            return_type_id,
            body,
        } => {
            writer.write_tabs();
            writer.write(&format!("fun {}(", id));

            if !arguments.is_empty() {
                writer.new_line();
                writer.push_tab();
                write_tokens(writer, arguments);
                writer.pop_tab();
                writer.new_line();
            }

            writer.write("): ");
            write_token(writer, return_type_id);
            writer.write(" = ");

            write_token(writer, body);
            writer.new_line();
        }
        KotlinIR::Fun {
            id,
            arguments,
            return_type_id,
            body,
        } => {
            writer.write_tabs();
            writer.write(&format!("fun {}(", id));

            if !arguments.is_empty() {
                writer.new_line();
                writer.push_tab();
                write_tokens(writer, arguments);
                writer.pop_tab();
                writer.new_line();
            }

            writer.write("): ");
            write_token(writer, return_type_id);
            writer.write(" {");
            writer.new_line();

            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();

            writer.new_line();
        }
        KotlinIR::Call { id, arguments } => {
            writer.write(id);

            if arguments.is_empty() {
                writer.write("()");
            } else {
                writer.write("(");
                writer.new_line();
                writer.push_tab();
                write_tokens(writer, arguments);
                writer.pop_tab();
                writer.write_tabs();
                writer.write(")");
            }
        }
        KotlinIR::AssignArgument { id, value } => {
            writer.write(&format!("{} = ", id.to_case(Case::Camel),));
            write_token(writer, value);
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
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Vec" => format!(
                "List<{}>",
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

pub fn generate_default_const_value(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer { size, signed, .. } => match size {
            1 if *signed => String::from("0"),
            4 if *signed => String::from("0"),
            8 if *signed => String::from("0L"),
            1 if !*signed => String::from("0U"),
            4 if !*signed => String::from("0U"),
            8 if !*signed => String::from("0UL"),
            _ => panic!("Unsupported integer size"),
        },
        TypeIDASTNode::Number { id: _, size } => match size {
            4 => String::from("0f"),
            8 => String::from("0.0"),
            _ => panic!("Unsupported integer size"),
        },
        TypeIDASTNode::Bool { id: _ } => String::from("false"),
        TypeIDASTNode::Char { id: _ } => String::from("0"),
        TypeIDASTNode::Other { id } => match id.as_str() {
            "String" => String::from("\"\""),
            "Vec" => String::from("listOf()"),
            _ => format!("{}.createDefault()", id),
        },
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Vec" => String::from("listOf()"),
            _ => format!(
                "{}.createDefault<{}>()",
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
