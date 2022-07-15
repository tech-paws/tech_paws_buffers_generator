#[derive(Debug, Clone, PartialEq)]
pub struct __print_hello_world_rpc_args__;

impl IntoVMBuffers for __print_hello_world_rpc_args__ {
    fn read_from_buffers(_: &mut BytesReader) -> Self {
        __print_hello_world_rpc_args__
    }

    fn write_to_buffers(&self, _: &mut BytesWriter) {}

    fn skip_in_buffers(_: &mut BytesReader, _: u64) {}
}

pub fn print_hello_world_rpc_handler(
    state: &mut vm::CycleState,
    client_buffer_address: vm::BufferAddress,
    server_buffer_address: vm::BufferAddress,
) -> bool {
    let args = vm::buffer_read(state, server_buffer_address, |bytes_reader| {
        bytes_reader.reset();
        let status = bytes_reader.read_byte();

        if status == 0xFF {
            Some(__print_hello_world_rpc_args__::read_from_buffers(bytes_reader))
        } else {
            None
        }
    });

    if let Some(args) = &args {
        vm::buffer_write(state, server_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0x00);
        });
        print_hello_world(
            state,
        );
    }

    args.is_some()
}

#[derive(Debug, Clone, PartialEq)]
pub struct __hello_world_rpc_args__;

impl IntoVMBuffers for __hello_world_rpc_args__ {
    fn read_from_buffers(_: &mut BytesReader) -> Self {
        __hello_world_rpc_args__
    }

    fn write_to_buffers(&self, _: &mut BytesWriter) {}

    fn skip_in_buffers(_: &mut BytesReader, _: u64) {}
}

pub fn hello_world_rpc_handler(
    state: &mut vm::CycleState,
    client_buffer_address: vm::BufferAddress,
    server_buffer_address: vm::BufferAddress,
) -> bool {
    let args = vm::buffer_read(state, server_buffer_address, |bytes_reader| {
        bytes_reader.reset();
        let status = bytes_reader.read_byte();

        if status == 0xFF {
            Some(__hello_world_rpc_args__::read_from_buffers(bytes_reader))
        } else {
            None
        }
    });

    if let Some(args) = &args {
        vm::buffer_write(state, server_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0x00);
        });
        let ret = hello_world(
            state,
        );

        match ret {
            RpcResult::Data(ret) => {
                vm::buffer_write(state, client_buffer_address, |bytes_writer| {
                    bytes_writer.clear();
                    bytes_writer.write_byte(0xFF);
                    ret.write_to_buffers(bytes_writer);
                });
            }
            RpcResult::Skip => (),
        }
    }

    args.is_some()
}

#[derive(Debug, Clone, PartialEq)]
pub struct __say_hello_rpc_args__ {
    pub name: String,
}

impl IntoVMBuffers for __say_hello_rpc_args__ {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            name: String::read_from_buffers(bytes_reader),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        self.name.write_to_buffers(bytes_writer);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            String::read_from_buffers(bytes_reader);
        }
    }
}

pub fn say_hello_rpc_handler(
    state: &mut vm::CycleState,
    client_buffer_address: vm::BufferAddress,
    server_buffer_address: vm::BufferAddress,
) -> bool {
    let args = vm::buffer_read(state, server_buffer_address, |bytes_reader| {
        bytes_reader.reset();
        let status = bytes_reader.read_byte();

        if status == 0xFF {
            Some(__say_hello_rpc_args__::read_from_buffers(bytes_reader))
        } else {
            None
        }
    });

    if let Some(args) = &args {
        vm::buffer_write(state, server_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0x00);
        });
        let ret = say_hello(
            state,
            args.clone().name,
        );

        match ret {
            RpcResult::Data(ret) => {
                vm::buffer_write(state, client_buffer_address, |bytes_writer| {
                    bytes_writer.clear();
                    bytes_writer.write_byte(0xFF);
                    ret.write_to_buffers(bytes_writer);
                });
            }
            RpcResult::Skip => (),
        }
    }

    args.is_some()
}

#[derive(Debug, Clone, PartialEq)]
pub struct __sum_rpc_args__ {
    pub a: i32,
    pub b: f32,
    pub c: f64,
}

impl IntoVMBuffers for __sum_rpc_args__ {
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
    state: &mut vm::CycleState,
    client_buffer_address: vm::BufferAddress,
    server_buffer_address: vm::BufferAddress,
) -> bool {
    let args = vm::buffer_read(state, server_buffer_address, |bytes_reader| {
        bytes_reader.reset();
        let status = bytes_reader.read_byte();

        if status == 0xFF {
            Some(__sum_rpc_args__::read_from_buffers(bytes_reader))
        } else {
            None
        }
    });

    if let Some(args) = &args {
        vm::buffer_write(state, server_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0x00);
        });
        sum(
            state,
            args.clone().a,
            args.clone().b,
            args.clone().c,
        );
    }

    args.is_some()
}
