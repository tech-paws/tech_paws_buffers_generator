#[derive(Debug, Clone, PartialEq)]
pub struct __say_hello_rpc_args__ {
    name: String,
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
        let ret = say_hello_rpc_handler_impl(
            state,
            args.clone().name,
        );
        vm::buffer_write(state, client_buffer_address, |bytes_writer| {
            bytes_writer.clear();
            bytes_writer.write_byte(0xFF);
            ret.write_to_buffers(bytes_writer);
        });
    }

    args.is_some()
}
