#[derive(Debug, Clone, PartialEq)]
pub struct __sum_rpc_args__ {
    a: i32,
    b: f32,
    c: f64,
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
        sum_rpc_handler_impl(
            state,
            args.clone().a,
            args.clone().b,
            args.clone().c,
        );
    }

    args.is_some()
}
