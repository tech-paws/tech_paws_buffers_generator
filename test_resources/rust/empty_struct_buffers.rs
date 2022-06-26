impl IntoVMBuffers for Empty {
    fn read_from_buffers(_: &mut BytesReader) -> Self {
        Empty
    }

    fn write_to_buffers(&self, _: &mut BytesWriter) {}

    fn skip_in_buffers(_: &mut BytesReader, _: u64) {}
}
