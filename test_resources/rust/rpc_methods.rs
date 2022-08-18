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
    memory: &mut tech_paws_runtime::Memory,
    state_getter: fn() -> &'static mut tech_paws_runtime::State,
    cycle_address: tech_paws_runtime::CycleAddress,
    client_buffer_address: tech_paws_runtime::BufferAddress,
    server_buffer_address: tech_paws_runtime::BufferAddress,
) -> bool {
    let args = tech_paws_runtime::buffer_read(memory, server_buffer_address, |bytes_reader| {
        bytes_reader.reset();
        let status = bytes_reader.read_byte();

        if status == 0xFF {
            Some(__print_hello_world_rpc_args__::read_from_buffers(bytes_reader))
        } else {
            None
        }
    });

    if let Some(args) = &args {
        tech_paws_runtime::buffer_write(memory, server_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0x00);
        });

        unsafe {
            let args = args.clone();
            let state = state_getter();
            let cycle = state
                .cycles_states
                .get_by_id(cycle_address)
                .clone()
                .data_ptr()
                .as_mut()
                .unwrap();

            cycle.async_spawner.spawn(async move {
                let state = state_getter();
                let cycle = state
                    .cycles_states
                    .get_by_id(cycle_address)
                    .clone()
                    .data_ptr()
                    .as_mut()
                    .unwrap();

                print_hello_world(
                    &mut cycle.memory,
                ).await;
            });
        }
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
    memory: &mut tech_paws_runtime::Memory,
    state_getter: fn() -> &'static mut tech_paws_runtime::State,
    cycle_address: tech_paws_runtime::CycleAddress,
    client_buffer_address: tech_paws_runtime::BufferAddress,
    server_buffer_address: tech_paws_runtime::BufferAddress,
) -> bool {
    let args = tech_paws_runtime::buffer_read(memory, server_buffer_address, |bytes_reader| {
        bytes_reader.reset();
        let status = bytes_reader.read_byte();

        if status == 0xFF {
            Some(__hello_world_rpc_args__::read_from_buffers(bytes_reader))
        } else {
            None
        }
    });

    if let Some(args) = &args {
        tech_paws_runtime::buffer_write(memory, server_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0x00);
        });

        unsafe {
            let args = args.clone();
            let state = state_getter();
            let cycle = state
                .cycles_states
                .get_by_id(cycle_address)
                .clone()
                .data_ptr()
                .as_mut()
                .unwrap();

            cycle.async_spawner.spawn(async move {
                let state = state_getter();
                let cycle = state
                    .cycles_states
                    .get_by_id(cycle_address)
                    .clone()
                    .data_ptr()
                    .as_mut()
                    .unwrap();

                let mut emitter = Emitter::<String>::new(
                    &mut cycle.memory,
                    client_buffer_address,
                );

                hello_world(
                    &mut emitter,
                ).await;
            });
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
    memory: &mut tech_paws_runtime::Memory,
    state_getter: fn() -> &'static mut tech_paws_runtime::State,
    cycle_address: tech_paws_runtime::CycleAddress,
    client_buffer_address: tech_paws_runtime::BufferAddress,
    server_buffer_address: tech_paws_runtime::BufferAddress,
) -> bool {
    let args = tech_paws_runtime::buffer_read(memory, server_buffer_address, |bytes_reader| {
        bytes_reader.reset();
        let status = bytes_reader.read_byte();

        if status == 0xFF {
            Some(__say_hello_rpc_args__::read_from_buffers(bytes_reader))
        } else {
            None
        }
    });

    if let Some(args) = &args {
        tech_paws_runtime::buffer_write(memory, server_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0x00);
        });

        unsafe {
            let args = args.clone();
            let state = state_getter();
            let cycle = state
                .cycles_states
                .get_by_id(cycle_address)
                .clone()
                .data_ptr()
                .as_mut()
                .unwrap();

            cycle.async_spawner.spawn(async move {
                let state = state_getter();
                let cycle = state
                    .cycles_states
                    .get_by_id(cycle_address)
                    .clone()
                    .data_ptr()
                    .as_mut()
                    .unwrap();

                let mut emitter = Emitter::<String>::new(
                    &mut cycle.memory,
                    client_buffer_address,
                );

                say_hello(
                    &mut emitter,
                    args.clone().name,
                ).await;
            });
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
    memory: &mut tech_paws_runtime::Memory,
    state_getter: fn() -> &'static mut tech_paws_runtime::State,
    cycle_address: tech_paws_runtime::CycleAddress,
    client_buffer_address: tech_paws_runtime::BufferAddress,
    server_buffer_address: tech_paws_runtime::BufferAddress,
) -> bool {
    let args = tech_paws_runtime::buffer_read(memory, server_buffer_address, |bytes_reader| {
        bytes_reader.reset();
        let status = bytes_reader.read_byte();

        if status == 0xFF {
            Some(__sum_rpc_args__::read_from_buffers(bytes_reader))
        } else {
            None
        }
    });

    if let Some(args) = &args {
        tech_paws_runtime::buffer_write(memory, server_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0x00);
        });

        unsafe {
            let args = args.clone();
            let state = state_getter();
            let cycle = state
                .cycles_states
                .get_by_id(cycle_address)
                .clone()
                .data_ptr()
                .as_mut()
                .unwrap();

            cycle.async_spawner.spawn(async move {
                let state = state_getter();
                let cycle = state
                    .cycles_states
                    .get_by_id(cycle_address)
                    .clone()
                    .data_ptr()
                    .as_mut()
                    .unwrap();

                sum(
                    &mut cycle.memory,
                    args.clone().a,
                    args.clone().b,
                    args.clone().c,
                ).await;
            });
        }
    }

    args.is_some()
}
