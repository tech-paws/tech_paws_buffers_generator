use crate::lexer::{Lexer, Literal, Token};

#[derive(Debug)]
pub enum ASTNode {
    Enum(EnumASTNode),
    Struct(StructASTNode),
    Fn(FnASTNode),
    Mod(ModASTNode),
}

#[derive(Debug)]
pub struct ModASTNode {
    pub id: String,
    pub items: Vec<ASTNode>,
}

#[derive(Debug)]
pub struct EnumASTNode {
    pub id: String,
    pub items: Vec<EnumItemASTNode>,
}

#[derive(Debug)]
pub struct StructASTNode {
    pub id: String,
    pub fields: Vec<StructFieldASTNode>,
    pub emplace_buffers: bool,
    pub into_buffers: bool,
}

#[derive(Debug)]
pub struct FnASTNode {
    pub id: String,
    pub args: Vec<FnArgASTNode>,
    pub return_type_id: Option<TypeIDASTNode>,
}

#[derive(Debug)]
pub enum EnumItemASTNode {
    Empty {
        position: u32,
        id: String,
    },
    Tuple {
        position: u32,
        id: String,
        values: Vec<TupleFieldASTNode>,
    },
    Struct {
        position: u32,
        id: String,
        fields: Vec<StructFieldASTNode>,
    },
}

#[derive(Debug)]
pub enum ConstValueASTNode {
    Literal {
        literal: Literal,
        type_id: TypeIDASTNode,
    },
}

#[derive(Debug)]
pub struct IdValuePair {
    pub id: String,
    pub value: ConstValueASTNode,
}

#[derive(Debug, Clone)]
pub enum TypeIDASTNode {
    Integer {
        id: String,
        size: usize,
        signed: bool,
    },
    Number {
        id: String,
        size: usize,
    },
    Bool {
        id: String,
    },
    Char {
        id: String,
    },
    Other {
        id: String,
    },
}

impl EnumItemASTNode {
    pub fn id(&self) -> &str {
        match self {
            EnumItemASTNode::Empty { position: _, id } => id,
            EnumItemASTNode::Tuple {
                position: _,
                id,
                values: _,
            } => id,
            EnumItemASTNode::Struct {
                position: _,
                id,
                fields: _,
            } => id,
        }
    }

    pub fn position(&self) -> u32 {
        match self {
            EnumItemASTNode::Empty { position, id: _ } => *position,
            EnumItemASTNode::Tuple {
                position,
                id: _,
                values: _,
            } => *position,
            EnumItemASTNode::Struct {
                position,
                id: _,
                fields: _,
            } => *position,
        }
    }
}

impl TypeIDASTNode {
    pub fn u32_type_id() -> Self {
        TypeIDASTNode::Integer {
            id: String::from("u32"),
            size: 8,
            signed: false,
        }
    }
}

#[derive(Debug)]
pub struct TupleFieldASTNode {
    pub position: u32,
    pub type_id: TypeIDASTNode,
}

#[derive(Debug, Clone)]
pub struct StructFieldASTNode {
    pub position: u32,
    pub name: String,
    pub type_id: TypeIDASTNode,
}

#[derive(Debug, Clone)]
pub struct FnArgASTNode {
    pub id: String,
    pub type_id: TypeIDASTNode,
}

pub fn parse(lexer: &mut Lexer) -> Vec<ASTNode> {
    let mut ast_nodes = vec![];

    while *lexer.current_token() != Token::EOF {
        match *lexer.current_token() {
            Token::Struct => ast_nodes.push(parse_struct(lexer)),
            Token::Enum => ast_nodes.push(parse_enum(lexer)),
            Token::Fn => ast_nodes.push(parse_fn(lexer)),
            _ => panic!("Unexpected token: {:?}", lexer.current_token()),
        }
    }

    ast_nodes
}

pub fn parse_struct(lexer: &mut Lexer) -> ASTNode {
    if *lexer.current_token() != Token::Struct {
        panic!("Expected 'struct' but got {:?}", lexer.current_token());
    }

    let name = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        panic!("Expected string value, but got {:?}", lexer.current_token());
    };

    if *lexer.next_token() == Token::Symbol(';') {
        lexer.next_token();

        return ASTNode::Struct(StructASTNode {
            id: name,
            fields: Vec::new(),
            emplace_buffers: true,
            into_buffers: true,
        });
    }

    if *lexer.current_token() != Token::Symbol('{') {
        panic!("Expected ';' or '{{', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    let parameters = parse_struct_parameters(lexer);

    if *lexer.current_token() != Token::Symbol('}') {
        panic!("Expected '}}', but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    ASTNode::Struct(StructASTNode {
        id: name,
        fields: parameters,
        emplace_buffers: true,
        into_buffers: true,
    })
}

pub fn parse_enum(lexer: &mut Lexer) -> ASTNode {
    if *lexer.current_token() != Token::Enum {
        panic!("Expected 'struct' but got {:?}", lexer.current_token());
    }

    let name = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        panic!("Expected string value, but got {:?}", lexer.current_token());
    };

    if *lexer.next_token() != Token::Symbol('{') {
        panic!("Expected '{{', but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() != Token::Symbol('#') {
        panic!("Expected '#', but got {:?}", lexer.current_token());
    }

    let node = parse_enum_items(name, lexer);

    if *lexer.current_token() != Token::Symbol('}') {
        panic!("Expected '}}', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    node
}

pub fn parse_enum_items(id: String, lexer: &mut Lexer) -> ASTNode {
    let mut items = vec![];

    while let Token::Symbol('#') = lexer.current_token() {
        let position = parse_position(lexer);
        let name = if let Token::ID { name } = lexer.current_token() {
            name.clone()
        } else {
            panic!("Expected id, but got {:?}", lexer.current_token());
        };

        let item = match *lexer.next_token() {
            Token::Symbol('(') => parse_tuple_enum(position, name, lexer),
            Token::Symbol('{') => parse_struct_enum(position, name, lexer),
            _ => EnumItemASTNode::Empty { position, id: name },
        };

        items.push(item);

        if *lexer.current_token() != Token::Symbol(',') {
            break;
        }

        lexer.next_token();
    }

    ASTNode::Enum(EnumASTNode { id, items })
}

pub fn parse_struct_enum(position: u32, id: String, lexer: &mut Lexer) -> EnumItemASTNode {
    if *lexer.current_token() != Token::Symbol('{') {
        panic!("Expected ';' or '{{', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    let fields = parse_struct_parameters(lexer);

    if *lexer.current_token() != Token::Symbol('}') {
        panic!("Expected '}}', but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    EnumItemASTNode::Struct {
        position,
        id,
        fields,
    }
}

pub fn parse_tuple_enum(position: u32, id: String, lexer: &mut Lexer) -> EnumItemASTNode {
    if *lexer.current_token() != Token::Symbol('(') {
        panic!("Expected ';' or '(', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    let values = parse_tuple_parameters(lexer);

    if *lexer.current_token() != Token::Symbol(')') {
        panic!("Expected ')', but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    EnumItemASTNode::Tuple {
        position,
        id,
        values,
    }
}

pub fn parse_const_value(lexer: &mut Lexer) -> ConstValueASTNode {
    let literal = if let Token::Literal(literal) = lexer.current_token() {
        literal.clone()
    } else {
        panic!("Expected const value, but got {:?}", lexer.current_token());
    };

    let type_id = match literal {
        Literal::StringLiteral(_) => {
            TypeIDASTNode::Other {
                id: String::from("String"),
            }
        }
        Literal::IntLiteral(_) => {
            TypeIDASTNode::Integer {
                id: String::from("i32"),
                size: 4,
                signed: true,
            }
        }
        Literal::NumberLiteral(_) => {
            TypeIDASTNode::Number {
                id: String::from("f32"),
                size: 4,
            }
        }
    };

    ConstValueASTNode::Literal { literal, type_id }
}

pub fn parse_struct_parameters(lexer: &mut Lexer) -> Vec<StructFieldASTNode> {
    let mut fields = vec![];

    while *lexer.current_token() == Token::Symbol('#') {
        let position = parse_position(lexer);
        let name = if let Token::ID { name } = lexer.current_token() {
            name.clone()
        } else {
            panic!("Expected string value, but got {:?}", lexer.current_token());
        };

        if *lexer.next_token() != Token::Symbol(':') {
            panic!("Expected ':', but got {:?}", lexer.current_token());
        }

        lexer.next_token();
        let type_id = parse_type_id(lexer);
        fields.push(StructFieldASTNode {
            position,
            name,
            type_id,
        });

        if *lexer.current_token() != Token::Symbol(',') {
            break;
        }

        lexer.next_token();
    }

    fields
}

pub fn parse_tuple_parameters(lexer: &mut Lexer) -> Vec<TupleFieldASTNode> {
    let mut fields = vec![];

    while *lexer.current_token() == Token::Symbol('#') {
        let position = parse_position(lexer);
        let type_id = parse_type_id(lexer);

        fields.push(TupleFieldASTNode { position, type_id });

        if *lexer.current_token() != Token::Symbol(',') {
            break;
        }

        lexer.next_token();
    }

    fields
}

pub fn parse_type_id(lexer: &mut Lexer) -> TypeIDASTNode {
    let name = if let Token::ID { name } = lexer.current_token() {
        name.clone()
    } else {
        panic!("Expected string value, but got {:?}", lexer.current_token());
    };
    lexer.next_token();

    match name.as_str() {
        "i8" => {
            TypeIDASTNode::Integer {
                id: name,
                size: 1,
                signed: true,
            }
        }
        "i32" => {
            TypeIDASTNode::Integer {
                id: name,
                size: 4,
                signed: true,
            }
        }
        "i64" => {
            TypeIDASTNode::Integer {
                id: name,
                size: 8,
                signed: true,
            }
        }
        "u8" => {
            TypeIDASTNode::Integer {
                id: name,
                size: 1,
                signed: false,
            }
        }
        "u32" => {
            TypeIDASTNode::Integer {
                id: name,
                size: 4,
                signed: false,
            }
        }
        "u64" => {
            TypeIDASTNode::Integer {
                id: name,
                size: 8,
                signed: false,
            }
        }
        "f32" => TypeIDASTNode::Number { id: name, size: 4 },
        "f64" => TypeIDASTNode::Number { id: name, size: 8 },
        "char" => TypeIDASTNode::Char { id: name },
        "bool" => TypeIDASTNode::Bool { id: name },
        _ => TypeIDASTNode::Other { id: name },
    }
}

pub fn parse_fn(lexer: &mut Lexer) -> ASTNode {
    if *lexer.current_token() != Token::Fn {
        panic!("Expected 'fn' but got {:?}", lexer.current_token());
    }

    let id = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        panic!("Expected id, but got {:?}", lexer.current_token());
    };

    if *lexer.next_token() != Token::Symbol('(') {
        panic!("Expected '(', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    let args = parse_fn_args(lexer);

    if *lexer.current_token() != Token::Symbol(')') {
        panic!("Expected ')', but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() == Token::Symbol(';') {
        lexer.next_token();

        return ASTNode::Fn(FnASTNode {
            id,
            args,
            return_type_id: None,
        });
    }

    if *lexer.current_token() != Token::Symbol('-') {
        panic!("Expected '->', but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() != Token::Symbol('>') {
        panic!("Expected '->', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    let return_type_id = Some(parse_type_id(lexer));

    if *lexer.current_token() != Token::Symbol(';') {
        panic!("Expected ';', but got {:?}", lexer.current_token());
    }
    lexer.next_token();

    ASTNode::Fn(FnASTNode {
        id,
        args,
        return_type_id,
    })
}

pub fn parse_fn_args(lexer: &mut Lexer) -> Vec<FnArgASTNode> {
    let mut args = vec![];

    while let Token::ID { name } = lexer.current_token() {
        let id = name.clone();

        if *lexer.next_token() != Token::Symbol(':') {
            panic!("Expected ':', but got {:?}", lexer.current_token());
        }

        lexer.next_token();
        let type_id = parse_type_id(lexer);
        args.push(FnArgASTNode { id, type_id });

        if *lexer.current_token() != Token::Symbol(',') {
            break;
        }

        lexer.next_token();
    }

    args
}

/// Parse #[<number>]
pub fn parse_position(lexer: &mut Lexer) -> u32 {
    if *lexer.current_token() != Token::Symbol('#') {
        panic!("Expected '#' but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() != Token::Symbol('[') {
        panic!("Expected '[' but got {:?}", lexer.current_token());
    }

    let position = if let Token::Literal(Literal::IntLiteral(value)) = lexer.next_token() {
        *value
    } else {
        panic!("Expected int but got {:?}", lexer.current_token());
    };

    if *lexer.next_token() != Token::Symbol(']') {
        panic!("Expected ']' but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    position as u32
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::writer::Writer;

    use super::*;

    fn stringify_ast(ast: &[ASTNode]) -> String {
        stringify_ast_impl(0, ast)
    }

    fn stringify_ast_impl(tab: usize, ast: &[ASTNode]) -> String {
        let mut writer = Writer::new(2);

        for node in ast {
            match node {
                ASTNode::Mod(node) => {
                    writer.writeln_tab(tab, "Mod {");
                    writer.writeln_tab(tab + 1, &format!("id: \"{}\",", node.id));
                    writer.writeln_tab(tab + 1, "nodes: [");
                    writer.write(&stringify_ast_impl(tab + 2, &ast));
                    writer.writeln_tab(tab + 1, "]");
                    writer.writeln_tab(tab, "}");
                }
                ASTNode::Enum(EnumASTNode { id, items }) => {
                    writer.writeln_tab(tab, "Enum {");
                    writer.writeln_tab(tab + 1, &format!("id: \"{}\",", id));
                    writer.writeln_tab(tab + 1, "items: [");

                    for item in items {
                        match &item {
                            EnumItemASTNode::Tuple {
                                position,
                                id,
                                values,
                            } => {
                                writer.writeln_tab(tab + 2, "TupleFieldASTNode {");
                                writer.writeln_tab(tab + 3, &format!("position: {},", position));
                                writer.writeln_tab(tab + 3, &format!("id: \"{}\",", id));
                                writer.writeln_tab(tab + 3, "items: [");

                                for value in values {
                                    writer.writeln_tab(tab + 4, &format!("{:?}", value));
                                }

                                writer.writeln_tab(tab + 3, "]");
                                writer.writeln_tab(tab + 2, "}");
                            }
                            EnumItemASTNode::Struct {
                                position,
                                id,
                                fields,
                            } => {
                                writer.writeln_tab(tab + 2, "EnumItemASTNode {");
                                writer.writeln_tab(tab + 3, &format!("position: {},", position));
                                writer.writeln_tab(tab + 3, &format!("id: \"{}\",", id));
                                writer.writeln_tab(tab + 3, "fields: [");

                                for field in fields {
                                    writer.writeln_tab(tab + 4, &format!("{:?}", field));
                                }

                                writer.writeln_tab(tab + 3, "]");
                                writer.writeln_tab(tab + 2, "}");
                            }
                            _ => writer.writeln_tab(tab + 2, &format!("{:?}", item)),
                        }
                    }

                    writer.writeln_tab(tab + 1, "]");
                    writer.writeln_tab(tab, "}");
                }
                ASTNode::Struct(StructASTNode {
                    id,
                    fields,
                    emplace_buffers: _,
                    into_buffers: _,
                }) => {
                    writer.writeln_tab(tab, "Struct {");
                    writer.writeln_tab(tab + 1, &format!("id: \"{}\",", id));
                    writer.writeln_tab(tab + 1, "fields: [");

                    for field in fields {
                        writer.writeln_tab(tab + 2, &format!("{:?}", field));
                    }

                    writer.writeln_tab(tab + 1, "]");
                    writer.writeln_tab(tab, "}");
                }
                ASTNode::Fn(FnASTNode {
                    id,
                    args,
                    return_type_id,
                }) => {
                    writer.writeln_tab(tab, "Fn {");
                    writer.writeln_tab(tab + 1, &format!("id: \"{}\",", id));
                    writer.writeln_tab(tab + 1, &format!("return_type_id: {:?},", return_type_id));
                    writer.writeln_tab(tab + 1, "args: [");

                    for arg in args {
                        writer.writeln_tab(tab + 2, &format!("{:?}", arg));
                    }

                    writer.writeln_tab(tab + 1, "]");
                    writer.writeln_tab(tab, "}");
                }
            }
        }

        println!("{}", writer.show());
        writer.into()
    }

    #[test]
    fn parse_position_test() {
        let mut lexer = Lexer::tokenize("#[123]");
        let position = parse_position(&mut lexer);
        assert_eq!(position, 123);
    }

    #[test]
    fn parse_empty_file_test() {
        let src = fs::read_to_string("test_resources/empty.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/empty.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_empty_struct_test() {
        let src = fs::read_to_string("test_resources/empty_struct.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/empty_struct.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_two_empty_structs_test() {
        let src = fs::read_to_string("test_resources/two_empty_structs.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/two_empty_structs.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_struct_with_parameters_test() {
        let src = fs::read_to_string("test_resources/struct_with_parameters.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/struct_with_parameters.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_enum_test() {
        let src = fs::read_to_string("test_resources/enum.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/enum.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_rpc_method_test() {
        let src = fs::read_to_string("test_resources/rpc_method.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/rpc_method.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_rpc_method_without_ret() {
        let src = fs::read_to_string("test_resources/rpc_method_without_ret.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/rpc_method_without_ret.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_complex_test() {
        let src = fs::read_to_string("test_resources/complex.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/complex.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }
}
