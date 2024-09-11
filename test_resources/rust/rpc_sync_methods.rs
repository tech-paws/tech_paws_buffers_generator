pub fn register_rpc(runtime: &mut TechPawsBuffersRuntime) {
    let scope_id = TechPawsScopeId(uuid!("4de616f8-12c5-4d2c-8d48-9c5fb038991f"));
    runtime.memory.add_scope(scope_id);
    runtime.register_rpc_method(
        TechPawsRpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(0),
            handler: print_hello_world_rpc_handler,
        },
        TechPawsRuntimeRpcMethodPayloadSize::Zero,
    );
    runtime.register_rpc_method(
        TechPawsRpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(1),
            handler: hello_world_rpc_handler,
        },
        TechPawsRuntimeRpcMethodPayloadSize::Medium,
    );
    runtime.register_rpc_method(
        TechPawsRpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(2),
            handler: say_hello_rpc_handler,
        },
        TechPawsRuntimeRpcMethodPayloadSize::Medium,
    );
    runtime.register_rpc_method(
        TechPawsRpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(3),
            handler: sum_rpc_handler,
        },
        TechPawsRuntimeRpcMethodPayloadSize::Medium,
    );
}

pub fn print_hello_world_rpc_handler(
    scope_id: TechPawsScopeId,
    memory: &mut TechPawsRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    print_hello_world();
}

pub fn hello_world_rpc_handler(
    scope_id: TechPawsScopeId,
    memory: &mut TechPawsRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let result = hello_world();

    memory.get_scope_mut(scope_id).rpc_buffer_write(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Client,
        |bytes_writer| {
            result.write_to_buffers(bytes_writer);
        },
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct __say_hello_rpc_args__ {
    pub first_name: String,
    pub last_name: String,
}

impl TechPawsBuffersModel for __say_hello_rpc_args__ {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            first_name: String::read_from_buffers(bytes_reader),
            last_name: String::read_from_buffers(bytes_reader),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        self.first_name.write_to_buffers(bytes_writer);
        self.last_name.write_to_buffers(bytes_writer);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            String::read_from_buffers(bytes_reader);
            String::read_from_buffers(bytes_reader);
        }
    }
}

pub fn say_hello_rpc_handler(
    scope_id: TechPawsScopeId,
    memory: &mut TechPawsRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let args = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Server,
        |bytes_reader| __say_hello_rpc_args__::read_from_buffers(bytes_reader),
    );

    let result = say_hello(
        args.first_name,
        args.last_name,
    );

    memory.get_scope_mut(scope_id).rpc_buffer_write(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Client,
        |bytes_writer| {
            result.write_to_buffers(bytes_writer);
        },
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct __sum_rpc_args__ {
    pub a: i32,
    pub b: f32,
    pub c: f64,
}

impl TechPawsBuffersModel for __sum_rpc_args__ {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            a: bytes_reader.read_i32(),
            b: bytes_reader.read_f32(),
            c: bytes_reader.read_f64(),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_i32(self.a);
        bytes_writer.write_f32(self.b);
        bytes_writer.write_f64(self.c);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_i32();
            bytes_reader.read_f32();
            bytes_reader.read_f64();
        }
    }
}

pub fn sum_rpc_handler(
    scope_id: TechPawsScopeId,
    memory: &mut TechPawsRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let args = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Server,
        |bytes_reader| __sum_rpc_args__::read_from_buffers(bytes_reader),
    );

    let result = sum(
        args.a,
        args.b,
        args.c,
    );

    memory.get_scope_mut(scope_id).rpc_buffer_write(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Client,
        |bytes_writer| {
            bytes_writer.write_f64(result);
        },
    );
}
