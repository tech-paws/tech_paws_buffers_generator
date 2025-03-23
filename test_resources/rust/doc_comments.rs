// GENERATED, DO NOT EDIT

//! Top level doc comment
//! Some description
//! Here are my constants
//! Addresses

#![allow(warnings)]
#![allow(clippy)]
#![allow(unknown_lints)]

use tech_paws_buffers::memory::{BytesReader, BytesWriter, BuffersModel};
use tech_paws_buffers::runtime_memory::{
    RpcMethodAddress, RpcRuntimeMemory, RpcMethodBuffer,
    RpcMethodPayloadSize, BuffersScopeId,
};
use tech_paws_buffers::{RpcMethodHandler, RpcRuntime, RpcMethod, SignalRpcResult};
use uuid::uuid;

/// Renderer
/// id = 1
pub mod addr {
    /// Some value
    /// String value
    pub const SOME_VALUE: &'static str = "Hello World!";

    /// Renderer
    /// id = 1
    pub mod groups {
        /// Main group
        pub const MAIN: u8 = 0;

        /// Renderer
        /// id = 1
        pub const RENDER: MyCommand = MyCommand(1);
    }
}

/// Some doc comment
/// Another doc comment
#[derive(Debug, Clone, PartialEq)]
pub enum MyEnum {
    Idle,
    Move {
        /// This is x field
        x: f64,

        /// This is y field
        y: f64,
    },
    Update(
        /// This is first option
        f64,

        /// This is second option
        f64,

        /// This is third option
        String,
    ),
}

impl Default for MyEnum {
    fn default() -> Self {
        Self::Idle
    }
}

/// Hello World!
/// This is View Data, Important Structure!
#[derive(Debug, Clone, PartialEq)]
pub struct ViewData {
    /// Delta time is delta time
    pub delta_time: f32,

    /// View Width
    pub view_width: f32,

    /// View Height!
    pub view_height: f32,

    /// Touch Start X
    /// It is starting position
    pub touch_start_x: f32,

    /// Touch Start Y
    /// It is starting position
    pub touch_start_y: f32,
}

impl Default for ViewData {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            view_width: 0.0,
            view_height: 0.0,
            touch_start_x: 0.0,
            touch_start_y: 0.0,
        }
    }
}

/// This is a test rpc
/// Use it only for tests
pub trait TestRpc {
    /// Say hello returns hello [name]! string.
    ///
    /// # Panic
    ///
    /// Don't worry, this function doesn't panic!!
    fn say_hello(
        name: String,
    ) -> String;

    /// Get up to date view data frame.
    fn view_data() -> SignalRpcResult<ViewData>;
}

impl BuffersModel for MyEnum {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        let value = bytes_reader.read_u32();

        match value {
            1 => return MyEnum::Idle,
            2 => return MyEnum::Move {
                x: bytes_reader.read_f64(),
                y: bytes_reader.read_f64(),
            },
            3 => return MyEnum::Update(
                bytes_reader.read_f64(),
                bytes_reader.read_f64(),
                String::read_from_buffers(bytes_reader),
            ),
            _ => panic!("Unsupported enum value: {}", value),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        match self {
            MyEnum::Idle => {
                bytes_writer.write_u32(1);
            },
            MyEnum::Move {
                x,
                y,
            } => {
                bytes_writer.write_u32(2);
                bytes_writer.write_f64(*x);
                bytes_writer.write_f64(*y);
            },
            MyEnum::Update(
                v0,
                v1,
                v2,
            ) => {
                bytes_writer.write_u32(3);
                bytes_writer.write_f64(*v0);
                bytes_writer.write_f64(*v1);
                v2.write_to_buffers(bytes_writer);
            },
        }
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            let value = bytes_reader.read_u32();

            match value {
                1 => (),
                2 => {
                    bytes_reader.read_f64();
                    bytes_reader.read_f64();
                },
                3 => {
                    bytes_reader.read_f64();
                    bytes_reader.read_f64();
                    String::read_from_buffers(bytes_reader);
                },
                _ => panic!("Unsupported enum value: {}", value),
            }
        }
    }
}

impl BuffersModel for ViewData {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            delta_time: bytes_reader.read_f32(),
            view_width: bytes_reader.read_f32(),
            view_height: bytes_reader.read_f32(),
            touch_start_x: bytes_reader.read_f32(),
            touch_start_y: bytes_reader.read_f32(),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_f32(self.delta_time);
        bytes_writer.write_f32(self.view_width);
        bytes_writer.write_f32(self.view_height);
        bytes_writer.write_f32(self.touch_start_x);
        bytes_writer.write_f32(self.touch_start_y);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
        }
    }
}

pub fn register_test_rpc<R: TestRpc>(runtime: &mut RpcRuntime) {
    let scope_id = BuffersScopeId(uuid!("11111111-1111-1111-1111-111111111111"));
    runtime.memory.add_scope(scope_id);
    runtime.register_rpc_method(
        RpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(0),
            handler: test_rpc_say_hello_rpc_handler::<R>,
        },
        RpcMethodPayloadSize::Medium,
    );
    runtime.register_signal_rpc_method(
        RpcMethod {
            scope_id,
            rpc_method_address: RpcMethodAddress(1),
            handler: test_rpc_view_data_rpc_handler::<R>,
        },
        RpcMethodPayloadSize::Medium,
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct __say_hello_rpc_args__ {
    pub name: String,
}

impl BuffersModel for __say_hello_rpc_args__ {
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

pub fn test_rpc_say_hello_rpc_handler<R: TestRpc>(
    scope_id: BuffersScopeId,
    memory: &mut RpcRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let args = memory.get_scope_mut(scope_id).rpc_buffer_read(
        rpc_method_address,
        RpcMethodBuffer::Server,
        |bytes_reader| __say_hello_rpc_args__::read_from_buffers(bytes_reader),
    );

    let result = R::say_hello(
        args.name,
    );

    memory.get_scope_mut(scope_id).rpc_buffer_write(
        rpc_method_address,
        RpcMethodBuffer::Client,
        |bytes_writer| {
            result.write_to_buffers(bytes_writer);
        },
    );
}

pub fn test_rpc_view_data_rpc_handler<R: TestRpc>(
    scope_id: BuffersScopeId,
    memory: &mut RpcRuntimeMemory,
    rpc_method_address: RpcMethodAddress,
) {
    let result = R::view_data();

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
