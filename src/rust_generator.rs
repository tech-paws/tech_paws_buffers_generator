use crate::{lexer::Literal, parser::{ASTNode, ConstValueASTNode, EnumASTNode, EnumItemASTNode, StructASTNode, StructFieldASTNode, TupleFieldASTNode, TypeIDASTNode, ValueEnumASTNode}};

pub struct Writer {
    res: String,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            res: String::with_capacity(10000),
        }
    }

    pub fn show(&self) -> &str {
        &self.res
    }

    pub fn write(&mut self, data: &str) {
        self.res += data;
    }

    pub fn writeln(&mut self, data: &str) {
        self.res += data;
        self.res += "\n";
    }

    pub fn writeln_tab(&mut self, tab: i32, data: &str) {
        for _ in 0..tab {
            self.res += "    ";
        }

        self.res += data;
        self.res += "\n";
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate(ast: &[ASTNode], models: bool, buffers: bool, _rpc: bool) -> String {
    let mut writer = Writer::new();

    writer.writeln("// GENERATED, DO NOT EDIT");
    writer.writeln("");

    if buffers {
        writer.writeln("use tech_paws_buffers::{BytesReader, BytesWriter, IntoVMBuffers};");
    }

    if models {
        writer.write(&generate_models(ast));
    }

    writer.show().to_string()
}

pub fn generate_models(ast: &[ASTNode]) -> String {
    let mut writer = Writer::new();

    for node in ast {
        match node {
            ASTNode::Struct(node) => writer.writeln(&generate_struct_model(node)),
            ASTNode::Enum(node) => writer.writeln(&generate_enum_model(node)),
            ASTNode::ValueEnum(node) => writer.writeln(&gnerate_value_enum_model(node)),
            ASTNode::Fn(_) => (),
        }
    }

    let mut res = writer.show().to_string();

    if res.ends_with("\n\n") {
        res.pop();
    }

    res
}

pub fn generate_struct_model(node: &StructASTNode) -> String {
    let mut writer = Writer::new();

    if node.fields.is_empty() {
        writer.writeln("#[derive(Debug, Clone, PartialEq)]");
        writer.writeln(&format!("pub struct {};", node.id));
    } else {
        writer.writeln("#[derive(Debug, Clone, PartialEq)]");
        writer.writeln(&format!("pub struct {} {{", node.id));
        writer.write(&generate_struct_parameters(1, &node.fields));
        writer.writeln("}");
    }

    writer.show().to_string()
}

pub fn generate_struct_parameters(tab: i32, params: &[StructFieldASTNode]) -> String {
    let mut writer = Writer::new();

    for param in params {
        let type_id = generate_type_id(&param.type_id);
        writer.writeln_tab(tab, &format!("{}: {},", param.name, type_id));
    }

    writer.show().to_string()
}

pub fn generate_tuple_parameters(tab: i32, params: &[TupleFieldASTNode]) -> String {
    let mut writer = Writer::new();

    for param in params {
        let type_id = generate_type_id(&param.type_id);
        writer.writeln_tab(tab, &format!("{},", type_id));
    }

    writer.show().to_string()
}

pub fn generate_type_id(type_id: &TypeIDASTNode) -> String {
    match type_id {
        TypeIDASTNode::Primitive { id } => id.clone(),
        TypeIDASTNode::Generic { id: _, generics: _ } => todo!(),
    }
}

pub fn gnerate_value_enum_model(node: &ValueEnumASTNode) -> String {
    let mut writer = Writer::new();

    writer.writeln("#[derive(Debug, Clone, PartialEq)]");
    writer.writeln(&format!("pub enum {} {{", node.id));

    for item in node.items.iter() {
        writer.writeln_tab(
            1,
            &format!("{} = {},", item.id, generate_const_value(&item.value)),
        );
    }

    writer.writeln("}");
    writer.show().to_string()
}

pub fn generate_enum_model(node: &EnumASTNode) -> String {
    let mut writer = Writer::new();

    writer.writeln("#[derive(Debug, Clone, PartialEq)]");
    writer.writeln(&format!("pub enum {} {{", node.id));

    for item in node.items.iter() {
        match item {
            EnumItemASTNode::Empty { position: _, id } => {
                writer.writeln_tab(1, &format!("{},", id))
            }
            EnumItemASTNode::Tuple {
                position: _,
                id,
                values,
            } => {
                writer.writeln_tab(1, &format!("{}(", id));
                writer.write(&generate_tuple_parameters(2, values));
                writer.writeln_tab(1, "),");
            }
            EnumItemASTNode::Struct {
                position: _,
                id,
                fields,
            } => {
                writer.writeln_tab(1, &format!("{} {{", id));
                writer.write(&generate_struct_parameters(2, fields));
                writer.writeln_tab(1, "},");
            }
        }
    }

    writer.writeln("}");
    writer.show().to_string()
}

pub fn generate_const_value(node: &ConstValueASTNode) -> String {
    match node {
        ConstValueASTNode::Literal(literal) => {
            match literal {
                Literal::StringLiteral(value) => format!("\"{}\"", value),
                Literal::IntLiteral(value) => format!("{}", value),
                Literal::NumberLiteral(value) => format!("{}", value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{lexer::Lexer, parser::parse};

    use super::*;

    #[test]
    fn generate_empty_file() {
        let src = fs::read_to_string("test_resources/empty.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/empty.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate(&ast, true, true, true);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_empty_struct_model() {
        let src = fs::read_to_string("test_resources/empty_struct.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/empty_struct_models.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_struct_with_parameters_model() {
        let src = fs::read_to_string("test_resources/struct_with_parameters.tpb").unwrap();
        let target =
            fs::read_to_string("test_resources/rust/struct_with_parameters_models.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_two_structs_models() {
        let src = fs::read_to_string("test_resources/two_empty_structs.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/two_empty_structs_models.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_value_enum_models() {
        let src = fs::read_to_string("test_resources/enum_value.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/enum_value_models.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }

    #[test]
    fn generate_complex_enum_models() {
        let src = fs::read_to_string("test_resources/enum_complex.tpb").unwrap();
        let target = fs::read_to_string("test_resources/rust/enum_complex_models.rs").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let ast = parse(&mut lexer);
        let actual = generate_models(&ast);
        println!("{}", actual);
        assert_eq!(actual, target);
    }
}
