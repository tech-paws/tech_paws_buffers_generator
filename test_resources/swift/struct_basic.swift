struct ViewData: TechPawsBuffersModel {
    let deltaTime: Float
    let viewWidth: Float
    let viewHeight: Float
    let touchStartX: Float
    let touchStartY: Float
    let lastTouchX: Float
    let lastTouchY: Float
    let touchX: Float
    let touchY: Float

    static func createBuffersDefault() -> Self {
        return ViewData(
            deltaTime: 0,
            viewWidth: 0,
            viewHeight: 0,
            touchStartX: 0,
            touchStartY: 0,
            lastTouchX: 0,
            lastTouchY: 0,
            touchX: 0,
            touchY: 0
        )
    }

    static func readFromBuffers(
        _ bytesReader: TechPawsBuffersBytesReader
    ) -> Self {
        let deltaTime = bytesReader.readFloat()
        let viewWidth = bytesReader.readFloat()
        let viewHeight = bytesReader.readFloat()
        let touchStartX = bytesReader.readFloat()
        let touchStartY = bytesReader.readFloat()
        let lastTouchX = bytesReader.readFloat()
        let lastTouchY = bytesReader.readFloat()
        let touchX = bytesReader.readFloat()
        let touchY = bytesReader.readFloat()

        return ViewData(
            deltaTime: deltaTime,
            viewWidth: viewWidth,
            viewHeight: viewHeight,
            touchStartX: touchStartX,
            touchStartY: touchStartY,
            lastTouchX: lastTouchX,
            lastTouchY: lastTouchY,
            touchX: touchX,
            touchY: touchY
        )
    }

    static func skipInBuffers(
        _ bytesReader: TechPawsBuffersBytesReader,
        _ count: UInt64
    ) {
        for _ in 1...count {
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
            let _ = bytesReader.readFloat()
        }
    }

    func writeToBuffers(
        _ bytesWriter: TechPawsBuffersBytesWriter
    ) {
        bytesWriter.writeFloat(deltaTime)
        bytesWriter.writeFloat(viewWidth)
        bytesWriter.writeFloat(viewHeight)
        bytesWriter.writeFloat(touchStartX)
        bytesWriter.writeFloat(touchStartY)
        bytesWriter.writeFloat(lastTouchX)
        bytesWriter.writeFloat(lastTouchY)
        bytesWriter.writeFloat(touchX)
        bytesWriter.writeFloat(touchY)
    }
}
