use crate::lexer::{Lexer, Literal, Token};

#[derive(Debug)]
pub enum ASTNode {
    Enum {
        id: String,
        items: Vec<EnumItemASTNode>,
    },
    ValueEnum {
        id: String,
        items: Vec<IdValuePair>,
    },
    Struct(StructASTNode),
}

#[derive(Debug)]
pub struct StructASTNode {
    pub id: String,
    pub fields: Vec<StructFieldASTNode>,
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
    Literal(Literal),
}

#[derive(Debug)]
pub struct IdValuePair {
    pub id: String,
    pub value: ConstValueASTNode,
}

#[derive(Debug)]
pub enum TypeIDASTNode {
    Primitive {
        id: String,
    },
    Generic {
        id: String,
        generics: Vec<TypeIDASTNode>,
    },
}

#[derive(Debug)]
pub struct TupleFieldASTNode {
    pub position: u32,
    pub type_id: TypeIDASTNode,
}

#[derive(Debug)]
pub struct StructFieldASTNode {
    pub position: u32,
    pub name: String,
    pub type_id: TypeIDASTNode,
}

pub fn parse(lexer: &mut Lexer) -> Vec<ASTNode> {
    let mut ast_nodes = vec![];

    while *lexer.current_token() != Token::EOF {
        match *lexer.current_token() {
            Token::Struct => ast_nodes.push(parse_struct(lexer)),
            Token::Enum => ast_nodes.push(parse_enum(lexer)),
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

    let node = if *lexer.next_token() == Token::Symbol('#') {
        parse_enum_items(name, lexer)
    } else {
        parse_enum_value_items(name, lexer)
    };

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

    ASTNode::Enum { id, items }
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

pub fn parse_enum_value_items(id: String, lexer: &mut Lexer) -> ASTNode {
    let mut items = vec![];

    while let Token::ID { name } = lexer.current_token() {
        let name = name.clone();

        if *lexer.next_token() != Token::Symbol('=') {
            panic!("Expected '=', but got {:?}", lexer.current_token());
        }

        let literal = if let Token::Literal(literal) = lexer.next_token() {
            literal.clone()
        } else {
            panic!("Expected const value, but got {:?}", lexer.current_token());
        };

        items.push(IdValuePair {
            id: name,
            value: ConstValueASTNode::Literal(literal),
        });

        if *lexer.next_token() != Token::Symbol(',') {
            break;
        }

        lexer.next_token();
    }

    ASTNode::ValueEnum { id, items }
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
    TypeIDASTNode::Primitive { id: name }
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

    use super::*;

    fn stringify_ast(ast: Vec<ASTNode>) -> String {
        let mut res = String::new();

        for node in ast {
            match node {
                ASTNode::Enum { id, items } => {
                    let mut items_res = String::new();

                    for item in items {
                        match &item {
                            EnumItemASTNode::Tuple {
                                position,
                                id,
                                values,
                            } => {
                                let mut values_res = String::new();

                                for value in values {
                                    values_res += &format!("        {:?}\n", value);
                                }

                                items_res += &format!(
                                    "    TupleFieldASTNode {{\n      position: {},\n      id: \"{}\",\n      items: [\n{}      ]\n    }}\n",
                                    position, id, values_res
                                );
                            }
                            EnumItemASTNode::Struct {
                                position,
                                id,
                                fields,
                            } => {
                                let mut fields_res = String::new();

                                for field in fields {
                                    fields_res += &format!("        {:?}\n", field);
                                }

                                items_res += &format!(
                                    "    EnumItemASTNode {{\n      position: {},\n      id: \"{}\",\n      fields: [\n{}      ]\n    }}\n",
                                    position, id, fields_res
                                );
                            }
                            _ => items_res += &format!("    {:?}\n", item),
                        }
                    }

                    res += &format!(
                        "Enum {{\n  id: \"{}\",\n  items: [\n{}  ]\n}}\n",
                        id, items_res
                    );
                }
                ASTNode::ValueEnum { id, items } => {
                    let mut items_res = String::new();

                    for item in items {
                        items_res += &format!("    {:?}\n", item);
                    }

                    res += &format!(
                        "ValueEnum {{\n  id: \"{}\",\n  items: [\n{}  ]\n}}\n",
                        id, items_res
                    );
                }
                ASTNode::Struct(StructASTNode { id, fields }) => {
                    let mut fields_res = String::new();

                    for field in fields {
                        fields_res += &format!("    {:?}\n", field);
                    }

                    res += &format!(
                        "Struct {{\n  id: \"{}\",\n  fields: [\n{}  ]\n}}\n",
                        id, fields_res
                    );
                }
            }
        }

        println!("{}", res);
        res
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
        let actual_ast = stringify_ast(parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_empty_struct_test() {
        let src = fs::read_to_string("test_resources/empty_struct.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/empty_struct.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_two_empty_structs_test() {
        let src = fs::read_to_string("test_resources/two_empty_structs.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/two_empty_structs.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_struct_with_parameters_test() {
        let src = fs::read_to_string("test_resources/struct_with_parameters.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/struct_with_parameters.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_value_enum_test() {
        let src = fs::read_to_string("test_resources/enum_value.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/enum_value.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_complex_enum_test() {
        let src = fs::read_to_string("test_resources/enum_complex.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/enum_complex.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }
}
