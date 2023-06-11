use crate::{ast::{FnASTNode, StructFieldASTNode, StructASTNode, TypeIDASTNode}, writer::Writer, rust_generator::{generate_type_id, generate_read}};

use super::{struct_models::generate_struct_model, struct_buffers::generate_struct_buffers};

const RPC_NEW_DATA_STATUS: &str = "0xFF";
const RPC_NO_DATA_STATUS: &str = "0x00";

pub fn generate_rpc_method(node: &FnASTNode) -> String {
    if node.is_read {
        generate_r_rpc_method(node)
    } else {
        generate_rw_rpc_method(node)
    }
}

fn generate_r_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::default();

    writer.writeln(&format!("pub fn {}_rpc_handler(", node.id));
    writer.writeln_tab(1, "memory: &mut tech_paws_runtime::Memory,");
    writer.writeln_tab(
        1,
        "state_getter: fn() -> &'static mut tech_paws_runtime::State,",
    );
    writer.writeln_tab(1, "cycle_address: tech_paws_runtime::CycleAddress,");
    writer.writeln_tab(
        1,
        "client_buffer_address: tech_paws_runtime::BufferAddress,",
    );
    writer.writeln_tab(
        1,
        "server_buffer_address: tech_paws_runtime::BufferAddress,",
    );
    writer.writeln(") -> bool {");

    writer.writeln_tab(1, "let need_to_process = tech_paws_runtime::buffer_read(");
    writer.writeln_tab(2, "memory,");
    writer.writeln_tab(2, "server_buffer_address,");
    writer.writeln_tab(2, "|bytes_reader| {");
    writer.writeln_tab(3, "bytes_reader.reset();");
    writer.writeln_tab(3, "let status = bytes_reader.read_byte();");
    writer.writeln("");
    writer.writeln_tab(3, &format!("if status == {} {{", RPC_NEW_DATA_STATUS));
    writer.writeln_tab(4, "true");
    writer.writeln_tab(3, "} else {");
    writer.writeln_tab(4, "false");
    writer.writeln_tab(3, "}");
    writer.writeln_tab(2, "},");
    writer.writeln_tab(1, ");");
    writer.writeln("");

    writer.writeln_tab(1, "if need_to_process {");
    writer.writeln_tab(2, "tech_paws_runtime::buffer_write(");
    writer.writeln_tab(3, "memory,");
    writer.writeln_tab(3, "server_buffer_address,");
    writer.writeln_tab(3, "|bytes_writer| {");
    writer.writeln_tab(4, "bytes_writer.clear();");
    writer.writeln_tab(4, "bytes_writer.write_byte(0x00);");
    writer.writeln_tab(3, "},");
    writer.writeln_tab(2, ");");

    writer.writeln("");
    writer.writeln_tab(2, "unsafe {");
    writer.writeln_tab(3, "let state = state_getter();");
    writer.writeln_tab(3, "let cycle = state");
    writer.writeln_tab(4, ".cycles_states");
    writer.writeln_tab(4, ".get_by_id(cycle_address)");
    writer.writeln_tab(4, ".clone()");
    writer.writeln_tab(4, ".data_ptr()");
    writer.writeln_tab(4, ".as_mut()");
    writer.writeln_tab(4, ".unwrap();");
    writer.writeln("");

    writer.writeln_tab(3, "cycle.async_spawner.spawn(async move {");
    writer.writeln_tab(4, "let state = state_getter();");
    writer.writeln_tab(4, "let cycle = state");
    writer.writeln_tab(5, ".cycles_states");
    writer.writeln_tab(5, ".get_by_id(cycle_address)");
    writer.writeln_tab(5, ".clone()");
    writer.writeln_tab(5, ".data_ptr()");
    writer.writeln_tab(5, ".as_mut()");
    writer.writeln_tab(5, ".unwrap();");
    writer.writeln("");

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(
            4,
            &format!(
                "let mut emitter = Emitter::<{}>::new(",
                generate_type_id(return_type_id)
            ),
        );
        writer.writeln_tab(5, "&mut cycle.memory,");
        writer.writeln_tab(5, "client_buffer_address,");
        writer.writeln_tab(4, ");");
        writer.writeln("");

        writer.writeln_tab(4, &format!("{}(&mut emitter).await;", node.id));
    } else {
        writer.writeln_tab(4, "let mut emitter = VoidEmitter::new(");
        writer.writeln_tab(5, "&mut cycle.memory,");
        writer.writeln_tab(5, "client_buffer_address,");
        writer.writeln_tab(4, ");");
        writer.writeln("");
        writer.writeln_tab(4, &format!("{}(&mut emitter).await;", node.id));
    }

    writer.writeln("");
    writer.writeln_tab(4, "tech_paws_runtime::buffer_write(");
    writer.writeln_tab(5, "&mut cycle.memory,");
    writer.writeln_tab(5, "server_buffer_address,");
    writer.writeln_tab(5, "|bytes_writer| {");
    writer.writeln_tab(6, "bytes_writer.clear();");
    writer.writeln_tab(
        6,
        &format!("bytes_writer.write_byte({});", RPC_NEW_DATA_STATUS),
    );
    writer.writeln_tab(5, "},");
    writer.writeln_tab(4, ");");

    writer.writeln_tab(3, "});");
    writer.writeln_tab(2, "}");
    writer.writeln_tab(1, "}");
    writer.writeln("");
    writer.writeln_tab(1, "need_to_process");
    writer.writeln("}");

    writer.show().to_string()
}

fn generate_rw_rpc_method(node: &FnASTNode) -> String {
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
    writer.writeln_tab(1, "memory: &mut tech_paws_runtime::Memory,");
    writer.writeln_tab(
        1,
        "state_getter: fn() -> &'static mut tech_paws_runtime::State,",
    );
    writer.writeln_tab(1, "cycle_address: tech_paws_runtime::CycleAddress,");
    writer.writeln_tab(
        1,
        "client_buffer_address: tech_paws_runtime::BufferAddress,",
    );
    writer.writeln_tab(
        1,
        "server_buffer_address: tech_paws_runtime::BufferAddress,",
    );
    writer.writeln(") -> bool {");

    writer.writeln_tab(
        1,
        "let args = tech_paws_runtime::buffer_read(memory, server_buffer_address, |bytes_reader| {",
    );
    writer.writeln_tab(2, "bytes_reader.reset();");
    writer.writeln_tab(2, "let status = bytes_reader.read_byte();");
    writer.writeln("");
    writer.writeln_tab(2, &format!("if status == {} {{", RPC_NEW_DATA_STATUS));
    writer.writeln_tab(
        3,
        &format!(
            "Some({})",
            &generate_read(&TypeIDASTNode::Other { id: args_struct_id })
        ),
    );
    writer.writeln_tab(2, "} else {");
    writer.writeln_tab(3, "None");
    writer.writeln_tab(2, "}");
    writer.writeln_tab(1, "});");
    writer.writeln("");

    writer.writeln_tab(1, "if let Some(args) = &args {");
    writer.writeln_tab(
        2,
        "tech_paws_runtime::buffer_write(memory, server_buffer_address, |bytes_writer| {",
    );
    writer.writeln_tab(3, "bytes_writer.clear();");
    writer.writeln_tab(
        3,
        &format!("bytes_writer.write_byte({});", RPC_NO_DATA_STATUS),
    );
    writer.writeln_tab(2, "});");

    writer.writeln("");
    writer.writeln_tab(2, "unsafe {");
    writer.writeln_tab(3, "let args = args.clone();");
    writer.writeln_tab(3, "let state = state_getter();");
    writer.writeln_tab(3, "let cycle = state");
    writer.writeln_tab(4, ".cycles_states");
    writer.writeln_tab(4, ".get_by_id(cycle_address)");
    writer.writeln_tab(4, ".clone()");
    writer.writeln_tab(4, ".data_ptr()");
    writer.writeln_tab(4, ".as_mut()");
    writer.writeln_tab(4, ".unwrap();");
    writer.writeln("");

    writer.writeln_tab(3, "cycle.async_spawner.spawn(async move {");
    writer.writeln_tab(4, "let state = state_getter();");
    writer.writeln_tab(4, "let cycle = state");
    writer.writeln_tab(5, ".cycles_states");
    writer.writeln_tab(5, ".get_by_id(cycle_address)");
    writer.writeln_tab(5, ".clone()");
    writer.writeln_tab(5, ".data_ptr()");
    writer.writeln_tab(5, ".as_mut()");
    writer.writeln_tab(5, ".unwrap();");
    writer.writeln("");

    if let Some(return_type_id) = &node.return_type_id {
        writer.writeln_tab(
            4,
            &format!(
                "let mut emitter = Emitter::<{}>::new(",
                generate_type_id(return_type_id)
            ),
        );
        writer.writeln_tab(5, "&mut cycle.memory,");
        writer.writeln_tab(5, "client_buffer_address,");
        writer.writeln_tab(4, ");");
        writer.writeln("");

        writer.writeln_tab(4, &format!("{}(", node.id));
        writer.writeln_tab(5, "&mut emitter,");

        for arg in node.args.iter() {
            writer.writeln_tab(5, &format!("args.clone().{},", arg.id));
        }

        writer.writeln_tab(4, ").await;");
    } else {
        writer.writeln_tab(4, "let mut emitter = VoidEmitter::new(");
        writer.writeln_tab(5, "&mut cycle.memory,");
        writer.writeln_tab(5, "client_buffer_address,");
        writer.writeln_tab(4, ");");
        writer.writeln("");

        writer.writeln_tab(4, &format!("{}(", node.id));
        writer.writeln_tab(5, "&mut emitter,");

        for arg in node.args.iter() {
            writer.writeln_tab(5, &format!("args.clone().{},", arg.id));
        }

        writer.writeln_tab(4, ").await;");
    }

    writer.writeln_tab(3, "});");
    writer.writeln_tab(2, "}");

    writer.writeln_tab(1, "}");
    writer.writeln("");
    writer.writeln_tab(1, "args.is_some()");
    writer.writeln("}");

    writer.show().to_string()
}
