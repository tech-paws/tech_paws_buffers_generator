use convert_case::{Case, Casing};
use strum_macros::IntoStaticStr;

use crate::{
    ast::{ConstValueASTNode, TypeIDASTNode},
    lexer::Literal,
    writer::Writer,
};

#[derive(Clone, Debug, PartialEq, IntoStaticStr)]
pub enum SwiftIR {
    Struct {
        id: String,
        body: Vec<SwiftIR>,
        extends: Vec<SwiftIR>,
    },
    Gap,
    Id(String),
    TypeId(TypeIDASTNode),
    List {
        items: Vec<SwiftIR>,
        separator: &'static str,
        new_line: bool,
    },
    Statements {
        items: Vec<SwiftIR>,
    },
    TopLevelDeclarations {
        items: Vec<SwiftIR>,
    },
    ForLoop {
        item: Option<Box<SwiftIR>>,
        collection_expr: Box<SwiftIR>,
        body: Box<SwiftIR>,
    },
    Switch {
        item: Box<SwiftIR>,
        body: Box<SwiftIR>,
    },
    Case {
        item: Box<SwiftIR>,
        body: Box<SwiftIR>,
    },
    DefaultCase {
        body: Box<SwiftIR>,
    },
    Range {
        from: Box<SwiftIR>,
        to: Box<SwiftIR>,
    },
    Enum {
        id: String,
        body: Vec<SwiftIR>,
        extends: Vec<SwiftIR>,
    },
    EnumCase {
        id: String,
        parameters: Vec<SwiftIR>,
    },
    EnumCaseType {
        id: Option<String>,
        type_id: TypeIDASTNode,
    },
    FieldAccess {
        instance: Option<Box<SwiftIR>>,
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
    Func {
        id: String,
        is_static: bool,
        return_type_id: Option<Box<SwiftIR>>,
        arguments: Option<Box<SwiftIR>>,
        body: Option<Box<SwiftIR>>,
    },
    VarDeclaration {
        id: String,
        is_const: bool,
        type_id: Option<Box<SwiftIR>>,
        value: Option<Box<SwiftIR>>,
    },
    StaticVarDeclaration {
        id: String,
        is_const: bool,
        is_private: bool,
        is_private_set: bool,
        type_id: Option<Box<SwiftIR>>,
        value: Option<Box<SwiftIR>>,
    },
    FunctionArgument {
        id: String,
        named: bool,
        type_id: Box<SwiftIR>,
    },
    ReturnStatement {
        body: Box<SwiftIR>,
    },
    Continue,
    AssignStructNamedArgument {
        id: String,
        default_value_type_id: Option<TypeIDASTNode>,
        value: Option<Box<SwiftIR>>,
    },
    AssignArgument {
        id: Option<String>,
        default_value_type_id: TypeIDASTNode,
        value: Option<Box<SwiftIR>>,
    },
    SetVar {
        id: String,
        value: Box<SwiftIR>,
    },
    Call {
        id: String,
        arguments: Option<Box<SwiftIR>>,
    },
    ChainCalls {
        items: Vec<SwiftIR>,
    },
    TrailingCall {
        id: String,
        arguments: Option<Box<SwiftIR>>,
        input: Option<Box<SwiftIR>>,
        body: Box<SwiftIR>,
    },
    NamedBlock {
        id: String,
        body: Box<SwiftIR>,
    },
}

pub fn stringify_ir(tokens: &[SwiftIR]) -> String {
    let mut writer = Writer::default();
    write_tokens(&mut writer, tokens);
    writer.show().to_string()
}

pub fn write_tokens_separated(writer: &mut Writer, tokens: &[SwiftIR], separator: &'static str) {
    let mut it = tokens.iter().peekable();

    while let Some(token) = it.next() {
        write_token(writer, token);

        if it.peek().is_some() {
            writer.write(separator);
        }
    }
}

pub fn write_tokens_comma_separated(writer: &mut Writer, tokens: &[SwiftIR]) {
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

pub fn write_tokens(writer: &mut Writer, tokens: &[SwiftIR]) {
    let mut last_token: Option<&SwiftIR> = None;

    for token in tokens {
        let gaps_pairs = vec![
            ("Struct", "Struct"),
            ("Struct", "Enum"),
            ("Enum", "Enum"),
            ("Struct", "StructConstField"),
            ("Struct", "StructField"),
            ("Func", "Func"),
            ("Func", "StructConstField"),
            ("Func", "StructField"),
            ("Func", "EnumCase"),
            ("Func", "TopLevelDeclarations"),
            ("TopLevelDeclarations", "TopLevelDeclarations"),
            ("Statements", "Statements"),
            ("Statements", "ReturnStatement"),
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

fn write_token(writer: &mut Writer, token: &SwiftIR) {
    match token {
        SwiftIR::Id(id) => writer.write(id),
        SwiftIR::Gap => (),
        SwiftIR::TypeId(type_id) => writer.write(&generate_type_id(type_id)),
        SwiftIR::List {
            items,
            separator,
            new_line,
        } => {
            let mut it = items.iter().peekable();

            if *new_line && !items.is_empty() {
                writer.new_line();
                writer.push_tab();
            }

            while let Some(item) = it.next() {
                if *new_line {
                    writer.write_tabs();
                }

                write_token(writer, item);

                if it.peek().is_some() {
                    writer.write(separator);

                    if *new_line {
                        writer.new_line();
                    }
                }
            }

            if *new_line && !items.is_empty() {
                writer.new_line();
                writer.pop_tab();
                writer.write_tabs();
            }
        }
        SwiftIR::TopLevelDeclarations { items } => {
            for item in items {
                writer.write_tabs();

                // NOTE(sysint64): Removing trailing spaces when Gap is used.
                match item {
                    SwiftIR::Gap => {}
                    _ => {
                        writer.write_tabs();
                        write_token(writer, item);
                    }
                }

                writer.new_line();
            }
        }
        SwiftIR::Statements { items } => {
            let mut it = items.iter().peekable();

            while let Some(item) = it.next() {
                // NOTE(sysint64): Removing trailing spaces when Gap is used.
                match item {
                    SwiftIR::Gap => {}
                    _ => {
                        writer.write_tabs();
                        write_token(writer, item);
                    }
                }

                if it.peek().is_some() {
                    writer.new_line();
                }
            }
        }
        SwiftIR::Range { from, to } => {
            write_token(writer, from);
            writer.write("...");
            write_token(writer, to);
        }
        SwiftIR::ForLoop {
            item,
            collection_expr,
            body,
        } => {
            writer.write("for ");

            if let Some(item) = item {
                write_token(writer, item);
            } else {
                writer.write("_");
            }

            writer.write(" in ");
            write_token(writer, collection_expr);

            writer.write(" {");
            writer.new_line();

            writer.push_tab();
            write_token(writer, body);
            writer.pop_tab();
            writer.new_line();
            writer.write_tabs();
            writer.write("}");
        }
        SwiftIR::Switch { item, body } => {
            writer.write("switch ");
            write_token(writer, item);

            writer.write(" {");
            writer.new_line();
            write_token(writer, body);
            writer.new_line();
            writer.write_tabs();
            writer.write("}");
        }
        SwiftIR::Case { item, body } => {
            writer.write("case ");
            write_token(writer, item);

            writer.write(":");
            writer.new_line();
            writer.push_tab();
            write_token(writer, body);
            writer.pop_tab();
        }
        SwiftIR::DefaultCase { body } => {
            writer.write("default:");
            writer.new_line();
            writer.push_tab();
            write_token(writer, body);
            writer.pop_tab();
        }
        SwiftIR::VarDeclaration {
            id,
            is_const,
            type_id,
            value,
        } => {
            if *is_const {
                writer.write("let ");
            } else {
                writer.write("var ");
            }

            writer.write(id);

            if let Some(type_id) = type_id {
                writer.write(": ");
                write_token(writer, type_id);
            }

            if let Some(value) = value {
                writer.write(" = ");
                write_token(writer, value);
            }
        }
        SwiftIR::StaticVarDeclaration {
            id,
            is_const,
            is_private,
            is_private_set,
            type_id,
            value,
        } => {
            if *is_private {
                if *is_private_set {
                    writer.write("private(set) ");
                } else {
                    writer.write("private ");
                }
            }

            writer.write("static ");

            if *is_const {
                writer.write("let ");
            } else {
                writer.write("var ");
            }

            writer.write(id);

            if let Some(type_id) = type_id {
                writer.write(": ");
                write_token(writer, type_id);
            }

            if let Some(value) = value {
                writer.write(" = ");
                write_token(writer, value);
            }
        }
        SwiftIR::Struct { id, body, extends } => {
            writer.write_tabs();
            writer.write(&format!("struct {}", id));

            if !extends.is_empty() {
                writer.write(": ");
                write_tokens_separated(writer, extends, ", ");
            }

            writer.write(" {");
            writer.new_line();
            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();
            writer.writeln("}");
        }
        SwiftIR::StructConstField { id, type_id, value } => {
            writer.writeln(&format!(
                "static let {}: {} = {}",
                id.to_case(Case::Camel),
                generate_type_id(type_id),
                generate_const_value(value, type_id)
            ));
        }
        SwiftIR::StructField { id, type_id } => writer.writeln(&format!(
            "let {}: {}",
            id.to_case(Case::Camel),
            generate_type_id(type_id),
        )),
        SwiftIR::Func {
            id,
            is_static,
            return_type_id,
            arguments,
            body,
        } => {
            writer.write_tabs();

            if *is_static {
                writer.write("static ");
            }

            writer.write(&format!("func {}(", id));

            if let Some(arguments) = arguments {
                write_token(writer, arguments);
            }

            if let Some(return_type_id) = return_type_id {
                writer.write(") -> ");
                write_token(writer, return_type_id);
            } else {
                writer.write(")");
            }

            writer.write(" {");
            writer.new_line();

            if let Some(body) = body {
                writer.push_tab();
                write_token(writer, body);
                writer.pop_tab();

                match body.as_ref() {
                    SwiftIR::Statements { items } => {
                        if !items.is_empty() {
                            writer.new_line();
                        }
                    }
                    _ => {
                        writer.new_line();
                    }
                }
            }

            writer.writeln("}");
        }
        SwiftIR::FunctionArgument { id, named, type_id } => {
            if !named {
                writer.write("_ ");
            }

            writer.write(id);
            writer.write(": ");
            write_token(writer, type_id);
        }
        SwiftIR::ReturnStatement { body } => {
            writer.write("return ");
            write_token(writer, body);
        }
        SwiftIR::Continue => {
            writer.write("continue");
        }
        SwiftIR::AssignStructNamedArgument {
            id,
            default_value_type_id,
            value,
        } => {
            if let Some(value) = value {
                writer.write(&format!("{}: ", id.to_case(Case::Camel),));
                write_token(writer, value);
            } else if let Some(default_value_type_id) = default_value_type_id {
                writer.write(&format!(
                    "{}: {}",
                    id.to_case(Case::Camel),
                    generate_default_const_value(default_value_type_id)
                ));
            } else {
                panic!("value or default_value_type_id should be specified");
            }
        }
        SwiftIR::SetVar { id, value } => {
            writer.write(&format!("{} = ", id));
            write_token(writer, value);
        }
        SwiftIR::AssignArgument {
            id,
            default_value_type_id: type_id,
            value,
        } => {
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
        SwiftIR::Call { id, arguments } => {
            writer.write(id);

            if let Some(arguments) = arguments {
                writer.write("(");
                write_token(writer, arguments);
                writer.write(")");
            } else {
                writer.write("()");
            }
        }
        SwiftIR::ChainCalls { items } => {
            writer.push_tab();
            let mut it = items.iter().peekable();

            while let Some(item) = it.next() {
                // NOTE(sysint64): Removing trailing spaces when Gap is used.
                match item {
                    SwiftIR::Gap => {}
                    _ => {
                        write_token(writer, item);
                    }
                }

                if it.peek().is_some() {
                    writer.new_line();
                    writer.write_tabs();
                }
            }
            writer.pop_tab();
        }
        SwiftIR::NamedBlock { id, body } => {
            writer.write(id);
            writer.write(" {");
            writer.new_line();

            writer.push_tab();
            write_token(writer, body);
            writer.pop_tab();

            match body.as_ref() {
                SwiftIR::Statements { items } => {
                    if !items.is_empty() {
                        writer.new_line();
                    }
                }
                _ => {
                    writer.new_line();
                }
            }

            writer.write_tabs();
            writer.write("}");
        }
        SwiftIR::TrailingCall {
            id,
            arguments,
            input,
            body,
        } => {
            writer.write(id);

            if let Some(arguments) = arguments {
                writer.write("(");
                write_token(writer, arguments);
                writer.write(")");
            } else {
                writer.write("()");
            }

            writer.write(" {");

            if let Some(input) = input {
                writer.write(" ");
                write_token(writer, input);
                writer.write(" in");
            }

            writer.new_line();

            writer.push_tab();
            write_token(writer, body);
            writer.pop_tab();

            match body.as_ref() {
                SwiftIR::Statements { items } => {
                    if !items.is_empty() {
                        writer.new_line();
                    }
                }
                _ => {
                    writer.new_line();
                }
            }

            writer.write_tabs();
            writer.write("}");
        }
        SwiftIR::Enum { id, body, extends } => {
            writer.write_tabs();
            writer.write(&format!("enum {}", id.to_case(Case::Pascal)));

            if !extends.is_empty() {
                writer.write(": ");
                write_tokens_separated(writer, extends, ", ");
            }

            writer.write(" {");
            writer.new_line();
            writer.push_tab();
            write_tokens(writer, body);
            writer.pop_tab();
            writer.writeln("}");
            // writer.writeln(&format!("enum {} {{", id.to_case(Case::Pascal)));
            // writer.push_tab();
            // write_tokens(writer, body);
            // writer.pop_tab();
            // writer.writeln("}");
        }
        SwiftIR::EnumCase { id, parameters } => {
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
        SwiftIR::EnumCaseType { id, type_id } => {
            if let Some(id) = id {
                writer.write(&format!("/* {} */ {}", id, generate_type_id(type_id)));
            } else {
                writer.write(&generate_type_id(type_id));
            }
        }
        SwiftIR::FieldAccess { instance, field } => {
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
            "Option" => format!(
                "{}?",
                generics
                    .iter()
                    .map(generate_type_id)
                    .collect::<Vec<String>>()
                    .first()
                    .expect("Optional type cannot be empty")
            ),
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
            _ => format!("{}.createBuffersDefault()", id),
        },
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Option" => String::from("nil"),
            "Vec" => String::from("[]"),
            _ => format!(
                "{}<{}>.createBuffersDefault()",
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
