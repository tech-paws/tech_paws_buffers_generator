impl IntoVMBuffers for Empty {
    fn read_from_buffers(_: &mut BytesReader) -> Self {
        Empty
    }

    fn write_to_buffers(&self, _: &mut BytesWriter) {}

    fn skip_in_buffers(_: &mut BytesReader, _: u64) {}
}

impl IntoVMBuffers for ViewData {
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

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
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

impl IntoVMBuffers for GenericType {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            items: Vec::<Test>::read_from_buffers(bytes_reader),
            table: LinearTable::<f32, Test>::read_from_buffers(bytes_reader),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        self.items.write_to_buffers(bytes_writer);
        self.table.write_to_buffers(bytes_writer);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            Vec::<Test>::read_from_buffers(bytes_reader);
            LinearTable::<f32, Test>::read_from_buffers(bytes_reader);
        }
    }
}
