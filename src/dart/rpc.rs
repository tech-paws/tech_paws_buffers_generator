use crate::ast::*;
use convert_case::{Case, Casing};

use crate::{
    dart_generator::{generate_struct_buffers, generate_struct_model},
    writer::Writer,
};

pub fn generate_rpc_method(node: &FnASTNode) -> String {
    let mut writer = Writer::new(2);

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
        id: args_struct_id,
        fields: args_struct_fields,
        emplace_buffers: false,
        into_buffers: true,
    };

    writer.writeln(&generate_struct_model(&args_struct, "", false));

    writer.writeln(&generate_struct_buffers(&args_struct));

    writer.writeln("class {}RpcClient {");

    writer.show().to_string()
}
