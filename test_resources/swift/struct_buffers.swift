public struct EmptyBuffers {
    public static func readFromBuffers(bytesReader: BytesReader) -> Empty {
        return Empty();
    }

    public static func writeToBuffers(bytesWriter: BytesWriter, value: Empty) {
    }

    public static func skkipInBuffers(bytesReader: BytesReader, count: UInt64) {
    }
}

public struct ViewDataBuffers {
    static func readFromBuffers(bytesReader: BytesReader) -> ViewData {
        return ViewData(

        );
    }

    static func writeToBuffers(bytesWriter: BytesWriter, value: Empty) {
        bytesWriter.writeFloat(self.deltaTime);
        bytesWriter.writeFloat(self.viewWidth);
        bytesWriter.writeFloat(self.viewHeight);
        bytesWriter.writeFloat(self.touchStartX);
        bytesWriter.writeFloat(self.touchStartY);
        bytesWriter.writeFloat(self.lastTouchX);
        bytesWriter.writeFloat(self.lastTouchY);
        bytesWriter.writeFloat(self.touchX);
        bytesWriter.writeFloat(self.touchY);
    }

    static func skkipInBuffers(bytesReader: BytesReader, count: UInt64) {
        // for _ in 0..count {
        //     bytes_reader.read_f32();
        //     bytes_reader.read_f32();
        //     bytes_reader.read_f32();
        //     bytes_reader.read_f32();
        //     bytes_reader.read_f32();
        //     bytes_reader.read_f32();
        //     bytes_reader.read_f32();
        //     bytes_reader.read_f32();
        //     bytes_reader.read_f32();
        // }
    }
}
