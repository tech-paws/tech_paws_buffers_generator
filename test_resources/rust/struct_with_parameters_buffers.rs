impl IntoVMBuffers for Test {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            delta_time: bytes_reader.read_f32(),
            view_width: bytes_reader.read_f32(),
            view_height: bytes_reader.read_f32(),
            touch_start_x: bytes_reader.read_f32(),
            touch_start_y: bytes_reader.read_f32(),
            last_touch_x: bytes_reader.read_f32(),
            last_touch_y: bytes_reader.read_f32(),
            touch_x: bytes_reader.read_f32(),
            touch_y: bytes_reader.read_f32(),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_f32(self.delta_time);
        bytes_writer.write_f32(self.view_width);
        bytes_writer.write_f32(self.view_height);
        bytes_writer.write_f32(self.touch_start_x);
        bytes_writer.write_f32(self.touch_start_y);
        bytes_writer.write_f32(self.last_touch_x);
        bytes_writer.write_f32(self.last_touch_y);
        bytes_writer.write_f32(self.touch_x);
        bytes_writer.write_f32(self.touch_y);
    }

    fn skip(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
        }
    }
}
