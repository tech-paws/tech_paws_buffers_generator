impl IntoVMBuffers for Test {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            touch_x: bytes_reader.read_f32(),
            touch_y: bytes_reader.read_f32(),
            touch_status: TouchStatus::read_from_buffers(bytes_reader),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_f32(self.touch_x);
        bytes_writer.write_f32(self.touch_y);
        self.touch_status.write_to_buffers(bytes_writer);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            TouchStatus::read_from_buffers(bytes_reader);
        }
    }
}
