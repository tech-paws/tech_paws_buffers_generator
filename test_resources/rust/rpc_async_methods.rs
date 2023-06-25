pub fn register_rpc(runtime: &mut TechPawsRuntime, addr: &TechPawsRpcAddress) {
    let scope_id = TechPawsScopeId(uuid!("106c2228-ff3b-45c5-8a55-db9c0537f275"));
    runtime.memory.add_scope(scope_id);
    runtime.register_async_rpc_method(
        addr.async_group_address,
        scope_id,
        RpcMethodAddress(0),
        print_hello_world_rpc_handler,
    );
    runtime.register_async_rpc_method(
        addr.async_group_address,
        scope_id,
        RpcMethodAddress(1),
        hello_world_rpc_handler,
    );
    runtime.register_async_rpc_method(
        addr.async_group_address,
        scope_id,
        RpcMethodAddress(2),
        say_hello_rpc_handler,
    );
    runtime.register_async_rpc_method(
        addr.async_group_address,
        scope_id,
        RpcMethodAddress(3),
        sum_rpc_handler,
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct __print_hello_world_rpc_args__ {
    pub __method_id__: i64,
}

impl IntoVMBuffers for __print_hello_world_rpc_args__ {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            __method_id__: bytes_reader.read_i64(),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_i64(self.__method_id__);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_i64();
        }
    }
}

pub fn print_hello_world_rpc_handler(
    scope_id: TechPawsScopeId,
    runtime_as_mut: unsafe fn() -> &'static mut TechPawsRuntime,
    memory: &mut TechPawsRuntimeMemory,
    async_context: &mut TechPawsRuntimeAsyncContext,
    rpc_method_address: RpcMethodAddress,
) -> bool {
    let args = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Server,
        |bytes_reader| {
            let status = bytes_reader.read_u8();

            if status == 0xFF {
                Some(__print_hello_world_rpc_args__::read_from_buffers(bytes_reader))
            } else {
                None
            }
        },
    );

    if let Some(args) = &args {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Server,
            |bytes_writer| {
                bytes_writer.write_u8(0x00);
            },
        );

        let args = args.clone();

        async_context.async_spawner.spawn(async move {
            let memory = unsafe { &mut runtime_as_mut().memory };
            print_hello_world().await;

            memory.get_scope_mut(scope_id).rpc_buffer_write(
                rpc_method_address,
                TechPawsRuntimeRpcMethodBuffer::Client,
                |bytes_writer| {
                    bytes_writer.write_u8(0xFF);
                    bytes_writer.write_i64(args.__method_id__);
                },
            );
        });
    }

    args.is_some()
}

#[derive(Debug, Clone, PartialEq)]
pub struct __hello_world_rpc_args__ {
    pub __method_id__: i64,
}

impl IntoVMBuffers for __hello_world_rpc_args__ {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            __method_id__: bytes_reader.read_i64(),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_i64(self.__method_id__);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_i64();
        }
    }
}

pub fn hello_world_rpc_handler(
    scope_id: TechPawsScopeId,
    runtime_as_mut: unsafe fn() -> &'static mut TechPawsRuntime,
    memory: &mut TechPawsRuntimeMemory,
    async_context: &mut TechPawsRuntimeAsyncContext,
    rpc_method_address: RpcMethodAddress,
) -> bool {
    let args = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Server,
        |bytes_reader| {
            let status = bytes_reader.read_u8();

            if status == 0xFF {
                Some(__hello_world_rpc_args__::read_from_buffers(bytes_reader))
            } else {
                None
            }
        },
    );

    if let Some(args) = &args {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Server,
            |bytes_writer| {
                bytes_writer.write_u8(0x00);
            },
        );

        let args = args.clone();

        async_context.async_spawner.spawn(async move {
            let memory = unsafe { &mut runtime_as_mut().memory };
            let result = hello_world().await;

            memory.get_scope_mut(scope_id).rpc_buffer_write(
                rpc_method_address,
                TechPawsRuntimeRpcMethodBuffer::Client,
                |bytes_writer| {
                    bytes_writer.write_u8(0xFF);
                    bytes_writer.write_i64(args.__method_id__);
                    result.write_to_buffers(bytes_writer);
                },
            );
        });
    }

    args.is_some()
}

#[derive(Debug, Clone, PartialEq)]
pub struct __say_hello_rpc_args__ {
    pub __method_id__: i64,
    pub name: String,
}

impl IntoVMBuffers for __say_hello_rpc_args__ {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            __method_id__: bytes_reader.read_i64(),
            name: String::read_from_buffers(bytes_reader),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_i64(self.__method_id__);
        self.name.write_to_buffers(bytes_writer);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_i64();
            String::read_from_buffers(bytes_reader);
        }
    }
}

pub fn say_hello_rpc_handler(
    scope_id: TechPawsScopeId,
    runtime_as_mut: unsafe fn() -> &'static mut TechPawsRuntime,
    memory: &mut TechPawsRuntimeMemory,
    async_context: &mut TechPawsRuntimeAsyncContext,
    rpc_method_address: RpcMethodAddress,
) -> bool {
    let args = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Server,
        |bytes_reader| {
            let status = bytes_reader.read_u8();

            if status == 0xFF {
                Some(__say_hello_rpc_args__::read_from_buffers(bytes_reader))
            } else {
                None
            }
        },
    );

    if let Some(args) = &args {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Server,
            |bytes_writer| {
                bytes_writer.write_u8(0x00);
            },
        );

        let args = args.clone();

        async_context.async_spawner.spawn(async move {
            let memory = unsafe { &mut runtime_as_mut().memory };
            let result = say_hello(
                args.clone().name,
            ).await;

            memory.get_scope_mut(scope_id).rpc_buffer_write(
                rpc_method_address,
                TechPawsRuntimeRpcMethodBuffer::Client,
                |bytes_writer| {
                    bytes_writer.write_u8(0xFF);
                    bytes_writer.write_i64(args.__method_id__);
                    result.write_to_buffers(bytes_writer);
                },
            );
        });
    }

    args.is_some()
}

#[derive(Debug, Clone, PartialEq)]
pub struct __sum_rpc_args__ {
    pub __method_id__: i64,
    pub a: i32,
    pub b: f32,
    pub c: f64,
}

impl IntoVMBuffers for __sum_rpc_args__ {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            __method_id__: bytes_reader.read_i64(),
            a: bytes_reader.read_i32(),
            b: bytes_reader.read_f32(),
            c: bytes_reader.read_f64(),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_i64(self.__method_id__);
        bytes_writer.write_i32(self.a);
        bytes_writer.write_f32(self.b);
        bytes_writer.write_f64(self.c);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_i64();
            bytes_reader.read_i32();
            bytes_reader.read_f32();
            bytes_reader.read_f64();
        }
    }
}

pub fn sum_rpc_handler(
    scope_id: TechPawsScopeId,
    runtime_as_mut: unsafe fn() -> &'static mut TechPawsRuntime,
    memory: &mut TechPawsRuntimeMemory,
    async_context: &mut TechPawsRuntimeAsyncContext,
    rpc_method_address: RpcMethodAddress,
) -> bool {
    let args = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        TechPawsRuntimeRpcMethodBuffer::Server,
        |bytes_reader| {
            let status = bytes_reader.read_u8();

            if status == 0xFF {
                Some(__sum_rpc_args__::read_from_buffers(bytes_reader))
            } else {
                None
            }
        },
    );

    if let Some(args) = &args {
        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Server,
            |bytes_writer| {
                bytes_writer.write_u8(0x00);
            },
        );

        let args = args.clone();

        async_context.async_spawner.spawn(async move {
            let memory = unsafe { &mut runtime_as_mut().memory };
            let result = sum(
                args.clone().a,
                args.clone().b,
                args.clone().c,
            ).await;

            memory.get_scope_mut(scope_id).rpc_buffer_write(
                rpc_method_address,
                TechPawsRuntimeRpcMethodBuffer::Client,
                |bytes_writer| {
                    bytes_writer.write_u8(0xFF);
                    bytes_writer.write_i64(args.__method_id__);
                    bytes_writer.write_f64(result);
                },
            );
        });
    }

    args.is_some()
}
