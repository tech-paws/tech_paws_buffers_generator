use crate::{
    ast::{self, ASTNode, FnASTNode, StructASTNode, StructFieldASTNode, TypeIDASTNode},
    lexer::Literal,
    rust_generator::generate_write,
    rust_generator::{generate_read, generate_type_id},
    writer::Writer,
};

use super::{struct_buffers::generate_struct_buffers, struct_models::generate_struct_model};

const RPC_NEW_DATA_STATUS: &str = "0xFF";
const RPC_NO_DATA_STATUS: &str = "0x00";
const RPC_READ_BUSY_STATUS: &str = "0xFF";
const RPC_READ_FREE_STATUS: &str = "0x00";

pub fn generate_rpc_method(node: &FnASTNode) -> String {
    if node.is_read {
        if node.is_async {
            generate_async_read_rpc_method(node)
        } else {
            generate_sync_read_rpc_method(node)
        }
    } else {
        if node.is_async {
            generate_async_rpc_method(node)
        } else {
            generate_sync_rpc_method(node)
        }
    }
}

pub fn generate_register_fn(ast: &[ASTNode]) -> String {
    let mut writer = Writer::default();

    writer.writeln("pub fn register_rpc(");
    writer.writeln_tab(1, "runtime: &mut TechPawsRuntime,");
    writer.writeln_tab(1, "sync_group_address: GroupAddress,");
    writer.writeln_tab(1, "async_group_address: GroupAddress,");
    writer.writeln_tab(1, "read_group_address: GroupAddress,");
    writer.writeln(") {");

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

    writer.writeln_tab(
        1,
        &format!("let scope_id = TechPawsScopeId(uuid!(\"{}\"));", id),
    );
    writer.writeln_tab(1, "runtime.memory.add_scope(scope_id);");

    let fn_nodes = ast::find_fn_nodes(ast);
    let mut fn_id = 0;

    for node in fn_nodes.iter() {
        let (group_address, register_method) = if node.is_read {
            ("read_group_address", "register_async_rpc_method")
        } else {
            if node.is_async {
                ("async_group_address", "register_async_rpc_method")
            } else {
                ("sync_group_address", "register_sync_rpc_method")
            }
        };

        writer.writeln_tab(1, &format!("runtime.{}(", register_method));
        writer.writeln_tab(2, &format!("{},", group_address));
        writer.writeln_tab(2, "scope_id,");
        writer.writeln_tab(2, &format!("RpcMethodAddress({}),", fn_id));
        writer.writeln_tab(2, &format!("{}_rpc_handler,", node.id));
        writer.writeln_tab(1, ");");

        fn_id += 1;
    }

    writer.writeln("}");

    writer.show().to_string()
}

fn generate_sync_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::default();

    let args_struct_id = format!("__{}_rpc_args__", node.id);
    let mut args_struct_fields = vec![];

    for (i, arg) in node.args.iter().enumerate() {
        args_struct_fields.push(StructFieldASTNode {
            position: i as u32,
            name: arg.id.clone(),
            type_id: arg.type_id.clone(),
        });
    }

    let args_struct = StructASTNode {
        id: args_struct_id.clone(),
        fields: args_struct_fields,
        emplace_buffers: false,
        into_buffers: true,
    };

    writer.writeln(&generate_struct_model(&args_struct, false));
    writer.writeln(&generate_struct_buffers(&args_struct));

    writer.writeln(&format!("pub fn {}_rpc_handler(", node.id));
    writer.writeln_tab(1, "scope_id: TechPawsScopeId,");
    writer.writeln_tab(1, "memory: &mut TechPawsRuntimeMemory,");
    writer.writeln_tab(1, "rpc_method_address: RpcMethodAddress,");
    writer.writeln(") -> bool {");

    // Get args
    writer.writeln_tab(
        1,
        "let args = memory.get_scope_mut(scope_id).rpc_buffer_read(",
    );
    writer.writeln_tab(2, "rpc_method_address,");
    writer.writeln_tab(2, "TechPawsRuntimeRpcMethodBuffer::Server,");
    writer.writeln_tab(2, "|bytes_reader| {");
    writer.writeln_tab(3, "let status = bytes_reader.read_u8();");
    writer.writeln("");
    writer.writeln_tab(3, &format!("if status == {} {{", RPC_NEW_DATA_STATUS));
    writer.writeln_tab(
        4,
        &format!("Some({}::read_from_buffers(bytes_reader))", args_struct_id),
    );
    writer.writeln_tab(3, "} else {");
    writer.writeln_tab(4, "None");
    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "},");
    writer.writeln_tab(1, ");");
    writer.writeln("");

    // Execute
    writer.writeln_tab(1, "if let Some(args) = &args {");
    writer.writeln_tab(2, "memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.writeln_tab(3, "rpc_method_address,");
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer::Server,");
    writer.writeln_tab(3, "|bytes_writer| {");
    writer.writeln_tab(
        4,
        &format!("bytes_writer.write_u8({});", RPC_NO_DATA_STATUS),
    );
    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");
    writer.writeln("");

    if node.args.is_empty() {
        if node.return_type_id.is_none() {
            writer.writeln_tab(2, &format!("{}();", node.id));
        } else {
            writer.writeln_tab(2, &format!("let result = {}();", node.id));
        }
    } else {
        if node.return_type_id.is_none() {
            writer.writeln_tab(2, &format!("{}(", node.id));
        } else {
            writer.writeln_tab(2, &format!("let result = {}(", node.id));
        }

        for arg in node.args.iter() {
            writer.writeln_tab(3, &format!("args.clone().{},", arg.id));
        }

        writer.writeln_tab(2, ");")
    }

    writer.writeln("");
    writer.writeln_tab(2, "memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.writeln_tab(3, "rpc_method_address,");
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer::Client,");
    writer.writeln_tab(3, "|bytes_writer| {");
    writer.writeln_tab(
        4,
        &format!("bytes_writer.write_u8({});", RPC_NEW_DATA_STATUS),
    );

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(4, &generate_write(return_type_id, "result", false));
    }

    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");
    writer.writeln_tab(1, "}");

    writer.writeln("");
    writer.writeln_tab(1, "args.is_some()");
    writer.writeln("}");

    writer.show().to_string()
}

fn generate_async_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::default();

    let args_struct_id = format!("__{}_rpc_args__", node.id);
    let mut args_struct_fields = vec![];

    args_struct_fields.push(StructFieldASTNode {
        position: 0,
        name: String::from("__method_id__"),
        type_id: TypeIDASTNode::Number {
            id: String::from("i64"),
            size: 8,
        },
    });

    for (i, arg) in node.args.iter().enumerate() {
        args_struct_fields.push(StructFieldASTNode {
            position: i as u32 + 1,
            name: arg.id.clone(),
            type_id: arg.type_id.clone(),
        });
    }

    let args_struct = StructASTNode {
        id: args_struct_id.clone(),
        fields: args_struct_fields,
        emplace_buffers: false,
        into_buffers: true,
    };

    writer.writeln(&generate_struct_model(&args_struct, false));
    writer.writeln(&generate_struct_buffers(&args_struct));

    writer.writeln(&format!("pub fn {}_rpc_handler(", node.id));
    writer.writeln_tab(1, "scope_id: TechPawsScopeId,");
    writer.writeln_tab(
        1,
        "runtime_as_mut: unsafe fn() -> &'static mut TechPawsRuntime,",
    );
    writer.writeln_tab(1, "memory: &mut TechPawsRuntimeMemory,");
    writer.writeln_tab(1, "async_context: &mut TechPawsRuntimeAsyncContext,");
    writer.writeln_tab(1, "rpc_method_address: RpcMethodAddress,");
    writer.writeln(") -> bool {");

    // Get args
    writer.writeln_tab(
        1,
        "let args = memory.get_scope_mut(scope_id).rpc_buffer_read(",
    );
    writer.writeln_tab(2, "rpc_method_address,");
    writer.writeln_tab(2, "TechPawsRuntimeRpcMethodBuffer::Server,");
    writer.writeln_tab(2, "|bytes_reader| {");
    writer.writeln_tab(3, "let status = bytes_reader.read_u8();");
    writer.writeln("");
    writer.writeln_tab(3, &format!("if status == {} {{", RPC_NEW_DATA_STATUS));
    writer.writeln_tab(
        4,
        &format!("Some({}::read_from_buffers(bytes_reader))", args_struct_id),
    );
    writer.writeln_tab(3, "} else {");
    writer.writeln_tab(4, "None");
    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "},");
    writer.writeln_tab(1, ");");
    writer.writeln("");

    // Execute
    writer.writeln_tab(1, "if let Some(args) = &args {");
    writer.writeln_tab(2, "memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.writeln_tab(3, "rpc_method_address,");
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer::Server,");
    writer.writeln_tab(3, "|bytes_writer| {");
    writer.writeln_tab(
        4,
        &format!("bytes_writer.write_u8({});", RPC_NO_DATA_STATUS),
    );
    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");
    writer.writeln("");

    writer.writeln_tab(2, "let args = args.clone();");
    writer.writeln("");
    writer.writeln_tab(2, "async_context.async_spawner.spawn(async move {");
    writer.writeln_tab(3, "let memory = unsafe { &mut runtime_as_mut().memory };");

    if node.args.is_empty() {
        if node.return_type_id.is_none() {
            writer.writeln_tab(3, &format!("{}().await;", node.id));
        } else {
            writer.writeln_tab(3, &format!("let result = {}().await;", node.id));
        }
    } else {
        if node.return_type_id.is_none() {
            writer.writeln_tab(3, &format!("{}(", node.id));
        } else {
            writer.writeln_tab(3, &format!("let result = {}(", node.id));
        }

        for arg in node.args.iter() {
            writer.writeln_tab(4, &format!("args.clone().{},", arg.id));
        }

        writer.writeln_tab(3, ").await;")
    }

    writer.writeln("");
    writer.writeln_tab(3, "memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.writeln_tab(4, "rpc_method_address,");
    writer.writeln_tab(4, "TechPawsRuntimeRpcMethodBuffer::Client,");
    writer.writeln_tab(4, "|bytes_writer| {");
    writer.writeln_tab(
        5,
        &format!("bytes_writer.write_u8({});", RPC_NEW_DATA_STATUS),
    );
    writer.writeln_tab(5, "bytes_writer.write_i64(args.__method_id__);");

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(5, &generate_write(return_type_id, "result", false));
    }

    writer.writeln_tab(4, "},");
    writer.writeln_tab(3, ");");

    writer.writeln_tab(2, "});");
    writer.writeln_tab(1, "}");

    writer.writeln("");
    writer.writeln_tab(1, "args.is_some()");
    writer.writeln("}");

    writer.show().to_string()
}

fn generate_sync_read_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!("pub fn {}_rpc_handler(", node.id));
    writer.writeln_tab(1, "scope_id: TechPawsScopeId,");
    writer.writeln_tab(1, "_: unsafe fn() -> &'static mut TechPawsRuntime,");
    writer.writeln_tab(1, "memory: &mut TechPawsRuntimeMemory,");
    writer.writeln_tab(1, "_: &mut TechPawsRuntimeAsyncContext,");
    writer.writeln_tab(1, "rpc_method_address: RpcMethodAddress,");
    writer.writeln(") -> bool {");

    writer.writeln_tab(1, &format!("let result = {}();", node.id));
    writer.writeln("");
    writer.writeln_tab(1, "if let Some(result) = &result {");
    writer.writeln_tab(2, "memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.writeln_tab(3, "rpc_method_address,");
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer::Client,");
    writer.writeln_tab(3, "|bytes_writer| {");
    writer.writeln_tab(
        4,
        &format!("bytes_writer.write_u8({});", RPC_NEW_DATA_STATUS),
    );

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(4, &generate_write(return_type_id, "result", false));
    }

    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");
    writer.writeln_tab(1, "}");
    writer.writeln("");

    writer.writeln_tab(1, "result.is_some()");
    writer.writeln("}");

    writer.show().to_string()
}

fn generate_async_read_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!("pub fn {}_rpc_handler(", node.id));
    writer.writeln_tab(1, "scope_id: TechPawsScopeId,");
    writer.writeln_tab(
        1,
        "runtime_as_mut: unsafe fn() -> &'static mut TechPawsRuntime,",
    );
    writer.writeln_tab(1, "memory: &mut TechPawsRuntimeMemory,");
    writer.writeln_tab(1, "async_context: &mut TechPawsRuntimeAsyncContext,");
    writer.writeln_tab(1, "rpc_method_address: RpcMethodAddress,");
    writer.writeln(") -> bool {");

    writer.writeln_tab(
        1,
        "let is_busy = memory.get_scope_mut(scope_id).rpc_buffer_read(",
    );
    writer.writeln_tab(2, "rpc_method_address,");
    writer.writeln_tab(2, "TechPawsRuntimeRpcMethodBuffer::Server,");
    writer.writeln_tab(2, "|bytes_reader| {");
    writer.writeln_tab(3, "let status = bytes_reader.read_u8();");
    writer.writeln_tab(3, &format!("status == {}", RPC_READ_BUSY_STATUS));
    writer.writeln_tab(2, "},");
    writer.writeln_tab(1, ");");
    writer.writeln("");

    writer.writeln_tab(1, "if !is_busy {");
    writer.writeln_tab(2, "memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.writeln_tab(3, "rpc_method_address,");
    writer.writeln_tab(3, "TechPawsRuntimeRpcMethodBuffer::Server,");
    writer.writeln_tab(3, "|bytes_writer| {");
    writer.writeln_tab(
        4,
        &format!("bytes_writer.write_u8({});", RPC_READ_BUSY_STATUS),
    );
    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");
    writer.writeln("");
    writer.writeln_tab(2, "async_context.async_spawner.spawn(async move {");
    writer.writeln_tab(3, &format!("let result = {}().await;", node.id));
    writer.writeln("");
    writer.writeln_tab(3, "if let Some(result) = &result {");
    writer.writeln_tab(4, "let memory = unsafe { &mut runtime_as_mut().memory };");
    writer.writeln_tab(4, "memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.writeln_tab(5, "rpc_method_address,");
    writer.writeln_tab(5, "TechPawsRuntimeRpcMethodBuffer::Client,");
    writer.writeln_tab(5, "|bytes_writer| {");
    writer.writeln_tab(
        6,
        &format!("bytes_writer.write_u8({});", RPC_NEW_DATA_STATUS),
    );

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(6, &generate_write(return_type_id, "result", false));
    }

    writer.writeln_tab(5, "},");
    writer.writeln_tab(4, ");");
    writer.writeln_tab(4, "memory.get_scope_mut(scope_id).rpc_buffer_write(");
    writer.writeln_tab(5, "rpc_method_address,");
    writer.writeln_tab(5, "TechPawsRuntimeRpcMethodBuffer::Server,");
    writer.writeln_tab(5, "|bytes_writer| {");
    writer.writeln_tab(
        6,
        &format!("bytes_writer.write_u8({});", RPC_READ_FREE_STATUS),
    );
    writer.writeln_tab(5, "},");
    writer.writeln_tab(4, ");");
    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "});");
    writer.writeln_tab(1, "}");
    writer.writeln("");
    writer.writeln_tab(1, "!is_busy");
    writer.writeln("}");

    writer.show().to_string()
}
