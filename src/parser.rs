use crate::ast::*;
use crate::lexer::{Lexer, Literal, Token};

pub fn parse(lexer: &mut Lexer) -> Vec<ASTNode> {
    let mut ast_nodes = vec![];

    while *lexer.current_token() != Token::EOF {
        match *lexer.current_token() {
            Token::Struct => ast_nodes.push(parse_struct(lexer)),
            Token::Enum => ast_nodes.push(parse_enum(lexer)),
            Token::Fn => ast_nodes.push(parse_fn(lexer)),
            Token::Symbol('#') => ast_nodes.push(parse_directive(lexer)),
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

    if let Token::Symbol('<') = lexer.next_token() {
        let mut generics = vec![];

        while let Token::ID { name: _ } = lexer.next_token() {
            let type_id = parse_type_id(lexer);
            generics.push(type_id);

            match lexer.current_token() {
                Token::Symbol('>') => {
                    lexer.next_token();
                    return TypeIDASTNode::Generic { id: name, generics };
                }
                Token::Symbol(',') => {
                    continue;
                }
                _ => panic!("Expected ',' or '>' but got {:?}", lexer.current_token()),
            }
        }
    }

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

/// Parse:
/// #[<id> = <const>] | #[<id>(<args>)]
/// args: <id> = <const>, args
pub fn parse_directive(lexer: &mut Lexer) -> ASTNode {
    if *lexer.current_token() != Token::Symbol('#') {
        panic!("Expected '#' but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() != Token::Symbol('[') {
        panic!("Expected '[' but got {:?}", lexer.current_token());
    }

    let id = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        panic!("Expected id but got {:?}", lexer.current_token());
    };

    let directive = match lexer.next_token() {
        Token::Symbol('=') => parse_value_directive(id, lexer),
        Token::Symbol('(') => parse_group_directive(id, lexer),
        _ => panic!("Expected '=' or '(' but got {:?}", lexer.current_token()),
    };

    if *lexer.next_token() != Token::Symbol(']') {
        panic!("Expected ']' but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    ASTNode::Directive(directive)
}

fn parse_value_directive(id: String, lexer: &mut Lexer) -> DirectiveASTNode {
    lexer.next_token();
    let value = parse_const_value(lexer);
    DirectiveASTNode::Value { id, value }
}

fn parse_group_directive(id: String, lexer: &mut Lexer) -> DirectiveASTNode {
    lexer.next_token();

    let mut values = vec![];

    while let Token::ID { name } = lexer.current_token() {
        let id = name.clone();

        if *lexer.next_token() != Token::Symbol('=') {
            panic!("Expected '=', but got {:?}", lexer.current_token());
        }

        lexer.next_token();
        let value = parse_const_value(lexer);
        values.push(IdValuePair { id, value });
        lexer.next_token();

        if *lexer.current_token() != Token::Symbol(',') {
            break;
        } else {
            lexer.next_token();
        }
    }

    DirectiveASTNode::Group {
        group_id: id,
        values,
    }
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
                ASTNode::Directive(DirectiveASTNode::Group {
                    group_id,
                    values: args,
                }) => {
                    writer.writeln_tab(tab, "DirectiveASTNode::Group {");
                    writer.writeln_tab(tab + 1, &format!("group_id: \"{}\",", group_id));
                    writer.writeln_tab(tab + 1, "args: [");

                    for arg in args {
                        writer.writeln_tab(tab + 2, "IdValuePair {");
                        writer.writeln_tab(tab + 3, &format!("id: \"{}\"", arg.id));
                        writer.writeln_tab(tab + 3, &format!("value: {:?}", arg.value));
                        writer.writeln_tab(tab + 2, "}");
                    }

                    writer.writeln_tab(tab + 1, "]");
                    writer.writeln_tab(tab, "}");
                }
                ASTNode::Directive(DirectiveASTNode::Value { id, value }) => {
                    writer.writeln_tab(tab, "DirectiveASTNode::Value {");
                    writer.writeln_tab(tab + 1, &format!("id: \"{}\"", id));
                    writer.writeln_tab(tab + 1, &format!("value: {:?}", value));
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
        writer.show().to_string()
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
    fn parse_struct_test() {
        let src = fs::read_to_string("test_resources/struct.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/struct.ast").unwrap();
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
    fn parse_rpc_methods_test() {
        let src = fs::read_to_string("test_resources/rpc_methods.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/rpc_methods.ast").unwrap();
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

    #[test]
    fn parse_directive_test() {
        let src = fs::read_to_string("test_resources/directive.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/directive.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }
}
