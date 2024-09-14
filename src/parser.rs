use crate::ast::*;
use crate::lexer::{Lexer, Literal, Token};

macro_rules! parse_error {
    ($lexer:expr, $($arg:tt)*) => ({
        let line = $lexer.line();
        let pos = $lexer.pos();
        panic!("{}:{}: {}", line, pos, format!($($arg)*));
    });
}

#[derive(Clone, Default)]
struct ParseContext {
    doc_comments: Vec<String>,
}

pub fn parse(lexer: &mut Lexer) -> Vec<ASTNode> {
    let mut ast_nodes = vec![];

    while *lexer.current_token() != Token::EOF {
        let node = parse_with_context(None, lexer);
        ast_nodes.push(node);
    }

    ast_nodes
}

fn parse_with_context(context: Option<ParseContext>, lexer: &mut Lexer) -> ASTNode {
    let mut context = if let Some(context) = context {
        context
    } else {
        ParseContext::default()
    };

    match lexer.current_token().clone() {
        Token::Struct => parse_struct(&mut context, lexer),
        Token::Enum => parse_enum(&mut context, lexer),
        Token::Async => parse_async(&mut context, lexer),
        Token::Fn => parse_fn(&mut context, lexer, false),
        Token::Signal => parse_signal(&mut context, lexer, false),
        Token::Const => ASTNode::Const(parse_const(lexer)),
        Token::DocComment { .. } => {
            context.doc_comments = parse_doc_comments(lexer);

            match lexer.current_token().clone() {
                Token::Struct | Token::Enum | Token::Fn | Token::Signal => {
                    parse_with_context(Some(context), lexer)
                }
                _ => ASTNode::DocComments {
                    comments: context.doc_comments,
                },
            }
        }
        Token::Symbol('#') => parse_directive(lexer),
        _ => parse_error!(lexer, "Unexpected token: {:?}", lexer.current_token()),
    }
}

pub fn parse_doc_comments(lexer: &mut Lexer) -> Vec<String> {
    let mut comments = vec![];

    while let Token::DocComment { value } = lexer.current_token() {
        comments.push(value.clone());
        lexer.next_token();
    }

    comments
}

fn parse_struct(context: &mut ParseContext, lexer: &mut Lexer) -> ASTNode {
    if *lexer.current_token() != Token::Struct {
        parse_error!(
            lexer,
            "Expected 'struct' but got {:?}",
            lexer.current_token()
        );
    }

    let name = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        parse_error!(
            lexer,
            "Expected string value, but got {:?}",
            lexer.current_token()
        );
    };

    if *lexer.next_token() == Token::Symbol(';') {
        lexer.next_token();

        return ASTNode::Struct(StructASTNode {
            id: name,
            doc_comments: context.doc_comments.clone(),
            fields: Vec::new(),
            emplace_buffers: true,
            into_buffers: true,
        });
    }

    if *lexer.current_token() != Token::Symbol('{') {
        parse_error!(
            lexer,
            "Expected ';' or '{{', but got {:?}",
            lexer.current_token()
        );
    }

    match lexer.next_token() {
        Token::Symbol('#') => (),
        Token::DocComment { .. } => (),
        Token::ID { name: _ } => (),
        _ => parse_error!(
            lexer,
            "Expected '#' or Id, but got {:?}",
            lexer.current_token()
        ),
    }

    let parameters = parse_struct_parameters(lexer);

    if *lexer.current_token() != Token::Symbol('}') {
        parse_error!(lexer, "Expected '}}', but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    ASTNode::Struct(StructASTNode {
        id: name,
        doc_comments: context.doc_comments.clone(),
        fields: parameters,
        emplace_buffers: true,
        into_buffers: true,
    })
}

fn parse_enum(context: &mut ParseContext, lexer: &mut Lexer) -> ASTNode {
    if *lexer.current_token() != Token::Enum {
        parse_error!(
            lexer,
            "Expected 'struct' but got {:?}",
            lexer.current_token()
        );
    }

    let name = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        parse_error!(
            lexer,
            "Expected string value, but got {:?}",
            lexer.current_token()
        );
    };

    if *lexer.next_token() != Token::Symbol('{') {
        parse_error!(lexer, "Expected '{{', but got {:?}", lexer.current_token());
    }

    match lexer.next_token() {
        Token::Symbol('#') => (),
        Token::DocComment { .. } => (),
        Token::ID { name: _ } => (),
        _ => parse_error!(
            lexer,
            "Expected '#' or Id, but got {:?}",
            lexer.current_token()
        ),
    }

    let node = parse_enum_items(context, name, lexer);

    if *lexer.current_token() != Token::Symbol('}') {
        parse_error!(lexer, "Expected '}}', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    node
}

fn parse_enum_items(context: &mut ParseContext, id: String, lexer: &mut Lexer) -> ASTNode {
    let mut items = vec![];
    let mut positions = vec![];
    let mut auto_position = 0;

    loop {
        let mut context = context.clone();
        context.doc_comments = if let Token::DocComment { .. } = lexer.current_token() {
            parse_doc_comments(lexer)
        } else {
            vec![]
        };

        let position = if let Token::Symbol('#') = lexer.current_token() {
            let position = parse_position(lexer);

            if !positions.contains(&position) {
                positions.push(position);
            } else {
                parse_error!(lexer, "the position {} already exists", position);
            }

            position
        } else {
            auto_position
        };

        if let Token::ID { name } = lexer.current_token() {
            let name = name.clone();

            let item = match *lexer.next_token() {
                Token::Symbol('(') => parse_tuple_enum(&mut context, position, name, lexer),
                Token::Symbol('{') => parse_struct_enum(&mut context, position, name, lexer),
                _ => EnumItemASTNode::Empty {
                    doc_comments: context.doc_comments.clone(),
                    position,
                    id: name,
                },
            };

            auto_position += 1;

            while positions.contains(&auto_position) {
                auto_position += 1;
            }

            items.push(item);

            if *lexer.current_token() != Token::Symbol(',') {
                break;
            }

            lexer.next_token();
        } else {
            break;
        }
    }

    ASTNode::Enum(EnumASTNode {
        doc_comments: context.doc_comments.clone(),
        id,
        items,
    })
}

fn parse_struct_enum(
    context: &mut ParseContext,
    position: u32,
    id: String,
    lexer: &mut Lexer,
) -> EnumItemASTNode {
    if *lexer.current_token() != Token::Symbol('{') {
        parse_error!(
            lexer,
            "Expected ';' or '{{', but got {:?}",
            lexer.current_token()
        );
    }

    lexer.next_token();
    let fields = parse_struct_parameters(lexer);

    if *lexer.current_token() != Token::Symbol('}') {
        parse_error!(lexer, "Expected '}}', but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    EnumItemASTNode::Struct {
        doc_comments: context.doc_comments.clone(),
        position,
        id,
        fields,
    }
}

fn parse_tuple_enum(
    context: &mut ParseContext,
    position: u32,
    id: String,
    lexer: &mut Lexer,
) -> EnumItemASTNode {
    if *lexer.current_token() != Token::Symbol('(') {
        parse_error!(
            lexer,
            "Expected ';' or '(', but got {:?}",
            lexer.current_token()
        );
    }

    lexer.next_token();
    let values = parse_tuple_parameters(lexer);

    if *lexer.current_token() != Token::Symbol(')') {
        parse_error!(lexer, "Expected ')', but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    EnumItemASTNode::Tuple {
        doc_comments: context.doc_comments.clone(),
        position,
        id,
        values,
    }
}

pub fn parse_const_value(lexer: &mut Lexer) -> ConstValueASTNode {
    let literal = if let Token::Literal(literal) = lexer.current_token() {
        literal.clone()
    } else {
        parse_error!(
            lexer,
            "Expected const value, but got {:?}",
            lexer.current_token()
        );
    };

    let type_id = match literal {
        Literal::BoolLiteral(_) => TypeIDASTNode::Bool {
            id: String::from("bool"),
        },
        Literal::StringLiteral(_) => TypeIDASTNode::Other {
            id: String::from("String"),
        },
        Literal::IntLiteral(_) => TypeIDASTNode::Integer {
            id: String::from("i32"),
            size: 4,
            signed: true,
        },
        Literal::NumberLiteral(_) => TypeIDASTNode::Number {
            id: String::from("f32"),
            size: 4,
        },
    };

    ConstValueASTNode::Literal { literal, type_id }
}

pub fn parse_struct_parameters(lexer: &mut Lexer) -> Vec<StructFieldASTNode> {
    let mut fields = vec![];
    let mut positions = vec![];
    let mut auto_position = 0;

    loop {
        let doc_comments = if let Token::DocComment { .. } = lexer.current_token() {
            parse_doc_comments(lexer)
        } else {
            vec![]
        };

        let position = if let Token::Symbol('#') = lexer.current_token() {
            let position = parse_position(lexer);

            if !positions.contains(&position) {
                positions.push(position);
            } else {
                parse_error!(lexer, "the position {} already exists", position);
            }

            position
        } else {
            auto_position
        };

        if let Token::ID { name } = lexer.current_token() {
            let name = name.clone();

            if *lexer.next_token() != Token::Symbol(':') {
                parse_error!(lexer, "Expected ':', but got {:?}", lexer.current_token());
            }

            lexer.next_token();
            let type_id = parse_type_id(lexer);
            fields.push(StructFieldASTNode {
                doc_comments,
                position,
                name,
                type_id,
            });

            auto_position += 1;

            while positions.contains(&auto_position) {
                auto_position += 1;
            }

            if *lexer.current_token() != Token::Symbol(',') {
                break;
            }

            lexer.next_token();
        } else {
            break;
        }
    }

    fields.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());

    fields
}

pub fn parse_tuple_parameters(lexer: &mut Lexer) -> Vec<TupleFieldASTNode> {
    let mut fields = vec![];
    let mut positions = vec![];
    let mut auto_position = 0;

    loop {
        let doc_comments = if let Token::DocComment { .. } = lexer.current_token() {
            parse_doc_comments(lexer)
        } else {
            vec![]
        };

        let position = if let Token::Symbol('#') = lexer.current_token() {
            let position = parse_position(lexer);

            if !positions.contains(&position) {
                positions.push(position);
            } else {
                parse_error!(lexer, "the position {} already exists", position);
            }

            position
        } else {
            auto_position
        };

        if let Token::ID { .. } = lexer.current_token() {
            let type_id = parse_type_id(lexer);

            fields.push(TupleFieldASTNode {
                doc_comments,
                position,
                type_id,
            });

            auto_position += 1;

            while positions.contains(&auto_position) {
                auto_position += 1;
            }

            if *lexer.current_token() != Token::Symbol(',') {
                break;
            }

            lexer.next_token();
        } else {
            break;
        }
    }

    fields.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());

    fields
}

pub fn parse_type_id(lexer: &mut Lexer) -> TypeIDASTNode {
    let name = if let Token::ID { name } = lexer.current_token() {
        name.clone()
    } else {
        parse_error!(
            lexer,
            "Expected string value, but got {:?}",
            lexer.current_token()
        );
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
                _ => parse_error!(
                    lexer,
                    "Expected ',' or '>' but got {:?}",
                    lexer.current_token()
                ),
            }
        }
    }

    match name.as_str() {
        "i8" => TypeIDASTNode::Integer {
            id: name,
            size: 1,
            signed: true,
        },
        "i32" => TypeIDASTNode::Integer {
            id: name,
            size: 4,
            signed: true,
        },
        "i64" => TypeIDASTNode::Integer {
            id: name,
            size: 8,
            signed: true,
        },
        "u8" => TypeIDASTNode::Integer {
            id: name,
            size: 1,
            signed: false,
        },
        "u32" => TypeIDASTNode::Integer {
            id: name,
            size: 4,
            signed: false,
        },
        "u64" => TypeIDASTNode::Integer {
            id: name,
            size: 8,
            signed: false,
        },
        "f32" => TypeIDASTNode::Number { id: name, size: 4 },
        "f64" => TypeIDASTNode::Number { id: name, size: 8 },
        "char" => TypeIDASTNode::Char { id: name },
        "bool" => TypeIDASTNode::Bool { id: name },
        _ => TypeIDASTNode::Other { id: name },
    }
}

fn parse_async(context: &mut ParseContext, lexer: &mut Lexer) -> ASTNode {
    lexer.next_token();

    match lexer.current_token() {
        Token::Fn => parse_fn(context, lexer, true),
        Token::Signal => parse_signal(context, lexer, true),
        _ => parse_error!(
            lexer,
            "Expected 'fn' or stream' but got {:?}",
            lexer.current_token()
        ),
    }
}

fn parse_fn(context: &mut ParseContext, lexer: &mut Lexer, is_async: bool) -> ASTNode {
    if *lexer.current_token() != Token::Fn {
        parse_error!(lexer, "Expected 'fn' but got {:?}", lexer.current_token());
    }

    let id = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        parse_error!(lexer, "Expected id, but got {:?}", lexer.current_token());
    };

    if *lexer.next_token() != Token::Symbol('(') {
        parse_error!(lexer, "Expected '(', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    let args = parse_fn_args(lexer);

    if *lexer.current_token() != Token::Symbol(')') {
        parse_error!(lexer, "Expected ')', but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() == Token::Symbol(';') {
        lexer.next_token();

        return ASTNode::Fn(FnASTNode {
            id,
            args,
            position: lexer.next_fn_poisition(),
            is_signal: false,
            is_async,
            return_type_id: None,
            doc_comments: context.doc_comments.clone(),
        });
    }

    if *lexer.current_token() != Token::Symbol('-') {
        parse_error!(lexer, "Expected '->', but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() != Token::Symbol('>') {
        parse_error!(lexer, "Expected '->', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    let return_type_id = Some(parse_type_id(lexer));

    if *lexer.current_token() != Token::Symbol(';') {
        parse_error!(lexer, "Expected ';', but got {:?}", lexer.current_token());
    }
    lexer.next_token();

    ASTNode::Fn(FnASTNode {
        id,
        args,
        return_type_id,
        position: lexer.next_fn_poisition(),
        is_signal: false,
        is_async,
        doc_comments: context.doc_comments.clone(),
    })
}

fn parse_signal(context: &mut ParseContext, lexer: &mut Lexer, is_async: bool) -> ASTNode {
    if *lexer.current_token() != Token::Signal {
        parse_error!(
            lexer,
            "Expected 'stream' but got {:?}",
            lexer.current_token()
        );
    };

    let id = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        parse_error!(lexer, "Expected id, but got {:?}", lexer.current_token());
    };

    if *lexer.next_token() == Token::Symbol(';') {
        lexer.next_token();

        return ASTNode::Fn(FnASTNode {
            id,
            args: vec![],
            is_signal: true,
            position: lexer.next_fn_poisition(),
            is_async,
            return_type_id: None,
            doc_comments: context.doc_comments.clone(),
        });
    }

    if *lexer.current_token() != Token::Symbol('-') {
        parse_error!(lexer, "Expected '->', but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() != Token::Symbol('>') {
        parse_error!(lexer, "Expected '->', but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    let return_type_id = Some(parse_type_id(lexer));

    if *lexer.current_token() != Token::Symbol(';') {
        parse_error!(lexer, "Expected ';', but got {:?}", lexer.current_token());
    }
    lexer.next_token();

    ASTNode::Fn(FnASTNode {
        id,
        doc_comments: context.doc_comments.clone(),
        args: vec![],
        return_type_id,
        position: lexer.next_fn_poisition(),
        is_signal: true,
        is_async,
    })
}

pub fn parse_fn_args(lexer: &mut Lexer) -> Vec<FnArgASTNode> {
    let mut args = vec![];

    while let Token::ID { name } = lexer.current_token() {
        let id = name.clone();

        if *lexer.next_token() != Token::Symbol(':') {
            parse_error!(lexer, "Expected ':', but got {:?}", lexer.current_token());
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

pub fn parse_const(lexer: &mut Lexer) -> ConstBlockASTNode {
    let id = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        parse_error!(lexer, "Expected id but got {:?}", lexer.current_token());
    };

    if *lexer.next_token() != Token::Symbol('{') {
        parse_error!(lexer, "Expected '{{' but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    let mut items: Vec<ConstItemASTNode> = vec![];

    while *lexer.current_token() != Token::Symbol('}') && *lexer.current_token() != Token::EOF {
        match lexer.current_token() {
            Token::Const => {
                let const_ast_node = parse_const(lexer);
                items.push(ConstItemASTNode::ConstsBlock {
                    node: const_ast_node,
                });
            }
            Token::ID { name } => {
                let id = name.clone();

                if *lexer.next_token() != Token::Symbol(':') {
                    parse_error!(lexer, "Expected ':' but got {:?}", lexer.current_token());
                }

                lexer.next_token();

                let type_id = parse_type_id(lexer);

                if *lexer.current_token() != Token::Symbol('=') {
                    parse_error!(lexer, "Expected '=' but got {:?}", lexer.current_token());
                }

                lexer.next_token();
                let value = parse_const_value(lexer);

                if *lexer.next_token() != Token::Symbol(';') {
                    parse_error!(lexer, "Expected ';' but got {:?}", lexer.current_token());
                }

                lexer.next_token();
                items.push(ConstItemASTNode::Value { id, type_id, value });
            }
            _ => parse_error!(lexer, "Unexpected token: {:?}", lexer.current_token()),
        }
    }

    if *lexer.current_token() != Token::Symbol('}') {
        parse_error!(lexer, "Expected '}}' but got {:?}", lexer.current_token());
    }

    lexer.next_token();

    ConstBlockASTNode { id, items }
}

/// Parse #[<number>]
pub fn parse_position(lexer: &mut Lexer) -> u32 {
    if *lexer.current_token() != Token::Symbol('#') {
        parse_error!(lexer, "Expected '#' but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() != Token::Symbol('[') {
        parse_error!(lexer, "Expected '[' but got {:?}", lexer.current_token());
    }

    let position = if let Token::Literal(Literal::IntLiteral(value)) = lexer.next_token() {
        *value
    } else {
        parse_error!(lexer, "Expected int but got {:?}", lexer.current_token());
    };

    if *lexer.next_token() != Token::Symbol(']') {
        parse_error!(lexer, "Expected ']' but got {:?}", lexer.current_token());
    }

    lexer.next_token();
    position as u32
}

/// Parse:
/// #[<id> = <const>] | #[<id>(<args>)]
/// args: <id> = <const>, args
pub fn parse_directive(lexer: &mut Lexer) -> ASTNode {
    if *lexer.current_token() != Token::Symbol('#') {
        parse_error!(lexer, "Expected '#' but got {:?}", lexer.current_token());
    }

    if *lexer.next_token() != Token::Symbol('[') {
        parse_error!(lexer, "Expected '[' but got {:?}", lexer.current_token());
    }

    let id = if let Token::ID { name } = lexer.next_token() {
        name.clone()
    } else {
        parse_error!(lexer, "Expected id but got {:?}", lexer.current_token());
    };

    let directive = match lexer.next_token() {
        Token::Symbol('=') => parse_value_directive(id, lexer),
        Token::Symbol('(') => parse_group_directive(id, lexer),
        _ => parse_error!(
            lexer,
            "Expected '=' or '(' but got {:?}",
            lexer.current_token()
        ),
    };

    if *lexer.next_token() != Token::Symbol(']') {
        parse_error!(lexer, "Expected ']' but got {:?}", lexer.current_token());
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

        if *lexer.next_token() == Token::Symbol('=') {
            // parse_error!(lexer, "Expected '=', but got {:?}", lexer.current_token());

            lexer.next_token();
            let value = parse_const_value(lexer);
            values.push(IdValuePair {
                id,
                value: Some(value),
            });

            lexer.next_token();
        } else {
            values.push(IdValuePair { id, value: None });
        }

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
        let result = stringify_ast_impl(0, ast);
        println!("{}", result);
        result
    }

    fn stringify_ast_impl(tab: usize, ast: &[ASTNode]) -> String {
        let mut writer = Writer::new(2);

        for node in ast {
            match node {
                ASTNode::DocComments { comments } => {
                    writer.writeln_tab(tab, "DocComment {");
                    writer.writeln_tab(tab + 1, "comments: [");

                    for comment in comments {
                        writer.writeln_tab(tab + 2, &format!("\"{}\"", comment));
                    }

                    writer.writeln_tab(tab + 1, "]");
                    writer.writeln_tab(tab, "}");
                }
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
                ASTNode::Enum(EnumASTNode {
                    doc_comments,
                    id,
                    items,
                }) => {
                    writer.writeln_tab(tab, "Enum {");
                    writer.writeln_tab(tab + 1, "doc_comments: [");

                    for comment in doc_comments {
                        writer.writeln_tab(tab + 2, &format!("\"{}\"", comment));
                    }

                    writer.writeln_tab(tab + 1, "],");

                    writer.writeln_tab(tab + 1, &format!("id: \"{}\",", id));
                    writer.writeln_tab(tab + 1, "items: [");

                    for item in items {
                        match &item {
                            EnumItemASTNode::Tuple {
                                doc_comments,
                                position,
                                id,
                                values,
                            } => {
                                writer.writeln_tab(tab + 2, "TupleFieldASTNode {");

                                writer.writeln_tab(tab + 3, "doc_comments: [");

                                for comment in doc_comments {
                                    writer.writeln_tab(tab + 4, &format!("\"{}\"", comment));
                                }

                                writer.writeln_tab(tab + 3, "],");

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
                                doc_comments,
                                position,
                                id,
                                fields,
                            } => {
                                writer.writeln_tab(tab + 2, "EnumItemASTNode {");

                                writer.writeln_tab(tab + 3, "doc_comments: [");

                                for comment in doc_comments {
                                    writer.writeln_tab(tab + 4, &format!("\"{}\"", comment));
                                }

                                writer.writeln_tab(tab + 3, "],");

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
                    doc_comments,
                    id,
                    fields,
                    emplace_buffers: _,
                    into_buffers: _,
                }) => {
                    writer.writeln_tab(tab, "Struct {");

                    writer.writeln_tab(tab + 1, "doc_comments: [");

                    for comment in doc_comments {
                        writer.writeln_tab(tab + 2, &format!("\"{}\"", comment));
                    }

                    writer.writeln_tab(tab + 1, "],");

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
                    position,
                    args,
                    return_type_id,
                    is_signal,
                    is_async,
                    doc_comments,
                }) => {
                    writer.writeln_tab(tab, "Fn {");

                    writer.writeln_tab(tab + 1, "doc_comments: [");

                    for comment in doc_comments {
                        writer.writeln_tab(tab + 2, &format!("\"{}\"", comment));
                    }

                    writer.writeln_tab(tab + 1, "],");

                    writer.writeln_tab(tab + 1, &format!("id: \"{}\",", id));
                    writer.writeln_tab(tab + 1, &format!("position: {:?},", position));
                    writer.writeln_tab(tab + 1, &format!("return_type_id: {:?},", return_type_id));
                    writer.writeln_tab(tab + 1, &format!("is_signal: {:?},", is_signal));
                    writer.writeln_tab(tab + 1, &format!("is_async: {:?},", is_async));
                    writer.writeln_tab(tab + 1, "args: [");

                    for arg in args {
                        writer.writeln_tab(tab + 2, &format!("{:?}", arg));
                    }

                    writer.writeln_tab(tab + 1, "]");
                    writer.writeln_tab(tab, "}");
                }
                ASTNode::Const(ConstBlockASTNode { id, items }) => {
                    writer.writeln_tab(tab, "Const {");
                    writer.writeln_tab(tab + 1, &format!("id: \"{}\",", id));
                    writer.writeln_tab(tab + 1, "items: [");

                    for item in items {
                        match &item {
                            ConstItemASTNode::Value { id, type_id, value } => {
                                writer.writeln_tab(tab + 2, "Value {");
                                writer.writeln_tab(tab + 3, &format!("id: {}", id));
                                writer.writeln_tab(tab + 3, &format!("type_id: {:?}", type_id));
                                writer.writeln_tab(tab + 3, &format!("value: {:?}", value));
                                writer.writeln_tab(tab + 2, "}");
                            }
                            ConstItemASTNode::ConstsBlock { node } => {
                                writer.write(&stringify_ast_impl(
                                    tab + 2,
                                    &[ASTNode::Const(node.clone())],
                                ));
                            }
                        }
                    }

                    writer.writeln_tab(tab + 1, "]");
                    writer.writeln_tab(tab, "}");
                }
            }
        }

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

    #[test]
    fn parse_consts_test() {
        let src = fs::read_to_string("test_resources/consts.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/consts.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }

    #[test]
    fn parse_doc_comments_test() {
        let src = fs::read_to_string("test_resources/doc_comments.tpb").unwrap();
        let target_ast = fs::read_to_string("test_resources/doc_comments.ast").unwrap();
        let mut lexer = Lexer::tokenize(&src);
        let actual_ast = stringify_ast(&parse(&mut lexer));

        assert_eq!(actual_ast, target_ast);
    }
}
