use crate::{
    ast::{self, ASTNode, FnASTNode, StructASTNode, StructFieldASTNode, TypeIDASTNode},
    lexer::Literal,
    rust_generator::generate_write,
    writer::Writer,
};

use super::{struct_buffers::generate_struct_buffers, struct_models::generate_struct_model};

pub fn generate_rpc_method(node: &FnASTNode) -> String {
    if node.is_signal {
        generate_stream_rpc_method(node)
    } else if node.is_async {
        panic!("async is not supported");
    } else {
        generate_sync_rpc_method(node)
    }
}

pub fn generate_register_fn(ast: &[ASTNode]) -> String {
    let mut writer = Writer::default();

    writer.writeln("pub fn register_rpc(runtime: &mut TechPawsBuffersRuntime) {");

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

    writer.push_tab();
    writer.writeln(&format!(
        "let scope_id = TechPawsScopeId(uuid!(\"{}\"));",
        id
    ));
    writer.writeln("runtime.memory.add_scope(scope_id);");

    let fn_nodes = ast::find_fn_nodes(ast);

    for node in fn_nodes.iter() {
        let register_method = if node.is_signal {
            "register_signal_rpc_method"
        } else if node.is_async {
            "register_async_rpc_method"
        } else {
            "register_rpc_method"
        };

        let buffer_size = if node.args.is_empty() && node.return_type_id.is_none() {
            "TechPawsRuntimeRpcMethodPayloadSize::Zero"
        } else if let Some(TypeIDASTNode::Generic { id, .. }) = node.return_type_id.clone() {
            if id == "Vec" {
                "TechPawsRuntimeRpcMethodPayloadSize::Large"
            } else {
                "TechPawsRuntimeRpcMethodPayloadSize::Medium"
            }
        } else {
            "TechPawsRuntimeRpcMethodPayloadSize::Medium"
        };

        writer.writeln(&format!("runtime.{}(", register_method));
        writer.push_tab();
        writer.writeln("TechPawsRpcMethod {");
        writer.push_tab();
        writer.writeln("scope_id,");
        writer.writeln(&format!(
            "rpc_method_address: RpcMethodAddress({}),",
            node.position
        ));
        writer.writeln(&format!("handler: {}_rpc_handler,", node.id));
        writer.pop_tab();
        writer.writeln("},");
        writer.writeln(&format!("{buffer_size},"));
        writer.pop_tab();
        writer.writeln(");");
    }

    writer.pop_tab();
    writer.writeln("}");

    writer.show().to_string()
}

fn generate_sync_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::default();

    let args_struct_id = format!("__{}_rpc_args__", node.id);
    let mut args_struct_fields = vec![];

    if !node.args.is_empty() {
        for (i, arg) in node.args.iter().enumerate() {
            args_struct_fields.push(StructFieldASTNode {
                doc_comments: vec![],
                position: i as u32,
                name: arg.id.clone(),
                type_id: arg.type_id.clone(),
            });
        }

        let args_struct = StructASTNode {
            doc_comments: vec![],
            directives: vec![],
            id: args_struct_id.clone(),
            fields: args_struct_fields,
            emplace_buffers: false,
            into_buffers: true,
        };

        writer.writeln(&generate_struct_model(&args_struct, false));
        writer.writeln(&generate_struct_buffers(&args_struct));
    }

    writer.writeln(&format!("pub fn {}_rpc_handler(", node.id));
    writer.push_tab();
    writer.writeln("scope_id: TechPawsScopeId,");
    writer.writeln("memory: &mut TechPawsRuntimeMemory,");
    writer.writeln("rpc_method_address: RpcMethodAddress,");
    writer.pop_tab();
    writer.writeln(") {");

    writer.push_tab();

    if !node.args.is_empty() {
        writer.writeln("let args = memory.get_scope_mut(scope_id).rpc_buffer_read(");
        writer.push_tab();
        writer.writeln("rpc_method_address,");
        writer.writeln("TechPawsRuntimeRpcMethodBuffer::Server,");
        writer.writeln(&format!(
            "|bytes_reader| {}::read_from_buffers(bytes_reader),",
            args_struct_id,
        ));
        writer.pop_tab();
        writer.writeln(");");
        writer.new_line();
    }

    writer.write_tabs();

    if node.return_type_id.is_some() {
        writer.write("let result = ");
    }

    writer.write(&node.id);

    if node.args.is_empty() {
        writer.write("();");
    } else {
        writer.write("(");
        writer.new_line();
        writer.push_tab();

        for arg in &node.args {
            writer.writeln(&format!("args.{},", &arg.id));
        }

        writer.pop_tab();
        writer.write_tabs();
        writer.write(");");
    }

    writer.new_line();

    if let Some(return_type_id) = &node.return_type_id {
        writer.new_line();
        writer.writeln("memory.get_scope_mut(scope_id).rpc_buffer_write(");
        writer.push_tab();
        writer.writeln("rpc_method_address,");
        writer.writeln("TechPawsRuntimeRpcMethodBuffer::Client,");
        writer.writeln("|bytes_writer| {");
        writer.push_tab();
        writer.writeln(&generate_write(return_type_id, "result", false));
        writer.pop_tab();
        writer.writeln("},");
        writer.pop_tab();
        writer.writeln(");");
    }

    writer.pop_tab();
    writer.writeln("}");

    writer.show().to_string()
}

fn generate_stream_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!("pub fn {}_rpc_handler(", node.id));
    writer.push_tab();
    writer.writeln("scope_id: TechPawsScopeId,");
    writer.writeln("memory: &mut TechPawsRuntimeMemory,");
    writer.writeln("rpc_method_address: RpcMethodAddress,");
    writer.pop_tab();
    writer.writeln(") {");

    writer.push_tab();
    writer.write_tabs();

    writer.write("let result = ");
    writer.write(&node.id);

    if node.args.is_empty() {
        writer.write("();");
    }

    writer.new_line();

    writer.new_line();

    if node.return_type_id.is_some() {
        writer.writeln("if let TechPawsSignalRpcResult::Data(result) = result {");
        writer.push_tab();
    } else {
        writer.writeln("if result.has_new_data() {");
        writer.push_tab();
    }

    writer.writeln("memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.push_tab();
    writer.writeln("rpc_method_address,");
    writer.writeln("TechPawsRuntimeRpcMethodBuffer::Client,");
    writer.writeln("|bytes_writer| {");
    writer.push_tab();
    writer.writeln("bytes_writer.write_u8(0xFF);");

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln(&generate_write(return_type_id, "result", false));
    }

    writer.pop_tab();
    writer.writeln("},");
    writer.pop_tab();
    writer.writeln(");");

    writer.pop_tab();
    writer.writeln("}");
    writer.pop_tab();
    writer.writeln("}");

    writer.show().to_string()
}
