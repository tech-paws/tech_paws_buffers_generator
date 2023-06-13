use crate::{
    ast::{self, FnASTNode},
    dart_generator::{generate_option_type_id, generate_read, generate_type_id, generate_write},
    lexer::Literal,
};
use convert_case::{Case, Casing};

use crate::writer::Writer;

fn var_last_method_id(node: &FnASTNode) -> String {
    format!("_last{}MethodId", node.id.to_case(Case::Pascal))
}

fn var_method_completers(node: &FnASTNode) -> String {
    format!("_{}Completers", node.id.to_case(Case::Camel))
}

fn var_read_task(node: &FnASTNode) -> String {
    format!("_read{}Task", node.id.to_case(Case::Pascal))
}

fn var_read_stream_controller(node: &FnASTNode) -> String {
    format!("_read{}StreamController", node.id.to_case(Case::Pascal))
}

pub fn generate_rpc_methods(ast: &[ast::ASTNode]) -> String {
    let mut writer = Writer::new(2);

    let fn_nodes = ast::find_fn_nodes(ast);

    if fn_nodes.is_empty() {
        return String::from("");
    }

    let namespace = ast::find_directive_value(ast, "namespace").expect("namespace is required");
    let namespace = match namespace {
        ast::ConstValueASTNode::Literal {
            literal,
            type_id: _,
        } => match literal {
            Literal::StringLiteral(value) => value,
            _ => panic!("namespace should be a string literal"),
        },
    };

    let id = ast::find_directive_value(ast, "id").expect("id is required");
    let id = match id {
        ast::ConstValueASTNode::Literal {
            literal,
            type_id: _,
        } => match literal {
            Literal::StringLiteral(value) => value,
            _ => panic!("id should be a string literal"),
        },
    };

    writer.writeln(&format!(
        "class {}RpcClient {{",
        namespace.to_case(Case::Pascal)
    ));

    let constructor = generate_constructor(&fn_nodes);

    if constructor.is_empty() {
        writer.writeln_tab(
            1,
            &format!(
                "{}RpcClient(this._scheduler);",
                namespace.to_case(Case::Pascal)
            ),
        );
    } else {
        writer.writeln_tab(
            1,
            &format!(
                "{}RpcClient(this._scheduler) {{",
                namespace.to_case(Case::Pascal)
            ),
        );
        writer.write(&constructor);
        writer.writeln_tab(1, "}");
    }

    writer.writeln("");
    writer.writeln_tab(1, "final TechPawsRuntimeRpcScheduler _scheduler;");
    writer.writeln_tab(1, &format!("static const _scopeId = '{}';", id));

    for node in &fn_nodes {
        if node.is_async || node.is_read {
            writer.writeln("");
        }

        if node.is_async && !node.is_read {
            writer.writeln_tab(1, &format!("int {} = 0;", var_last_method_id(node)));
            writer.writeln_tab(
                1,
                &format!(
                    "final {} = <int, Completer<{}>>{{}};",
                    var_method_completers(node),
                    generate_option_type_id(&node.return_type_id)
                ),
            );
        }

        if node.is_async || node.is_read {
            writer.writeln_tab(
                1,
                &format!(
                    "late final TechPawsRuntimeRpcReadTask {};",
                    var_read_task(node),
                ),
            );
        }

        if node.is_read {
            writer.writeln_tab(
                1,
                &format!(
                    "final {} = StreamController<{}>.broadcast();",
                    var_read_stream_controller(node),
                    generate_option_type_id(&node.return_type_id),
                ),
            );
        }
    }

    for node in &fn_nodes {
        if node.is_read {
            writer.writeln("");

            writer.writeln_tab(
                1,
                &format!(
                    "Stream<{}> get {} => {}.stream;",
                    generate_option_type_id(&node.return_type_id),
                    node.id.to_case(Case::Camel),
                    var_read_stream_controller(node),
                ),
            );
        }
    }

    writer.writeln("");
    writer.write(&generate_disconnect(&fn_nodes));

    if fn_nodes.len() > 1 {
        writer.writeln("");
    }

    for node in fn_nodes.iter() {
        let method = if node.is_read {
            generate_read_rpc_method(node)
        } else if node.is_async {
            generate_async_rpc_method(node)
        } else {
            generate_sync_rpc_method(node)
        };

        writer.write(&method);

        if node.position != fn_nodes.last().unwrap().position {
            writer.writeln("");
        }
    }

    writer.writeln("}");
    writer.show().to_string()
}

fn generate_constructor(nodes: &[&ast::FnASTNode]) -> String {
    let mut writer = Writer::new(2);

    for node in nodes {
        if node.is_async || node.is_read {
            writer.writeln_tab(2, &format!("_connect{}();", node.id.to_case(Case::Pascal)));
        }
    }

    writer.show().to_string()
}

fn generate_disconnect(nodes: &[&ast::FnASTNode]) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(1, "void disconnect() {");

    for node in nodes {
        if node.is_async || node.is_read {
            writer.writeln_tab(
                2,
                &format!("_scheduler.disconnect({});", var_read_task(node)),
            );
        }

        if node.is_read {
            writer.writeln_tab(2, &format!("{}.close();", var_read_stream_controller(node)));
        }
    }

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_sync_rpc_method(node: &ast::FnASTNode) -> String {
    let mut writer = Writer::new(2);
    let return_type = generate_option_type_id(&node.return_type_id);

    if node.args.is_empty() {
        writer.writeln_tab(
            1,
            &format!("{} {}() {{", return_type, node.id.to_case(Case::Camel)),
        )
    } else {
        writer.writeln_tab(
            1,
            &format!("{} {}({{", return_type, node.id.to_case(Case::Camel)),
        );
        writer.write(&generate_fn_args(node));
        writer.writeln_tab(1, "}) {");
    }

    writer.writeln_tab(2, "_scheduler.syncWrite(");
    writer.writeln_tab(3, "_scopeId,");
    writer.writeln_tab(3, &format!("{},", node.position));
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer.server,");
    writer.writeln_tab(3, "(writer) {");
    writer.writeln_tab(
        4,
        "writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);",
    );

    for arg in &node.args {
        writer.writeln_tab(
            4,
            &generate_write(&arg.type_id, &arg.id.to_case(Case::Camel)),
        );
    }

    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");
    writer.writeln("");
    writer.writeln_tab(2, "_scheduler.loopSyncGroup();");
    writer.writeln("");

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(
            2,
            &format!(
                "final result = _scheduler.syncRead<{}>(",
                generate_type_id(return_type_id)
            ),
        );
        writer.writeln_tab(3, "_scopeId,");
        writer.writeln_tab(3, &format!("{},", node.position));
        writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer.client,");
        writer.writeln_tab(3, "(reader) {");
        writer.writeln_tab(4, "final status = reader.readInt8();");
        writer.writeln("");
        writer.writeln_tab(4, "if (status != TechPawsRpcBufferStatus.hasData.value) {");
        writer.writeln_tab(5, "throw StateError('No data');");
        writer.writeln_tab(4, "}");
        writer.writeln("");
        writer.writeln_tab(4, &format!("return {};", generate_read(return_type_id)));
        writer.writeln_tab(3, "},");
        writer.writeln_tab(2, ");");
        writer.writeln("");
        writer.writeln_tab(2, "return result;");
    }

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_fn_args(node: &ast::FnASTNode) -> String {
    let mut writer = Writer::new(2);

    for arg in node.args.iter() {
        writer.writeln_tab(
            2,
            &format!(
                "required {} {},",
                generate_type_id(&arg.type_id),
                arg.id.to_case(Case::Camel)
            ),
        )
    }

    writer.show().to_string()
}

fn generate_async_rpc_method(node: &ast::FnASTNode) -> String {
    let mut writer = Writer::new(2);
    let return_type = generate_option_type_id(&node.return_type_id);

    writer.writeln(&generate_async_rpc_connect_method(node));

    if node.args.is_empty() {
        writer.writeln_tab(
            1,
            &format!(
                "Future<{}> {}() {{",
                return_type,
                node.id.to_case(Case::Camel)
            ),
        )
    } else {
        writer.writeln_tab(
            1,
            &format!(
                "Future<{}> {}({{",
                return_type,
                node.id.to_case(Case::Camel)
            ),
        );
        writer.write(&generate_fn_args(node));
        writer.writeln_tab(1, "}) {");
    }

    let method_id_var = var_last_method_id(node);

    writer.writeln_tab(2, &format!("final methodId = {};", method_id_var));
    writer.writeln_tab(
        2,
        &format!("{} = rotateMethodId({});", method_id_var, method_id_var),
    );

    writer.writeln("");
    writer.writeln_tab(2, "_scheduler.write(");
    writer.writeln_tab(3, "_scopeId,");
    writer.writeln_tab(3, &format!("{},", node.position));
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer.server,");
    writer.writeln_tab(3, "(writer) {");
    writer.writeln_tab(
        4,
        "writer.writeInt8(TechPawsRpcBufferStatus.hasData.value);",
    );
    writer.writeln_tab(4, "writer.writeInt64(methodId);");

    for arg in &node.args {
        writer.writeln_tab(
            4,
            &generate_write(&arg.type_id, &arg.id.to_case(Case::Camel)),
        );
    }

    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");
    writer.writeln("");
    writer.writeln_tab(
        2,
        &format!(
            "final completer = Completer<{}>();",
            generate_option_type_id(&node.return_type_id)
        ),
    );
    writer.writeln_tab(
        2,
        &format!("{}[methodId] = completer;", var_method_completers(node)),
    );
    writer.writeln("");
    writer.writeln_tab(2, "return completer.future;");

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_async_rpc_connect_method(node: &FnASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(
        1,
        &format!("void _connect{}() {{", node.id.to_case(Case::Pascal)),
    );

    writer.writeln_tab(2, &format!("{} = _scheduler.read(", var_read_task(node)));
    writer.writeln_tab(3, "_scopeId,");
    writer.writeln_tab(3, &format!("{},", node.position));
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer.client,");
    writer.writeln_tab(3, "(reader) {");
    writer.writeln_tab(4, "final status = reader.readInt8();");
    writer.writeln("");
    writer.writeln_tab(4, "if (status != TechPawsRpcBufferStatus.hasData.value) {");
    writer.writeln_tab(5, "return;");
    writer.writeln_tab(4, "}");
    writer.writeln("");
    writer.writeln_tab(4, "final methodId = reader.readInt64();");

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(
            4,
            &format!("final result = {};", generate_read(return_type_id)),
        );
        writer.writeln("");
        writer.writeln_tab(
            4,
            &format!(
                "{}[methodId]?.complete(result);",
                var_method_completers(node)
            ),
        );
    } else {
        writer.writeln("");
        writer.writeln_tab(
            4,
            &format!("{}[methodId]?.complete();", var_method_completers(node)),
        );
    }

    writer.writeln_tab(
        4,
        &format!("{}.remove(methodId);", var_method_completers(node)),
    );
    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}

fn generate_read_rpc_method(node: &ast::FnASTNode) -> String {
    let mut writer = Writer::new(2);

    writer.writeln_tab(
        1,
        &format!("void _connect{}() {{", node.id.to_case(Case::Pascal)),
    );

    writer.writeln_tab(2, &format!("{} = _scheduler.read(", var_read_task(node)));
    writer.writeln_tab(3, "_scopeId,");
    writer.writeln_tab(3, &format!("{},", node.position));
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer.client,");
    writer.writeln_tab(3, "(reader) {");
    writer.writeln_tab(4, "final status = reader.readInt8();");
    writer.writeln("");
    writer.writeln_tab(4, "if (status != TechPawsRpcBufferStatus.hasData.value) {");
    writer.writeln_tab(5, "return;");
    writer.writeln_tab(4, "}");
    writer.writeln("");

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(
            4,
            &format!("final result = {};", generate_read(return_type_id)),
        );
        writer.writeln_tab(
            4,
            &format!("{}.add(result);", var_read_stream_controller(node)),
        );
    } else {
        writer.writeln_tab(
            4,
            &format!("{}.add(null);", var_read_stream_controller(node)),
        );
    }

    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");

    writer.writeln_tab(1, "}");

    writer.show().to_string()
}
