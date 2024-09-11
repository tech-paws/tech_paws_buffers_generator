struct Test: TechPawsBuffersModel {
    let touchY: Float
    let touchX: Float
    let touchStatus: TouchStatus

    static func createBuffersDefault() -> Self {
        return Test(
            touchY: 0,
            touchX: 0,
            touchStatus: TouchStatus.createBuffersDefault()
        )
    }

    static func readFromBuffers(
        _ bytesReader: TechPawsBuffersBytesReader
    ) -> Self {
        let touchY = bytesReader.readFloat()
        let touchX = bytesReader.readFloat()
        let touchStatus = TouchStatus.readFromBuffers(bytesReader)

        return Test(
            touchY: touchY,
            touchX: touchX,
            touchStatus: touchStatus
        )
    }

    static func skipInBuffers(
        _ bytesReader: TechPawsBuffersBytesReader,
        _ count: UInt64
    ) {
        for _ in 1...count {
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
            let _ = TouchStatus.readFromBuffers(bytesReader)
        }
    }

    func writeToBuffers(
        _ bytesWriter: TechPawsBuffersBytesWriter
    ) {
        bytesWriter.writeFloat(touchY)
        bytesWriter.writeFloat(touchX)
        touchStatus.writeToBuffers(bytesWriter)
    }
}
