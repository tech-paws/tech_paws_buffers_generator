use convert_case::{Case, Casing};

use crate::{
    ast::{self, FnASTNode, StructASTNode, StructFieldASTNode, TraitASTNode, TypeIDASTNode},
    kotlin::ir::generate_type_id,
    rust_generator::generate_write,
    writer::Writer,
};

use super::{struct_buffers::generate_struct_buffers, struct_models::generate_struct_model};

pub fn generate_rpc_method(trait_node: &TraitASTNode, node: &FnASTNode) -> String {
    if node.is_signal {
        generate_signal_rpc_method(trait_node, node)
    } else if node.is_async {
        panic!("async is not supported");
    } else {
        generate_sync_rpc_method(trait_node, node)
    }
}

pub fn generate_trait(node: &TraitASTNode) -> String {
    let mut writer = Writer::default();

    for comment in &node.doc_comments {
        writer.writeln(&format!("///{}", comment));
    }

    writer.writeln(&format!("pub trait {} {{", node.id));
    writer.push_tab();

    for (idx, method) in node.methods.iter().enumerate() {
        for comment in &method.doc_comments {
            writer.writeln(&format!("///{}", comment));
        }

        let mut writer_args = Writer::default();

        if !method.args.is_empty() {
            writer_args.new_line();
            writer_args.push_tab();
            writer_args.push_tab();

            for arg in &method.args {
                writer_args.writeln(&format!("{}: {},", arg.id, generate_type_id(&arg.type_id),));
            }

            writer_args.pop_tab();
            writer_args.write_tabs();
        }

        if method.is_signal {
            if let Some(return_type_id) = &method.return_type_id {
                writer.writeln(&format!(
                    "fn {}({}) -> SignalRpcResult<{}>;",
                    method.id,
                    writer_args.show(),
                    generate_type_id(return_type_id),
                ));
            } else {
                writer.writeln(&format!(
                    "fn {}({}) -> SignalRpcResult<()>;",
                    method.id,
                    writer_args.show(),
                ));
            }
        } else if let Some(return_type_id) = &method.return_type_id {
            writer.writeln(&format!(
                "fn {}({}) -> {};",
                method.id,
                writer_args.show(),
                generate_type_id(return_type_id),
            ));
        } else {
            writer.writeln(&format!("fn {}({});", method.id, writer_args.show(),));
        }

        if idx < node.methods.len() - 1 {
            writer.new_line();
        }
    }

    writer.pop_tab();
    writer.writeln("}");

    writer.show().to_string()
}

pub fn generate_register_fn(trait_node: &TraitASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!(
        "pub fn register_{}<R: {}>(runtime: &mut RpcRuntime) {{",
        trait_node.id.to_case(Case::Snake),
        trait_node.id,
    ));

    let id = ast::get_rpc_scope_id(trait_node);

    writer.push_tab();
    writer.writeln(&format!(
        "let scope_id = BuffersScopeId(uuid!(\"{}\"));",
        id
    ));
    writer.writeln("runtime.memory.add_scope(scope_id);");

    for node in &trait_node.methods {
        let register_method = if node.is_signal {
            "register_signal_rpc_method"
        } else if node.is_async {
            "register_async_rpc_method"
        } else {
            "register_rpc_method"
        };

        let buffer_size = if node.args.is_empty() && node.return_type_id.is_none() {
            "RpcMethodPayloadSize::Zero"
        } else if let Some(TypeIDASTNode::Generic { id, .. }) = node.return_type_id.clone() {
            if id == "Vec" {
                "RpcMethodPayloadSize::Large"
            } else {
                "RpcMethodPayloadSize::Medium"
            }
        } else {
            "RpcMethodPayloadSize::Medium"
        };

        writer.writeln(&format!("runtime.{}(", register_method));
        writer.push_tab();
        writer.writeln("RpcMethod {");
        writer.push_tab();
        writer.writeln("scope_id,");
        writer.writeln(&format!(
            "rpc_method_address: RpcMethodAddress({}),",
            node.position
        ));
        writer.writeln(&format!(
            "handler: {}_{}_rpc_handler::<R>,",
            trait_node.id.to_case(Case::Snake),
            node.id
        ));
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

fn generate_sync_rpc_method(trait_node: &TraitASTNode, node: &FnASTNode) -> String {
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

    writer.writeln(&format!(
        "pub fn {}_{}_rpc_handler<R: {}>(",
        trait_node.id.to_case(Case::Snake),
        node.id,
        trait_node.id,
    ));
    writer.push_tab();
    writer.writeln("scope_id: BuffersScopeId,");
    writer.writeln("memory: &mut RpcRuntimeMemory,");
    writer.writeln("rpc_method_address: RpcMethodAddress,");
    writer.pop_tab();
    writer.writeln(") {");

    writer.push_tab();

    if !node.args.is_empty() {
        writer.writeln("let args = memory.get_scope_mut(scope_id).rpc_buffer_read(");
        writer.push_tab();
        writer.writeln("rpc_method_address,");
        writer.writeln("RpcMethodBuffer::Server,");
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

    writer.write(&format!("R::{}", node.id));

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
        writer.writeln("RpcMethodBuffer::Client,");
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

fn generate_signal_rpc_method(trait_node: &TraitASTNode, node: &FnASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!(
        "pub fn {}_{}_rpc_handler<R: {}>(",
        trait_node.id.to_case(Case::Snake),
        node.id,
        trait_node.id,
    ));

    writer.push_tab();
    writer.writeln("scope_id: BuffersScopeId,");
    writer.writeln("memory: &mut RpcRuntimeMemory,");
    writer.writeln("rpc_method_address: RpcMethodAddress,");
    writer.pop_tab();
    writer.writeln(") {");

    writer.push_tab();
    writer.write_tabs();

    writer.write("let result = ");
    writer.write(&format!("R::{}", node.id));

    if node.args.is_empty() {
        writer.write("();");
    }

    writer.new_line();

    writer.new_line();

    if node.return_type_id.is_some() {
        writer.writeln("if let SignalRpcResult::Data(result) = result {");
        writer.push_tab();
    } else {
        writer.writeln("if result.has_new_data() {");
        writer.push_tab();
    }

    writer.writeln("memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.push_tab();
    writer.writeln("rpc_method_address,");
    writer.writeln("RpcMethodBuffer::Client,");
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
