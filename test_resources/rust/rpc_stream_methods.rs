pub fn register_rpc(runtime: &mut RpcRuntime) {
    let scope_id = BuffersScopeId(uuid!("723ca727-6a66-43a7-bfcc-b8ad94eac9be"));
    runtime.memory.add_scope(scope_id);
    runtime.register_signal_rpc_method(
        RpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(0),
            handler: counter_rpc_handler,
        },
        RpcMethodPayloadSize::Medium,
    );
    runtime.register_signal_rpc_method(
        RpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(1),
            handler: theme_rpc_handler,
        },
        RpcMethodPayloadSize::Medium,
    );
    runtime.register_signal_rpc_method(
        RpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(2),
            handler: trigger_rpc_handler,
        },
        RpcMethodPayloadSize::Zero,
    );
}

pub fn counter_rpc_handler(
    scope_id: BuffersScopeId,
    memory: &mut RpcRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let result = counter();

    if let SignalRpcResult::Data(result) = result {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            RpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
                bytes_writer.write_i32(result);
            },
        );
    }
}

pub fn theme_rpc_handler(
    scope_id: BuffersScopeId,
    memory: &mut RpcRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let result = theme();

    if let SignalRpcResult::Data(result) = result {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            RpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
                result.write_to_buffers(bytes_writer);
            },
        );
    }
}

pub fn trigger_rpc_handler(
    scope_id: BuffersScopeId,
    memory: &mut RpcRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let result = trigger();

    if result.has_new_data() {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            RpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
            },
        );
    }
}
