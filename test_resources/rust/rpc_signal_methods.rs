pub fn register_test_rpc<R: TestRpc>(runtime: &mut RpcRuntime) {
    let scope_id = BuffersScopeId(uuid!("11111111-1111-1111-1111-111111111111"));
    runtime.memory.add_scope(scope_id);
    runtime.register_signal_rpc_method(
        RpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(0),
            handler: test_rpc_counter_rpc_handler::<R>,
        },
        RpcMethodPayloadSize::Medium,
    );
    runtime.register_signal_rpc_method(
        RpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(1),
            handler: test_rpc_theme_rpc_handler::<R>,
        },
        RpcMethodPayloadSize::Medium,
    );
    runtime.register_signal_rpc_method(
        RpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(2),
            handler: test_rpc_trigger_rpc_handler::<R>,
        },
        RpcMethodPayloadSize::Zero,
    );
}

pub fn test_rpc_counter_rpc_handler<R: TestRpc>(
    scope_id: BuffersScopeId,
    memory: &mut RpcRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let result = R::counter();

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

pub fn test_rpc_theme_rpc_handler<R: TestRpc>(
    scope_id: BuffersScopeId,
    memory: &mut RpcRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let result = R::theme();

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

pub fn test_rpc_trigger_rpc_handler<R: TestRpc>(
    scope_id: BuffersScopeId,
    memory: &mut RpcRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let result = R::trigger();

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
