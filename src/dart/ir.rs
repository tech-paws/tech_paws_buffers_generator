use strum_macros::IntoStaticStr;

use crate::{ast::TypeIDASTNode, writer::Writer};

#[derive(Clone, Debug, PartialEq, IntoStaticStr)]
pub enum DartIR {
    Id(String),
    CopyTypeId(TypeIDASTNode),
    EmplaceTypeId(TypeIDASTNode),
    Class(ClassDartIR),
    DefaultConstructor(DefaultConstructorIR),
    ShortFunc(ShortFuncIR),
    Call(CallIR),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallIR {
    pub id: String,
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
    pub fields: Vec<DartIR>,
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
            ("ShortFunc", "ShortFunc"),
            ("ShortFunc", "Func"),
            ("Func", "Func"),
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
        DartIR::ShortFunc(ir) => write_short_func(writer, ir),
        DartIR::Call(ir) => write_call(writer, ir),
    }
}

pub fn write_call(writer: &mut Writer, ir: &CallIR) {
    if ir.is_const {
        writer.write("const ");
    }

    writer.write(&ir.id);

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

    if ir.fields.is_empty() {
        writer.write(&format!("{}();", ir.id));
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
