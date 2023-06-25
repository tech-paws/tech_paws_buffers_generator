pub fn register_rpc(runtime: &mut TechPawsRuntime, addr: &TechPawsRpcAddress) {
    let scope_id = TechPawsScopeId(uuid!("723ca727-6a66-43a7-bfcc-b8ad94eac9be"));
    runtime.memory.add_scope(scope_id);
    runtime.register_async_rpc_method(
        addr.read_group_address,
        scope_id,
        RpcMethodAddress(0),
        counter_rpc_handler,
    );
    runtime.register_async_rpc_method(
        addr.read_group_address,
        scope_id,
        RpcMethodAddress(1),
        theme_rpc_handler,
    );
    runtime.register_async_rpc_method(
        addr.read_group_address,
        scope_id,
        RpcMethodAddress(2),
        async_trigger_rpc_handler,
    );
    runtime.register_async_rpc_method(
        addr.read_group_address,
        scope_id,
        RpcMethodAddress(3),
        async_hello_read_rpc_handler,
    );
}

pub fn counter_rpc_handler(
    scope_id: TechPawsScopeId,
    _: unsafe fn() -> &'static mut TechPawsRuntime,
    memory: &mut TechPawsRuntimeMemory,
    _: &mut TechPawsRuntimeAsyncContext,
    rpc_method_address: RpcMethodAddress,
) -> bool {
    let result = counter();

    if let Some(result) = &result {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
            },
        );
    }

    result.is_some()
}

pub fn theme_rpc_handler(
    scope_id: TechPawsScopeId,
    _: unsafe fn() -> &'static mut TechPawsRuntime,
    memory: &mut TechPawsRuntimeMemory,
    _: &mut TechPawsRuntimeAsyncContext,
    rpc_method_address: RpcMethodAddress,
) -> bool {
    let result = theme();

    if let Some(result) = &result {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
                result.write_to_buffers(bytes_writer);
            },
        );
    }

    result.is_some()
}

pub fn async_trigger_rpc_handler(
    scope_id: TechPawsScopeId,
    runtime_as_mut: unsafe fn() -> &'static mut TechPawsRuntime,
    memory: &mut TechPawsRuntimeMemory,
    async_context: &mut TechPawsRuntimeAsyncContext,
    rpc_method_address: RpcMethodAddress,
) -> bool {
    let is_busy = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Server,
        |bytes_reader| {
            let status = bytes_reader.read_u8();
            status == 0xFF
        },
    );

    if !is_busy {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Server,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
            },
        );

        async_context.async_spawner.spawn(async move {
            let result = async_trigger().await;

            if let Some(result) = &result {
                let memory = unsafe { &mut runtime_as_mut().memory };
                memory.get_scope_mut(scope_id).rpc_buffer_write(
                    rpc_method_address,
                    TechPawsRuntimeRpcMethodBuffer::Client,
                    |bytes_writer| {
                        bytes_writer.write_u8(0xFF);
                    },
                );
                memory.get_scope_mut(scope_id).rpc_buffer_write(
                    rpc_method_address,
                    TechPawsRuntimeRpcMethodBuffer::Server,
                    |bytes_writer| {
                        bytes_writer.write_u8(0x00);
                    },
                );
            }
        });
    }

    !is_busy
}

pub fn async_hello_read_rpc_handler(
    scope_id: TechPawsScopeId,
    runtime_as_mut: unsafe fn() -> &'static mut TechPawsRuntime,
    memory: &mut TechPawsRuntimeMemory,
    async_context: &mut TechPawsRuntimeAsyncContext,
    rpc_method_address: RpcMethodAddress,
) -> bool {
    let is_busy = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Server,
        |bytes_reader| {
            let status = bytes_reader.read_u8();
            status == 0xFF
        },
    );

    if !is_busy {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Server,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
            },
        );

        async_context.async_spawner.spawn(async move {
            let result = async_hello_read().await;

            if let Some(result) = &result {
                let memory = unsafe { &mut runtime_as_mut().memory };
                memory.get_scope_mut(scope_id).rpc_buffer_write(
                    rpc_method_address,
                    TechPawsRuntimeRpcMethodBuffer::Client,
                    |bytes_writer| {
                        bytes_writer.write_u8(0xFF);
                        result.write_to_buffers(bytes_writer);
                    },
                );
                memory.get_scope_mut(scope_id).rpc_buffer_write(
                    rpc_method_address,
                    TechPawsRuntimeRpcMethodBuffer::Server,
                    |bytes_writer| {
                        bytes_writer.write_u8(0x00);
                    },
                );
            }
        });
    }

    !is_busy
}
