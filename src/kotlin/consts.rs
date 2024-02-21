use convert_case::{Case, Casing};

use crate::{
    ast::{ASTNode, ConstASTNode, ConstItemASTNode, ConstValueASTNode, TypeIDASTNode},
    lexer::Literal,
    writer::Writer,
};

use super::types::generate_type_id;

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

pub fn generate_consts(ast: &[ASTNode]) -> String {
    let mut writer = Writer::default();

    for node in ast {
        if let ASTNode::Const(node) = node {
            writer.writeln(&generate_const_block(0, node))
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_const_block(tab: usize, const_node: &ConstASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln_tab(
        tab,
        &format!("object {} {{", const_node.id.to_case(Case::Pascal)),
    );

    let mut is_first = true;
    let mut is_value = false;

    for item in &const_node.items {
        match &item {
            ConstItemASTNode::Value { id, type_id, value } => {
                if !is_value && !is_first {
                    writer.writeln("");
                }

                is_value = true;

                let const_value = generate_const_value(value, type_id);
                let generated_type_id = generate_type_id(type_id);

                let (type_id, value) = match type_id {
                    TypeIDASTNode::Other { id } => match id.as_str() {
                        "GroupAddress" => (String::from("ULong"), format!("{}UL", const_value)),
                        "CommandsBufferAddress" => {
                            (String::from("ULong"), format!("{}UL", const_value))
                        }
                        _ => (generated_type_id, const_value),
                    },
                    _ => (generated_type_id, const_value),
                };

                writer.writeln_tab(
                    tab + 1,
                    &format!("const val {}: {} = {}", id, type_id, value),
                );
            }
            ConstItemASTNode::ConstNode { node } => {
                if is_value && !is_first {
                    writer.writeln("");
                }

                writer.write(&generate_const_block(tab + 1, node));
                is_value = false;
            }
        }

        is_first = false;
    }

    writer.writeln_tab(tab, "}");

    writer.show().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{lexer::Lexer, parser::parse};

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
    fn generate_const_value_test_int8() {
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("i8"),
                        size: 1,
                        signed: true
                    }
                },
                &TypeIDASTNode::Integer {
                    id: String::from("i8"),
                    size: 1,
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
                        id: String::from("u8"),
                        size: 1,
                        signed: false
                    }
                },
                &TypeIDASTNode::Integer {
                    id: String::from("u8"),
                    size: 1,
                    signed: false
                }
            ),
            String::from("13U")
        );
    }

    #[test]
    fn generate_const_value_test_int32() {
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::IntLiteral(13),
                    type_id: TypeIDASTNode::Integer {
                        id: String::from("i32"),
                        size: 4,
                        signed: true
                    }
                },
                &TypeIDASTNode::Integer {
                    id: String::from("i32"),
                    size: 4,
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
                        id: String::from("u32"),
                        size: 4,
                        signed: false
                    }
                },
                &TypeIDASTNode::Integer {
                    id: String::from("u32"),
                    size: 4,
                    signed: false
                },
            ),
            String::from("13U")
        );
    }

    #[test]
    fn generate_const_value_test_int64() {
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
            String::from("13L")
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
            String::from("13UL")
        );
    }

    #[test]
    fn generate_const_value_test_float32() {
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::NumberLiteral(3.14),
                    type_id: TypeIDASTNode::Number {
                        id: String::from("f32"),
                        size: 4,
                    }
                },
                &TypeIDASTNode::Number {
                    id: String::from("f32"),
                    size: 4,
                }
            ),
            String::from("3.14f")
        );
        assert_eq!(
            generate_const_value(
                &ConstValueASTNode::Literal {
                    literal: Literal::NumberLiteral(314.0),
                    type_id: TypeIDASTNode::Number {
                        id: String::from("f32"),
                        size: 4,
                    }
                },
                &TypeIDASTNode::Number {
                    id: String::from("f32"),
                    size: 4,
                }
            ),
            String::from("314.0f")
        );
    }

    #[test]
    fn generate_const_value_test_float64() {
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

    #[test]
    fn generate_consts_test() {
        let src = fs::read_to_string("test_resources/consts.tpb").unwrap();
        let target = fs::read_to_string("test_resources/kotlin/consts.kt").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_consts(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }
}
