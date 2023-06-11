// GENERATED, DO NOT EDIT

#![allow(warnings)]
#![allow(clippy)]
#![allow(unknown_lints)]

use tech_paws_buffers::{BytesReader, BytesWriter, IntoVMBuffers};

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
    scope_id: TechPawsScopeId,
    memory: &mut TechPawsRuntimeMemory,
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

        print_hello_world();

        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
            },
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
    scope_id: TechPawsScopeId,
    memory: &mut TechPawsRuntimeMemory,
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

        let result = hello_world();

        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
                result.write_to_buffers(bytes_writer);
            },
        );
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
    scope_id: TechPawsScopeId,
    memory: &mut TechPawsRuntimeMemory,
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

        let result = say_hello(args.clone().name);

        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
                result.write_to_buffers(bytes_writer);
            },
        );
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
    scope_id: TechPawsScopeId,
    memory: &mut TechPawsRuntimeMemory,
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

        let result = sum(&mut emitter, args.clone().a, args.clone().b, args.clone().c);

        memory.get_scope_mut(scope_id).rpc_buffer_write(
            rpc_method_address,
            TechPawsRuntimeRpcMethodBuffer::Client,
            |bytes_writer| {
                bytes_writer.write_u8(0xFF);
                bytes_writer.write_f64(result);
            },
        );
    }
}
