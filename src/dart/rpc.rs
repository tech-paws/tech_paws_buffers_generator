use crate::{
    ast,
    dart_generator::{
        generate_option_type_id, generate_read, generate_read_emplace, generate_type_id,
        generate_write,
    },
    lexer::Literal,
};
use convert_case::{Case, Casing};

use crate::{
    dart_generator::{generate_struct_buffers, generate_struct_model},
    writer::Writer,
};

pub fn generate_rpc_methods(ast: &[ast::ASTNode]) -> String {
    let mut writer = Writer::new(2);

    let fn_nodes = ast::find_fn_nodes(ast);

    if fn_nodes.is_empty() {
        return String::from("");
    }

    for node in fn_nodes.iter() {
        let args_struct_id = format!("__{}_rpc_args__", node.id);

        let mut args_struct_fields = vec![];

        for (i, arg) in node.args.iter().enumerate() {
            args_struct_fields.push(ast::StructFieldASTNode {
                position: i as u32,
                name: arg.id.clone(),
                type_id: arg.type_id.clone(),
            });
        }

        let args_struct = ast::StructASTNode {
            id: args_struct_id,
            fields: args_struct_fields,
            emplace_buffers: false,
            into_buffers: true,
        };

        writer.writeln(&generate_struct_model(&args_struct, "", false));
        writer.writeln(&generate_struct_buffers(&args_struct));
    }

    let namespace = ast::find_directive_value(ast, "namespace").expect("namespace is required");

    let namespace = match namespace {
        ast::ConstValueASTNode::Literal {
            literal,
            type_id: _,
        } => {
            match literal {
                Literal::StringLiteral(value) => value,
                _ => panic!("namespace should be a string literal"),
            }
        }
    };

    writer.writeln(&format!(
        "class {}RpcClient implements RpcClient {{",
        namespace.to_case(Case::Pascal)
    ));
    writer.writeln_tab(1, "final VMChannelScheduler _scheduler;");

    if fn_nodes.len() > 1 {
        writer.writeln("");
    }

    for node in fn_nodes.iter() {
        writer.writeln_tab(
            1,
            &format!(
                "StreamController<{}>? _read{}Stream;",
                generate_option_type_id(&node.return_type_id),
                node.id.to_case(Case::Pascal),
            ),
        );
    }

    if fn_nodes.len() > 1 {
        writer.writeln("");
    }

    for node in fn_nodes.iter() {
        writer.writeln_tab(
            1,
            &format!(
                "final _read{}Tasks = <VMChannelReadTask>[];",
                node.id.to_case(Case::Pascal)
            ),
        );
    }

    writer.writeln("");
    writer.writeln_tab(
        1,
        &format!(
            "{}RpcClient(this._scheduler);",
            namespace.to_case(Case::Pascal)
        ),
    );
    writer.writeln("");
    writer.write(&generate_disconnect(&fn_nodes));
    writer.writeln("");

    for (idx, node) in fn_nodes.iter().enumerate() {
        writer.write(&generate_rpc_read(node));
        writer.writeln("");

        if node.return_type_id.is_some() {
            writer.write(&generate_rpc_read_emplace(node));
            writer.writeln("");
        }

        writer.write(&generate_rpc_write(node));
        writer.writeln("");
        writer.write(&generate_rpc_async(node));

        if idx != fn_nodes.len() - 1 {
            writer.writeln("");
        }
    }

    writer.writeln("}");
    writer.show().to_string()
}

fn generate_disconnect(nodes: &[&ast::FnASTNode]) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "void disconnect() {");

    for node in nodes {
        writer.writeln_tab(
            2,
            &format!(
                "for (final task in _read{}Tasks) _scheduler.disconnect(task);",
                node.id.to_case(Case::Pascal)
            ),
        );
    }

    if !nodes.is_empty() {
        writer.writeln("");
    }

    for node in nodes {
        writer.writeln_tab(
            2,
            &format!("_read{}Stream?.close();", node.id.to_case(Case::Pascal)),
        );
    }

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_rpc_read(node: &ast::FnASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(
        1,
        &format!(
            "Stream<{}> read{}() {{",
            generate_option_type_id(&node.return_type_id),
            node.id.to_case(Case::Pascal)
        ),
    );

    writer.writeln_tab(
        2,
        &format!(
            "if (_read{}Stream != null) {{",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(
        3,
        &format!(
            "return _read{}Stream!.stream;",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(2, "}");
    writer.writeln("");

    writer.writeln_tab(
        2,
        &format!(
            "_read{}Stream = StreamController<{}>.broadcast();",
            node.id.to_case(Case::Pascal),
            generate_option_type_id(&node.return_type_id)
        ),
    );
    writer.writeln("");

    writer.writeln_tab(
        2,
        &format!(
            "final task = _scheduler.read(k{}ClientAddress, (reader) {{",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(3, "reader.reset();");
    writer.writeln_tab(3, "final status = reader.readInt8();");
    writer.writeln("");
    writer.writeln_tab(3, "if (status == kStatusReceivedData) {");

    match &node.return_type_id {
        Some(type_id) => {
            writer.writeln_tab(
                4,
                &format!(
                    "_read{}Stream!.add({});",
                    node.id.to_case(Case::Pascal),
                    generate_read(type_id)
                ),
            )
        }
        None => {
            writer.writeln_tab(
                4,
                &format!("_read{}Stream!.add(null);", node.id.to_case(Case::Pascal)),
            )
        }
    }

    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "});");
    writer.writeln("");
    writer.writeln_tab(
        2,
        &format!("_read{}Tasks.add(task);", node.id.to_case(Case::Pascal)),
    );

    writer.writeln_tab(
        2,
        &format!(
            "return _read{}Stream!.stream;",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_rpc_read_emplace(node: &ast::FnASTNode) -> String {
    let mut writer = Writer::new(2);
    let return_type_id = node.return_type_id.as_ref().unwrap();

    writer.writeln_tab(
        1,
        &format!(
            "Stream<{}> read{}Emplace({} model) {{",
            generate_type_id(return_type_id),
            node.id.to_case(Case::Pascal),
            generate_type_id(return_type_id),
        ),
    );

    writer.writeln_tab(
        2,
        &format!(
            "if (_read{}Stream != null) {{",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(
        3,
        &format!(
            "return _read{}Stream!.stream;",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(2, "}");
    writer.writeln("");

    writer.writeln_tab(
        2,
        &format!(
            "_read{}Stream = StreamController<{}>.broadcast();",
            node.id.to_case(Case::Pascal),
            generate_option_type_id(&node.return_type_id)
        ),
    );
    writer.writeln("");

    writer.writeln_tab(
        2,
        &format!(
            "final task = _scheduler.read(k{}ClientAddress, (reader) {{",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(3, "reader.reset();");
    writer.writeln_tab(3, "final status = reader.readInt8();");
    writer.writeln("");
    writer.writeln_tab(3, "if (status == kStatusReceivedData) {");

    writer.writeln_tab(4, &generate_read_emplace(return_type_id, "model"));
    writer.writeln_tab(
        4,
        &format!("_read{}Stream!.add(model);", node.id.to_case(Case::Pascal),),
    );

    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "});");
    writer.writeln("");
    writer.writeln_tab(
        2,
        &format!("_read{}Tasks.add(task);", node.id.to_case(Case::Pascal)),
    );
    writer.writeln_tab(
        2,
        &format!(
            "return _read{}Stream!.stream;",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_rpc_write(node: &ast::FnASTNode) -> String {
    let mut writer = Writer::new(2);

    if node.args.is_empty() {
        writer.writeln_tab(
            1,
            &format!("void write{}() {{", node.id.to_case(Case::Pascal)),
        )
    } else {
        writer.writeln_tab(
            1,
            &format!("void write{}({{", node.id.to_case(Case::Pascal)),
        );

        for arg in node.args.iter() {
            writer.writeln_tab(
                2,
                &format!("required {} {},", generate_type_id(&arg.type_id), arg.id),
            )
        }

        writer.writeln_tab(1, "}) {");
        writer.writeln_tab(2, &format!("final args = __{}_rpc_args__(", node.id));

        for arg in node.args.iter() {
            writer.writeln_tab(3, &format!("{}: {},", arg.id, arg.id))
        }

        writer.writeln_tab(2, ");");
        writer.writeln("");
    }

    writer.writeln_tab(
        2,
        &format!(
            "_scheduler.write(k{}ServerAddress, (writer) {{",
            node.id.to_case(Case::Pascal)
        ),
    );

    writer.writeln_tab(3, "writer.clear();");
    writer.writeln_tab(3, "writer.writeInt8(kStatusReceivedData);");

    if !node.args.is_empty() {
        let type_id = ast::TypeIDASTNode::Other {
            id: format!("__{}_rpc_args__", node.id),
        };
        writer.writeln_tab(3, &generate_write(&type_id, "args"));
    }

    writer.writeln_tab(2, "});");

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_rpc_async(node: &ast::FnASTNode) -> String {
    let mut writer = Writer::new(2);

    if node.args.is_empty() {
        writer.writeln_tab(
            1,
            &format!(
                "Future<{}> {}() {{",
                generate_option_type_id(&node.return_type_id),
                node.id.to_case(Case::Camel)
            ),
        );
        writer.writeln_tab(2, &format!("write{}();", node.id.to_case(Case::Pascal)));
    } else {
        writer.writeln_tab(
            1,
            &format!(
                "Future<{}> {}({{",
                generate_option_type_id(&node.return_type_id),
                node.id.to_case(Case::Camel)
            ),
        );

        for arg in node.args.iter() {
            writer.writeln_tab(
                2,
                &format!("required {} {},", generate_type_id(&arg.type_id), arg.id),
            )
        }

        writer.writeln_tab(1, "}) {");
        writer.writeln_tab(2, &format!("write{}(", node.id.to_case(Case::Pascal)));

        for arg in node.args.iter() {
            writer.writeln_tab(3, &format!("{}: {},", arg.id, arg.id))
        }

        writer.writeln_tab(2, ");");
        writer.writeln("");
    }

    writer.writeln_tab(
        2,
        &format!(
            "final completer = Completer<{}>();",
            generate_option_type_id(&node.return_type_id)
        ),
    );
    writer.writeln("");

    writer.writeln_tab(2, "late VMChannelReadTask task;");
    writer.writeln_tab(
        2,
        &format!(
            "task = _scheduler.read(k{}ClientAddress, (reader) {{",
            node.id.to_case(Case::Pascal)
        ),
    );
    writer.writeln_tab(3, "reader.reset();");
    writer.writeln_tab(3, "final status = reader.readInt8();");
    writer.writeln("");
    writer.writeln_tab(3, "if (status == kStatusReceivedData) {");

    match &node.return_type_id {
        Some(type_id) => {
            writer.writeln_tab(
                4,
                &format!("completer.complete({});", generate_read(type_id)),
            )
        }
        None => writer.writeln_tab(4, "completer.complete();"),
    }

    writer.writeln_tab(4, "_scheduler.disconnect(task);");
    writer.writeln_tab(
        4,
        &format!("_read{}Tasks.remove(task);", node.id.to_case(Case::Pascal)),
    );

    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "});");
    writer.writeln("");
    writer.writeln_tab(
        2,
        &format!("_read{}Tasks.add(task);", node.id.to_case(Case::Pascal)),
    );
    writer.writeln_tab(2, "return completer.future;");
    writer.writeln_tab(1, "}");

    writer.show().to_string()
}
