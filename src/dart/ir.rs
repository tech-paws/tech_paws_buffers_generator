use strum_macros::IntoStaticStr;

use crate::{
    ast::{ConstValueASTNode, TypeIDASTNode},
    lexer::Literal,
    writer::Writer,
};

#[derive(Clone, Debug, PartialEq, IntoStaticStr)]
pub enum DartIR {
    Id(String),
    CopyTypeId(TypeIDASTNode),
    EmplaceTypeId(TypeIDASTNode),
    Class(ClassDartIR),
    DefaultConstructor(DefaultConstructorIR),
    NamedConstructor(NamedConstructorIR),
    ShortFunc(ShortFuncIR),
    Call(CallIR),
    List(ListIR),
    VarDeclaration(VarDeclarationIR),
    Assign(AssignIR),
    DefaultCopyValueForTypeID(TypeIDASTNode),
    DefaultEmplaceValueForTypeID(TypeIDASTNode),
    ArgumentDeclaration(ArgumentDeclarationIR),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgumentDeclarationIR {
    pub id: String,
    pub is_required: bool,
    pub is_this: bool,
    pub type_id: Option<Box<DartIR>>,
    pub assign: Option<Box<DartIR>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignIR {
    pub left: Box<DartIR>,
    pub right: Box<DartIR>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDeclarationIR {
    pub id: String,
    pub type_id: Box<DartIR>,
    pub is_final: bool,
    pub assign: Option<Box<DartIR>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListIR {
    pub items: Vec<DartIR>,
    pub separator: &'static str,
    pub new_line: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallIR {
    pub path: Vec<DartIR>,
    pub is_const: bool,
    pub args: Option<Box<DartIR>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShortFuncIR {
    pub id: String,
    pub args: Option<Box<DartIR>>,
    pub return_type_id: Option<Box<DartIR>>,
    pub is_override: bool,
    pub body: Box<DartIR>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassDartIR {
    pub id: String,
    pub body: Vec<DartIR>,
    pub implements: Vec<DartIR>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefaultConstructorIR {
    pub id: String,
    pub is_const: bool,
    pub fields: Option<Box<DartIR>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedConstructorIR {
    pub id: String,
    pub name: String,
    pub is_const: bool,
    pub fields: Option<Box<DartIR>>,
    pub assigns: Vec<DartIR>,
}

pub fn stringify_ir(tokens: &[DartIR]) -> String {
    let mut writer = Writer::new(2);
    write_tokens(&mut writer, tokens);
    writer.show().to_string()
}

pub fn write_tokens_separated(writer: &mut Writer, tokens: &[DartIR], separator: &'static str) {
    let mut it = tokens.iter().peekable();

    while let Some(token) = it.next() {
        write_token(writer, token);

        if it.peek().is_some() {
            writer.write(separator);
        }
    }
}

pub fn write_tokens(writer: &mut Writer, tokens: &[DartIR]) {
    let mut last_token: Option<&DartIR> = None;

    for token in tokens {
        let gaps_pairs = vec![
            ("Class", "Class"),
            ("DefaultConstructor", "ShortFunc"),
            ("DefaultConstructor", "Func"),
            ("DefaultConstructor", "NamedConstructor"),
            ("NamedConstructor", "ShortFunc"),
            ("NamedConstructor", "Func"),
            ("ShortFunc", "ShortFunc"),
            ("ShortFunc", "Func"),
            ("Func", "Func"),
            ("DefaultConstructor", "VarDeclaration"),
            ("NamedConstructor", "VarDeclaration"),
            ("Func", "VarDeclaration"),
            ("ShortFunc", "VarDeclaration"),
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

fn write_token(writer: &mut Writer, token: &DartIR) {
    match token {
        DartIR::Id(id) => writer.write(id),
        DartIR::CopyTypeId(type_id) => writer.write(&generate_copy_type_id(type_id)),
        DartIR::EmplaceTypeId(type_id) => writer.write(&generate_emplace_type_id(type_id)),
        DartIR::Class(ir) => write_class(writer, ir),
        DartIR::DefaultConstructor(ir) => write_default_constructor(writer, ir),
        DartIR::NamedConstructor(ir) => write_named_constructor(writer, ir),
        DartIR::ShortFunc(ir) => write_short_func(writer, ir),
        DartIR::Call(ir) => write_call(writer, ir),
        DartIR::List(ir) => write_list(writer, ir),
        DartIR::VarDeclaration(ir) => write_var_declaration(writer, ir),
        DartIR::Assign(ir) => write_asign(writer, ir),
        DartIR::DefaultCopyValueForTypeID(ir) => write_default_copy_value_for_type_id(writer, ir),
        DartIR::DefaultEmplaceValueForTypeID(ir) => {
            write_default_emplace_value_for_type_id(writer, ir)
        }
        DartIR::ArgumentDeclaration(ir) => write_argument_declaration(writer, ir),
    }
}

pub fn write_argument_declaration(writer: &mut Writer, ir: &ArgumentDeclarationIR) {
    if ir.is_required {
        writer.write("required ");
    }

    if let Some(type_id) = &ir.type_id {
        write_token(writer, type_id);
        writer.write(" ");
    }

    if ir.is_this {
        writer.write("this.");
    }

    writer.write(&ir.id);

    if let Some(assign) = &ir.assign {
        writer.write(" = ");
        write_token(writer, assign);
    }
}

pub fn write_default_copy_value_for_type_id(writer: &mut Writer, type_id: &TypeIDASTNode) {
    writer.write(&generate_default_copy_const_value(type_id));
}

pub fn write_default_emplace_value_for_type_id(writer: &mut Writer, type_id: &TypeIDASTNode) {
    writer.write(&generate_default_emplace_const_value(type_id));
}

pub fn write_asign(writer: &mut Writer, ir: &AssignIR) {
    write_token(writer, &ir.left);
    writer.write(" = ");
    write_token(writer, &ir.right);
}

pub fn write_var_declaration(writer: &mut Writer, ir: &VarDeclarationIR) {
    writer.write_tabs();

    if ir.is_final {
        writer.write("final ");
    }

    write_token(writer, &ir.type_id);
    writer.write(" ");
    writer.write(&ir.id);
    writer.write(";");

    writer.new_line();
}

pub fn write_list(writer: &mut Writer, ir: &ListIR) {
    let mut it = ir.items.iter().peekable();

    if ir.new_line && !ir.items.is_empty() {
        writer.new_line();
        writer.push_tab();
    }

    while let Some(item) = it.next() {
        if ir.new_line {
            writer.write_tabs();
        }

        write_token(writer, item);

        if it.peek().is_some() && ir.new_line {
            writer.write(ir.separator);
            writer.new_line();
        } else if ir.new_line {
            writer.write(ir.separator);
        }
    }

    if ir.new_line && !ir.items.is_empty() {
        writer.new_line();
        writer.pop_tab();
        writer.write_tabs();
    }
}

pub fn write_call(writer: &mut Writer, ir: &CallIR) {
    if ir.is_const {
        writer.write("const ");
    }

    write_tokens_separated(writer, &ir.path, ".");

    if let Some(args) = &ir.args {
        writer.write("(");
        write_token(writer, args);
        writer.write(")");
    } else {
        writer.write("()");
    }
}

pub fn write_short_func(writer: &mut Writer, ir: &ShortFuncIR) {
    writer.write_tabs();

    if ir.is_override {
        writer.write("@override");
        writer.new_line();
        writer.write_tabs();
    }

    if let Some(return_type_id) = &ir.return_type_id {
        write_token(writer, return_type_id);
        writer.write(" ");
    } else {
        writer.write("void ");
    }

    writer.write(&ir.id);

    if let Some(args) = &ir.args {
        writer.write("(");
        write_token(writer, args);
        writer.write(")");
    } else {
        writer.write("()");
    }

    writer.write(" => ");
    write_token(writer, &ir.body);
    writer.write(";");

    writer.new_line();
}

pub fn write_default_constructor(writer: &mut Writer, ir: &DefaultConstructorIR) {
    writer.write_tabs();

    if ir.is_const {
        writer.write("const ");
    }

    if let Some(fields) = &ir.fields {
        writer.write(&format!("{}({{", ir.id));
        write_token(writer, fields);
        writer.write("});");
    } else {
        writer.write(&format!("{}();", ir.id));
    }

    writer.new_line();
}

pub fn write_named_constructor(writer: &mut Writer, ir: &NamedConstructorIR) {
    writer.write_tabs();

    if ir.is_const {
        writer.write("const ");
    }

    writer.write(&format!("{}.{}()", ir.id, ir.name));

    if !ir.assigns.is_empty() {
        writer.new_line();
        writer.push_tab();
        writer.push_tab();

        writer.write_tabs();
        writer.write(": ");

        let mut it = ir.assigns.iter().peekable();

        while let Some(item) = it.next() {
            write_token(writer, item);

            if it.peek().is_some() {
                writer.write(",");
                writer.new_line();
                writer.write_tabs();
                writer.write("  ");
            }
        }

        writer.write(";");

        writer.pop_tab();
        writer.pop_tab();
    } else {
        writer.write(";");
    }

    writer.new_line();
}

fn write_class(writer: &mut Writer, ir: &ClassDartIR) {
    writer.write_tabs();
    writer.write(&format!("class {}", ir.id));

    if !ir.implements.is_empty() {
        writer.write(" implements ");
        write_tokens_separated(writer, &ir.implements, ", ");
    }

    writer.write(" {");
    writer.new_line();
    writer.push_tab();
    write_tokens(writer, &ir.body);
    writer.pop_tab();
    writer.writeln("}");
}

pub fn generate_copy_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer { .. } => String::from("int"),
        TypeIDASTNode::Number { .. } => String::from("double"),
        TypeIDASTNode::Bool { .. } => String::from("bool"),
        TypeIDASTNode::Char { id } => id.clone(),
        TypeIDASTNode::Other { id } => match id.as_str() {
            "GroupAddress" => String::from("int"),
            "CommandsBufferAddress" => String::from("int"),
            _ => id.clone(),
        },
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Option" => format!(
                "{}?",
                generics
                    .iter()
                    .map(generate_copy_type_id)
                    .collect::<Vec<String>>()
                    .first()
                    .expect("Optional type cannot be empty")
            ),
            "Vec" => format!(
                "List<{}>",
                generics
                    .iter()
                    .map(generate_copy_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => format!(
                "{}<{}>",
                id,
                generics
                    .iter()
                    .map(generate_copy_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        },
    }
}

pub fn generate_emplace_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer { .. }
        | TypeIDASTNode::Number { .. }
        | TypeIDASTNode::Bool { .. }
        | TypeIDASTNode::Char { .. }
        | TypeIDASTNode::Other { .. } => generate_copy_type_id(type_id),
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Option" => format!(
                "EmplaceOption<{}>",
                generics
                    .iter()
                    .map(generate_emplace_type_id)
                    .collect::<Vec<String>>()
                    .first()
                    .expect("Optional type cannot be empty")
            ),
            "Vec" => format!(
                "EmplaceList<{}>",
                generics
                    .iter()
                    .map(generate_emplace_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => format!(
                "{}<{}>",
                id,
                generics
                    .iter()
                    .map(generate_emplace_type_id)
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
                if let TypeIDASTNode::Number { .. } = type_id {
                    if value.floor() == *value {
                        format!("{}.0", value)
                    } else {
                        format!("{}", value)
                    }
                } else {
                    panic!("Integer literal cannot have non integer type")
                }
            }
            Literal::BoolLiteral(value) => format!("{}", value),
        },
    }
}

pub fn generate_default_copy_const_value(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size: _,
            signed: _,
        } => String::from("0"),
        TypeIDASTNode::Number { id: _, size: _ } => String::from("0.0"),
        TypeIDASTNode::Bool { id: _ } => String::from("false"),
        TypeIDASTNode::Char { id: _ } => String::from("0"),
        TypeIDASTNode::Other { id } => match id.as_str() {
            "String" => String::from("\"\""),
            "Vec" => String::from("[]"),
            _ => format!("const {}.createDefault()", id),
        },
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Option" => String::from("null"),
            "Vec" => format!(
                "const <{}>[]",
                generics
                    .iter()
                    .map(generate_copy_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => format!(
                "const {}<{}>.createDefault()",
                id,
                generics
                    .iter()
                    .map(generate_copy_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        },
    }
}

pub fn generate_default_emplace_const_value(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Integer {
            id: _,
            size: _,
            signed: _,
        } => String::from("0"),
        TypeIDASTNode::Number { id: _, size: _ } => String::from("0.0"),
        TypeIDASTNode::Bool { id: _ } => String::from("false"),
        TypeIDASTNode::Char { id: _ } => String::from("0"),
        TypeIDASTNode::Other { id } => match id.as_str() {
            "String" => String::from("\"\""),
            "Vec" => String::from("[]"),
            _ => format!("{}.createDefault()", id),
        },
        TypeIDASTNode::Generic { id, generics } => match id.as_str() {
            "Option" => String::from("null"),
            "Vec" => format!(
                "<{}>[]",
                generics
                    .iter()
                    .map(generate_copy_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            _ => format!(
                "{}<{}>.createDefault()",
                id,
                generics
                    .iter()
                    .map(generate_copy_type_id)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        },
    }
}
